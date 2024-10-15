use chrono::Local;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use rusqlite::Connection;
use polars::prelude::*;
use std::io::Error as IoError;

pub struct DatabaseError(rusqlite::Error);

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> Self {
        DatabaseError(err)
    }
}

impl From<DatabaseError> for std::io::Error {
    fn from(err: DatabaseError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, err.0.to_string())
    }
}

pub fn set_up_dir_structure() -> std::io::Result<PathBuf> {
    let now = Local::now();
    let dir_path = format!("Sim_{}/Results/Database", now.format("%Y-%m-%d_%H-%M-%S"));
    fs::create_dir_all(&dir_path)?;
    Ok(PathBuf::from(dir_path))
}

pub fn process_and_archive_results() -> std::io::Result<()> {
    let dir = "Jake";
    let db_files: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map(|ext| ext == "db").unwrap_or(false))
        .map(|entry| entry.path())
        .collect();

    if db_files.is_empty() {
        println!("No .db files found in {}", dir);
        return Ok(());
    }

    println!("Found {} .db files", db_files.len());

    let mut merged_data = Vec::new();
    for db_file in &db_files {
        let conn = Connection::open(db_file).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let query = r#"
            SELECT round, species, genome_size, exon_end_range,
                   non_coding_end_range, active_te, te_mobilized,
                   te_in_exons, te_mobilize_prob
            FROM simulation_results
            WHERE round = (SELECT MAX(round) FROM simulation_results)
        "#;
        let mut stmt = conn.prepare(query).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, i64>(7)?,
                row.get::<_, f64>(8)?,
            ))
        }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        for row in rows {
            merged_data.push(row.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?);
        }
    }

    let df = DataFrame::new(vec![
        Series::new("round".into(), merged_data.iter().map(|r| r.0).collect::<Vec<i64>>()),
        Series::new("species".into(), merged_data.iter().map(|r| r.1.clone()).collect::<Vec<String>>()),
        Series::new("genome_size".into(), merged_data.iter().map(|r| r.2).collect::<Vec<i64>>()),
        Series::new("exon_end_range".into(), merged_data.iter().map(|r| r.3).collect::<Vec<i64>>()),
        Series::new("non_coding_end_range".into(), merged_data.iter().map(|r| r.4).collect::<Vec<i64>>()),
        Series::new("active_te".into(), merged_data.iter().map(|r| r.5).collect::<Vec<i64>>()),
        Series::new("te_mobilized".into(), merged_data.iter().map(|r| r.6).collect::<Vec<i64>>()),
        Series::new("te_in_exons".into(), merged_data.iter().map(|r| r.7).collect::<Vec<i64>>()),
        Series::new("te_mobilize_prob".into(), merged_data.iter().map(|r| r.8).collect::<Vec<f64>>()),
    ]).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    println!("{:?}", df);

    // Create input Statistics from the simulation
    let species = df["species"].str()
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?
        .get(0)
        .ok_or_else(|| IoError::new(std::io::ErrorKind::NotFound, "Species not found"))?
        .to_string();

    let genome_size = df["genome_size"].i64()
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?
        .get(0)
        .ok_or_else(|| IoError::new(std::io::ErrorKind::NotFound, "Genome size not found"))?;

    let num_exons = df["exon_end_range"].i64().unwrap().get(0).unwrap();
    let num_simulation = df["round"].i64().unwrap().max().unwrap();
    let num_TEs = df["active_te"].i64().unwrap().get(0).unwrap();
    let te_mob_prob = df["te_mobilize_prob"].f64().unwrap().get(0).unwrap();

    // Create Summary Statistics from the simulation
    let min_mutation = df["te_in_exons"].i64().unwrap().min().unwrap();
    let max_mutation = df["te_in_exons"].i64().unwrap().max().unwrap();
    let mean_mutation = df["te_in_exons"].i64().unwrap().mean().unwrap();
    let median_mutation = df["te_in_exons"].i64().unwrap().median().unwrap();

    let _rounds: Vec<i64> = df["round"].i64()
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?
        .into_iter()
        .flatten()
        .collect();

    let _mutations: Vec<i64> = df["te_in_exons"].i64()
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?
        .into_iter()
        .flatten()
        .collect();

    // Create the results directory
    let results_dir = set_up_dir_structure()?;

    // Generate HTML report
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Simulation Results</title>
            <style>
                body {{ font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; }}
                h1, h2 {{ color: #333; }}
                table {{ border-collapse: collapse; width: 100%; }}
                th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
                th {{ background-color: #f2f2f2; }}
                img {{ max-width: 100%; height: auto; }}
            </style>
        </head>
        <body>
            <h1>Simulation Results</h1>
            
            <h2>Input Statistics</h2>
            <table>
                <tr><th>Statistic</th><th>Value</th></tr>
                <tr><td>Species</td><td>{}</td></tr>
                <tr><td>Genome Size</td><td>{}</td></tr>
                <tr><td>Number of Exons</td><td>{}</td></tr>
                <tr><td>Number of Simulations</td><td>{}</td></tr>
                <tr><td>Number of TEs</td><td>{}</td></tr>
                <tr><td>TE Mobilization Probability</td><td>{}</td></tr>
            </table>

            <h2>Summary Statistics</h2>
            <table>
                <tr><th>Statistic</th><th>Value</th></tr>
                <tr><td>Minimum Mutations</td><td>{}</td></tr>
                <tr><td>Maximum Mutations</td><td>{}</td></tr>
                <tr><td>Mean Mutations</td><td>{:.2}</td></tr>
                <tr><td>Median Mutations</td><td>{}</td></tr>
            </table>
        </body>
        </html>
        "#,
        species, genome_size, num_exons, num_simulation, num_TEs, te_mob_prob,
        min_mutation, max_mutation, mean_mutation, median_mutation
    );

    let html_path = results_dir.join("report.html");
    let mut html_file = File::create(html_path)?;
    html_file.write_all(html_content.as_bytes())?;

    println!("HTML report generated successfully!");

    Ok(())
}
