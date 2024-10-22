use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::path::Path;
use crate::data_process::{Record, convert_csv_to_list};
use rayon::prelude::*;  
use std::sync::atomic::{AtomicUsize, Ordering};
use rusqlite::{params, Connection, Result as SqliteResult};
use std::time::Instant;
use std::error::Error;
use std::fs::create_dir_all;
use csv::Writer;

#[derive(Clone, Debug)]
pub struct SimulationParam {
    pub species: String,
    pub genome_size: usize,
    pub exon_start_range: usize,
    pub exon_end_range: usize,
    pub noncoding_start_range: usize,
    pub non_coding_end_range: usize,
    pub active_te: usize,
    pub te_mobilized: usize,
    pub te_static: usize,
    pub te_in_exons: usize,
    pub te_in_noncoding: usize,
    pub simulation_round: usize,
    pub te_mobilize_prob: f64,
}

impl SimulationParam {
    pub fn new(record: &Record) -> Self {
        SimulationParam {
            species: record.species.clone(),
            genome_size: record.genome_size as usize,
            exon_start_range: 0,
            exon_end_range: record.exons as usize,
            noncoding_start_range: (record.exons + 1) as usize,
            non_coding_end_range: record.genome_size as usize,
            active_te: 10000,
            te_mobilized: 0,
            te_static: 0,
            te_in_exons: 0,
            te_in_noncoding: 0,
            simulation_round: 0,
            te_mobilize_prob: 0.5,
        }
    }

    pub fn run_simulation_round(&mut self, conn: &Connection) -> SqliteResult<usize> {
        self.simulation_round += 1;
        let mobilized: usize = (self.active_te as f64 * self.te_mobilize_prob).trunc() as usize;
        self.te_mobilized += mobilized;

        let te_lengths = get_te_lengths(mobilized);
        let local_te_in_exons = AtomicUsize::new(0);
        let local_te_in_noncoding = AtomicUsize::new(0);

        te_lengths.par_iter().for_each(|_| {
            let mut rng = rand::thread_rng();
            let insertion_position: usize = rng.gen_range(0..self.genome_size);

            if insertion_position > self.exon_end_range {
                local_te_in_noncoding.fetch_add(1, Ordering::Relaxed);
            } else {
                local_te_in_exons.fetch_add(1, Ordering::Relaxed);
            }
        });

        let round_mutations = local_te_in_exons.load(Ordering::Relaxed);
        self.te_in_exons += round_mutations;
        self.te_in_noncoding += local_te_in_noncoding.load(Ordering::Relaxed);

        println!(
            "Round {}: {} mutations for {}",
            self.simulation_round,
            round_mutations,
            self.species
        );

        self.store_results(conn)?;
        Ok(round_mutations)
    }

    fn store_results(&self, conn: &Connection) -> SqliteResult<()> {
        conn.execute(
            "INSERT INTO simulation_results (
                round, species, genome_size, exon_start_range, exon_end_range,
                noncoding_start_range, non_coding_end_range, active_te, te_mobilized,
                te_static, te_in_exons, te_in_noncoding, te_mobilize_prob
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                self.simulation_round as i64,
                self.species,
                self.genome_size as i64,
                self.exon_start_range as i64,
                self.exon_end_range as i64,
                self.noncoding_start_range as i64,
                self.non_coding_end_range as i64,
                self.active_te as i64,
                self.te_mobilized as i64,
                self.te_static as i64,
                self.te_in_exons as i64,
                self.te_in_noncoding as i64,
                self.te_mobilize_prob
            ],
        )?;
        Ok(())
    }
}

pub fn get_te_lengths(num_active_te: usize) -> Vec<usize> {
    let between = Uniform::from(100..=10000);
    (0..num_active_te).into_par_iter().map(|_| {
        let mut rng = rand::thread_rng();
        between.sample(&mut rng)
    }).collect()
}

pub fn get_all_species() -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let path = Path::new("Data/grouped_simplified_Species_genomesize_exon.csv");
    convert_csv_to_list(path.to_str().unwrap())
}

pub fn select_species(species_list: &[Record], species_name: Option<&str>) -> Vec<Record> {
    match species_name {
        Some(name) => species_list.iter()
            .filter(|&s| s.species == name)
            .cloned()
            .collect(),
        None => species_list.to_vec(),
    }
}

pub fn create_db() -> SqliteResult<Connection> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let db_name = format!("simulation_results_{}.db", timestamp);
    let conn = Connection::open(db_name)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS simulation_results (
            id INTEGER PRIMARY KEY,
            round INTEGER NOT NULL,
            species TEXT NOT NULL,
            genome_size INTEGER NOT NULL,
            exon_start_range INTEGER NOT NULL,
            exon_end_range INTEGER NOT NULL,
            noncoding_start_range INTEGER NOT NULL,
            non_coding_end_range INTEGER NOT NULL,
            active_te INTEGER NOT NULL,
            te_mobilized INTEGER NOT NULL,
            te_static INTEGER NOT NULL,
            te_in_exons INTEGER NOT NULL,
            te_in_noncoding INTEGER NOT NULL,
            te_mobilize_prob REAL NOT NULL
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

pub fn run_simulation(num_rounds: usize, species_name: Option<&str>) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let all_species = get_all_species()?;
    let selected_species = select_species(&all_species, species_name);
    
    if selected_species.is_empty() {
        return Err("No species found matching the given name".into());
    }

    let total_mutations: usize = selected_species.par_iter().map(|species| {
        let conn = create_db().expect("Failed to create database connection");
        let mut param = SimulationParam::new(species);
        let mut species_mutations = 0;
        let exon_genome_ratio: f64 = (param.exon_end_range as f64 / species.genome_size as f64) * 100.0;
        
        for _ in 0..num_rounds {
            species_mutations += param.run_simulation_round(&conn).unwrap_or(0);
        }
        
        // Export results to CSV after simulation completes
        if let Some(db_path) = conn.path() {
            let output_dir = Path::new("simulation_results");
            let db_path = Path::new(db_path);  // Convert &str to &Path
            if let Err(e) = convert_db_to_csv(db_path, output_dir) {
                eprintln!("Failed to export results to CSV: {}", e);
            }
        }
        
        // Report additional information
        println!("Completed simulation for species: {}", species.species);
        println!("  Starting genome size: {}", species.genome_size);
        println!("  Starting number of exons: {}", param.exon_end_range);
        println!("  Exon to Genome ratio is ~: {:.2}%", exon_genome_ratio);
        println!("  Number of active TEs: {}", param.active_te);
        println!("  TE mobility probability: {}", param.te_mobilize_prob);
        println!("  Mean number of mutations per round: {:.2}", species_mutations as f64 / num_rounds as f64);

        species_mutations
    }).sum();

    let duration = start_time.elapsed();
    println!("Simulation complete:");
    println!("  Number of species: {}", selected_species.len());
    println!("  Number of rounds per species: {}", num_rounds);
    println!("  Total mutations across all rounds: {}", total_mutations);
    println!("  Time taken: {:?}", duration);

    Ok(())
}