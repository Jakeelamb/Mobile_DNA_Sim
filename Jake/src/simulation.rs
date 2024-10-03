use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::path::Path;
use crate::data_process::{Record, convert_csv_to_list};
use rayon::prelude::*;  
use std::sync::atomic::{AtomicUsize, Ordering};
use rusqlite::{params, Connection, Result as SqliteResult};
use std::time::Instant;

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
            "Round {}: {} mutations",
            self.simulation_round,
            round_mutations
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

pub fn select_first_species() -> Result<Record, Box<dyn std::error::Error>> {
    let path = Path::new("Data/Species_data.csv");
    let records = convert_csv_to_list(path.to_str().unwrap())?;
    records.first().cloned().ok_or_else(|| "No species found in the CSV file".into())
}

pub fn create_db() -> SqliteResult<Connection> {
    let conn = Connection::open("simulation_results.db")?;

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

pub fn run_simulation(num_rounds: usize) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let selected_species = select_first_species()?;
    let mut param = SimulationParam::new(&selected_species);
    let conn = create_db()?;

    let mut total_mutations = 0;
    for _ in 0..num_rounds {
        total_mutations += param.run_simulation_round(&conn)?;
    }

    let duration = start_time.elapsed();
    println!("Simulation complete:");
    println!("  Number of rounds: {}", num_rounds);
    println!("  Total mutations: {}", total_mutations);
    println!("  Time taken: {:?}", duration);

    Ok(())
}
