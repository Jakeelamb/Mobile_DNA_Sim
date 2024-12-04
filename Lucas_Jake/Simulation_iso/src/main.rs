use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};
use rand::Rng;
use std::path::Path;
use rayon::prelude::*;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::time::Instant;
use std::fs::create_dir_all;
use csv::Writer;  // Add this import

#[derive(Clone, Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "Species_name")]
    pub species: String,
    #[serde(rename = "Genome_size")]
    pub genome_size: u64,
    #[serde(rename = "Num_exons")]
    pub exons: u64,
    #[serde(rename = "Domain")]
    pub assembly: String,
    #[serde(rename = "Exon_to_genome_ratio")]
    pub exon_to_genome_ratio: f64,
}

#[derive(Clone, Debug)]
pub struct SimulationParam {
    pub species: String,
    pub genome_size: usize,
    pub exon_length: usize,
    pub noncoding_length: usize,
    pub active_te: usize,
    pub te_mobilized: usize,
    pub te_in_exons: usize,
    pub te_in_noncoding: usize,
    pub simulation_round: usize,
    pub te_mobilize_prob: f64,
    pub exon_bp_deletion_prob: f64,
    pub noncoding_bp_deletion_prob: f64,
    pub exon_window_deletion_prob: f64,
    pub noncoding_window_deletion_prob: f64,
    pub window_size: usize,
    pub bp_deleted_exon: usize,
    pub bp_deleted_noncoding: usize,
}

impl SimulationParam {
    pub fn new(record: &Record) -> Self {
        SimulationParam {
            species: record.species.clone(),
            genome_size: record.genome_size as usize,
            exon_length: record.exons as usize,
            noncoding_length: (record.genome_size - record.exons) as usize,
            active_te: 10000,
            te_mobilized: 0,
            te_in_exons: 0,
            te_in_noncoding: 0,
            simulation_round: 0,
            te_mobilize_prob: 0.5,
            exon_bp_deletion_prob: 0.00001,
            noncoding_bp_deletion_prob: 0.001,
            exon_window_deletion_prob: 0.00001,
            noncoding_window_deletion_prob: 0.01,
            window_size: 1000,
            bp_deleted_exon: 0,
            bp_deleted_noncoding: 0,
        }
    }

    pub fn process_deletions(&mut self) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        
        // Calculate single base deletions for exons and noncoding separately
        let expected_exon_deletions = (self.exon_length as f64 * self.exon_bp_deletion_prob) as usize;
        let expected_noncoding_deletions = (self.noncoding_length as f64 * self.noncoding_bp_deletion_prob) as usize;

        // Calculate window deletions for each region
        let exon_windows = self.exon_length / self.window_size;
        let noncoding_windows = self.noncoding_length / self.window_size;
        
        let expected_exon_windows = (exon_windows as f64 * self.exon_window_deletion_prob) as usize;
        let expected_noncoding_windows = (noncoding_windows as f64 * self.noncoding_window_deletion_prob) as usize;
        
        let mut additional_exon_deletions = 0;
        let mut additional_noncoding_deletions = 0;
        
        // Process window deletions for exons
        for _ in 0..expected_exon_windows {
            let deletion_size = rng.gen_range(1..=self.window_size);
            additional_exon_deletions += deletion_size;
        }
        
        // Process window deletions for noncoding regions
        for _ in 0..expected_noncoding_windows {
            let deletion_size = rng.gen_range(1..=self.window_size);
            additional_noncoding_deletions += deletion_size;
        }

        // Ensure we don't delete more than what exists
        let total_exon_deletions = (expected_exon_deletions + additional_exon_deletions)
            .min(self.exon_length);
        let total_noncoding_deletions = (expected_noncoding_deletions + additional_noncoding_deletions)
            .min(self.noncoding_length);

