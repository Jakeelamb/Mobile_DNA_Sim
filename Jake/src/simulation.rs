use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::path::Path;
use crate::data_process::{Record, convert_csv_to_list};
use multimap::MultiMap;
use std::fs::File;
use std::io::Write;
use rayon::prelude::*;  
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug)]
pub struct SimulationParam {
    pub species: String,
    pub genome_size: u64,
    pub exon_start_range: u64,
    pub exon_end_range: u64,
    pub noncoding_start_range: u64,
    pub non_coding_end_range: u64,
    pub active_te: u64,
    pub te_mobilized: u64,
    pub te_static: u64,
    pub te_in_exons: u64,
    pub te_in_noncoding: u64,
    pub simulation_round: u64,
    pub te_mobilize_prob: f64,
    results: MultiMap<u64, String>,
}

impl SimulationParam {
    pub fn new(record: &Record) -> Self {
        SimulationParam {
            species: record.species.clone(),
            genome_size: record.genome_size,
            exon_start_range: 0,
            exon_end_range: record.exons,
            noncoding_start_range: record.exons + 1,
            non_coding_end_range: record.genome_size,
            active_te: 10000,
            te_mobilized: 0,
            te_static: 0,
            te_in_exons: 0,
            te_in_noncoding: 0,
            simulation_round: 0,
            te_mobilize_prob: 0.5,
            results: MultiMap::new(),
        }
    }

    pub fn run_simulation_round(&mut self) {
        self.simulation_round += 1;
        let mobilized: f64 = (self.active_te as f64 * self.te_mobilize_prob).trunc();
        self.te_mobilized += mobilized as u64;
    
        let te_lengths = get_te_lengths(self.active_te);
    
        // Atomic counters to be updated safely across threads
        let local_te_in_exons = AtomicU64::new(0);
        let local_te_in_noncoding = AtomicU64::new(0);
    
        te_lengths.par_iter().for_each(|_| {
            let mut rng = rand::thread_rng();  // Create new RNG for each thread
            let insertion_position: u64 = rng.gen_range(0..self.genome_size);
            if insertion_position > self.exon_end_range {
                local_te_in_noncoding.fetch_add(1, Ordering::Relaxed);  // Atomically update
                // println!("TE landed in {}, no mutation", insertion_position);
            } else {
                local_te_in_exons.fetch_add(1, Ordering::Relaxed);  // Atomically update
                println!("TE landed in {}, Mutation!", insertion_position);
            }
        });
    
        // Aggregate the results back into the shared state
        self.te_in_exons += local_te_in_exons.load(Ordering::Relaxed);
        self.te_in_noncoding += local_te_in_noncoding.load(Ordering::Relaxed);
    
        self.store_results();
    }

    fn store_results(&mut self) {
        self.results.insert_many(self.simulation_round, vec![
            self.species.clone(),
            self.genome_size.to_string(),
            self.exon_start_range.to_string(),
            self.exon_end_range.to_string(),
            self.noncoding_start_range.to_string(),
            self.non_coding_end_range.to_string(),
            self.active_te.to_string(),
            self.te_mobilized.to_string(),
            self.te_static.to_string(),
            self.te_in_exons.to_string(),
            self.te_in_noncoding.to_string(),
            self.te_mobilize_prob.to_string(),
        ]);
    }

    pub fn write_results_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "Round,Species,GenomeSize,ExonStartRange,ExonEndRange,NoncodingStartRange,NoncodingEndRange,ActiveTE,TEMobilized,TEStatic,TEInExons,TEInNoncoding,TEMobilizeProb")?;
        
        for (round, values) in self.results.iter_all() {
            writeln!(file, "{},{}", round, values.join(","))?;
        }
        Ok(())
    }
}

pub fn get_te_lengths(num_active_te: u64) -> Vec<u64> {
    let between = Uniform::from(100..=10000);
    (0..num_active_te).into_par_iter().map(|_| {
        let mut rng = rand::thread_rng();  // New RNG for each thread
        between.sample(&mut rng)
    }).collect()}

pub fn select_first_species() -> Result<Record, Box<dyn std::error::Error>> {
    let path = Path::new("Data/Species_data.csv");
    let records = convert_csv_to_list(path.to_str().unwrap())?;
    records.first().cloned().ok_or_else(|| "No species found in the CSV file".into())
}

pub fn run_simulation(num_rounds: u64) -> Result<SimulationParam, Box<dyn std::error::Error>> {
    let selected_species = select_first_species()?;
    let mut param = SimulationParam::new(&selected_species);
    for _ in 0..num_rounds {
        param.run_simulation_round();
    }
    Ok(param)
}