        (total_exon_deletions, total_noncoding_deletions)
    }

    pub fn store_results(&self, conn: &Connection) -> SqliteResult<()> {
        conn.execute(
            "INSERT INTO simulation_results (
                round, species, genome_size, exon_length, noncoding_length,
                active_te, te_mobilized, te_in_exons, te_in_noncoding,
                te_mobilize_prob, exon_bp_deletion_prob, noncoding_bp_deletion_prob,
                exon_window_deletion_prob, noncoding_window_deletion_prob,
                bp_deleted_exon, bp_deleted_noncoding
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                self.simulation_round as i64,
                self.species,
                self.genome_size as i64,
                self.exon_length as i64,
                self.noncoding_length as i64,
                self.active_te as i64,
                self.te_mobilized as i64,
                self.te_in_exons as i64,
                self.te_in_noncoding as i64,
                self.te_mobilize_prob,
                self.exon_bp_deletion_prob,
                self.noncoding_bp_deletion_prob,
                self.exon_window_deletion_prob,
                self.noncoding_window_deletion_prob,
                self.bp_deleted_exon as i64,
                self.bp_deleted_noncoding as i64,
            ],
        )?;
        Ok(())
    }

    pub fn run_simulation_round(&mut self, conn: &Connection) -> SqliteResult<usize> {
        self.simulation_round += 1;
        let mobilized = (self.active_te as f64 * self.te_mobilize_prob) as usize;
        self.te_mobilized += mobilized;

        // Generate TE lengths
        let chunk_size = (mobilized / rayon::current_num_threads()).max(1);
        let te_lengths: Vec<usize> = (0..mobilized)
            .into_par_iter()
            .with_min_len(chunk_size)
            .map(|_| {
                let mut rng = rand::thread_rng();
                rng.gen_range(100..=10000)
            })
            .collect();

        let mut exon_insertions = 0;
        let mut noncoding_insertions = 0;
        let mut exon_growth = 0;
        let mut noncoding_growth = 0;

        // Process insertions in batches
        for length in te_lengths {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(self.exon_length as f64 / self.genome_size as f64) {
                exon_insertions += 1;
                exon_growth += length;
            } else {
                noncoding_insertions += 1;
                noncoding_growth += length;
            }
        }

        // Update TE counts
        self.te_in_exons += exon_insertions;
        self.te_in_noncoding += noncoding_insertions;

        // Process deletions
        let (exon_deletions, noncoding_deletions) = self.process_deletions();
        
        // Update genome metrics
        self.exon_length = self.exon_length + exon_growth - exon_deletions;
        self.noncoding_length = self.noncoding_length + noncoding_growth - noncoding_deletions;
        self.genome_size = self.exon_length + self.noncoding_length;
        self.bp_deleted_exon += exon_deletions;
        self.bp_deleted_noncoding += noncoding_deletions;

        // Store results periodically
        if self.simulation_round % 10 == 0 {
            self.store_results(conn)?;
        }
        
        // Print progress periodically
        if self.simulation_round % 100 == 0 {
            println!(
                "Round {}: TEs: {} mobilized, Exons: +{} -{}, Noncoding: +{} -{}",
                self.simulation_round,
                mobilized,
                exon_growth,
                exon_deletions,
                noncoding_growth,
                noncoding_deletions
            );
        }

        Ok(exon_insertions)
    }
}

fn create_db() -> SqliteResult<Connection> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let db_name = format!("simulation_results_{}.db", timestamp);
    let conn = Connection::open(&db_name)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS simulation_results (
            id INTEGER PRIMARY KEY,
            round INTEGER NOT NULL,
            species TEXT NOT NULL,
            genome_size INTEGER NOT NULL,
            exon_length INTEGER NOT NULL,
            noncoding_length INTEGER NOT NULL,
            active_te INTEGER NOT NULL,
            te_mobilized INTEGER NOT NULL,
            te_in_exons INTEGER NOT NULL,
            te_in_noncoding INTEGER NOT NULL,
            te_mobilize_prob REAL NOT NULL,
            exon_bp_deletion_prob REAL NOT NULL,
            noncoding_bp_deletion_prob REAL NOT NULL,
            exon_window_deletion_prob REAL NOT NULL,
            noncoding_window_deletion_prob REAL NOT NULL,
            bp_deleted_exon INTEGER NOT NULL,
            bp_deleted_noncoding INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn convert_db_to_csv(db_path: &Path, output_dir: &Path) -> Result<(), Box<dyn Error>> {
    // Create output directory if it doesn't exist
    create_dir_all(output_dir)?;

    // Connect to the database
    let conn = Connection::open(db_path)?;

    // Get list of all tables
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'"
    )?;
    
    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<SqliteResult<Vec<String>>>()?;

    // Process each table
    for table_name in tables {
        println!("Converting table: {}", table_name);
        
        // Get column names and data in a single statement
        let mut stmt = conn.prepare(&format!(
            "SELECT * FROM {}",
            table_name
        ))?;
        
        let column_names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(String::from)
            .collect();

        // Create CSV writer
        let output_path = output_dir.join(format!("{}.csv", table_name));
        let mut writer = Writer::from_path(output_path)?;

        // Write header
        writer.write_record(&column_names)?;

        // Write data rows
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let mut record = Vec::new();
            for i in 0..column_names.len() {
                let value: String = match row.get_ref(i)? {
                    rusqlite::types::ValueRef::Null => String::from(""),
                    rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                    rusqlite::types::ValueRef::Real(f) => f.to_string(),
                    rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                    rusqlite::types::ValueRef::Blob(b) => format!("{:?}", b),
                };
                record.push(value);
            }
            writer.write_record(&record)?;
        }
        writer.flush()?;
    }

    println!("Conversion completed successfully!");
    Ok(())
}

fn run_simulation(num_rounds: usize, species_name: Option<&str>, data_path: &str) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let all_species = read_species_data(data_path)?;
    
    let selected_species: Vec<Record> = match species_name {
        Some(name) => all_species.into_iter().filter(|s| s.species == name).collect(),
        None => all_species,
    };

    if selected_species.is_empty() {
        return Err("No species found matching the given name".into());
    }

    let total_mutations: usize = selected_species.par_iter().map(|species| {
        let conn = create_db().expect("Failed to create database connection");
        let mut param = SimulationParam::new(species);
        let mut species_mutations = 0;
        
        let exon_genome_ratio: f64 = (param.exon_length as f64 / species.genome_size as f64) * 100.0;
        
        for _ in 0..num_rounds {
            species_mutations += param.run_simulation_round(&conn).unwrap_or(0);
        }
        
        println!("Completed simulation for species: {}", species.species);
        println!("  Starting genome size: {}", species.genome_size);
        println!("  Starting number of exons: {}", param.exon_length);
        println!("  Exon to Genome ratio is ~: {:.2}%", exon_genome_ratio);
        println!("  Number of active TEs: {}", param.active_te);
        println!("  TE mobility probability: {}", param.te_mobilize_prob);
        println!("  Total base pairs deleted from exons: {}", param.bp_deleted_exon);
        println!("  Total base pairs deleted from noncoding: {}", param.bp_deleted_noncoding);
        
        species_mutations
    }).sum();

    let duration = start_time.elapsed();
    println!("Simulation complete:");
    println!("  Total mutations: {}", total_mutations);
    println!("  Time taken: {:?}", duration);

    Ok(())
}
fn read_species_data(path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let records: Result<Vec<Record>, _> = reader.deserialize().collect();
    records.map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let data_path = "/home/jake/Projects/Mobile_DNA_Sim/Jake/Data/grouped_simplified_Species_genomesize_exon.csv"; // Update this with your actual path
    let all_species = read_species_data(data_path)?;

    println!("Available species:");
    for (i, species) in all_species.iter().enumerate() {
        println!("{}. {}", i + 1, species.species);
    }
    
    println!("Enter a number to select a species, or press Enter to run for all species:");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "" => {
            println!("Running simulation for all species...");
            run_simulation(100, None, data_path)?;
        }
        input => {
            if let Ok(num) = input.parse::<usize>() {
                if num > 0 && num <= all_species.len() {
                    let selected_species = &all_species[num - 1].species;
                    println!("Running simulation for {}...", selected_species);
                    run_simulation(1000, Some(selected_species), data_path)?;
                } else {
                    println!("Invalid input. Running simulation for all species...");
                    run_simulation(100, None, data_path)?;
                }
            } else {
                println!("Invalid input. Running simulation for all species...");
                run_simulation(100, None, data_path)?;
            }
        }
    }
    
    Ok(())
}