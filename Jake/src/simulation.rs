use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::path::Path;
use data_process::{Record, convert_csv_to_list};
use std::collections::HashMap;
use std::sync::Mutex; // Ensure you have this import if using Mutex

//struct to define simualation parameters
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
    pub simulation_rounds: u64,
    pub te_mobilize_prob: f64,
}
// Store the results of each round in a thread-safe hashmap
lazy_static::lazy_static! {
    static ref RESULTS: Mutex<HashMap<String, YourResultType>> = Mutex::new(HashMap::new());
}

// Functions that pertain to the simulation struct
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
            simulation_rounds: 0, // Changed from simulation_round to simulation_rounds
            te_mobilize_prob: 0.5,
        }
    }
    // Function to run the simulation
    pub fn run_simulation_round(&mut self) {
        // Simulate TE mobilization
        self.simulation_rounds += 1; // Update this line
        let mobilized: f64 = self.active_te as f64 * self.te_mobilize_prob;
        self.te_mobilized += mobilized.round() as u64;

        // Simulate TE mobility
        let te_lengths = get_te_lengths(self.active_te);
        let mut rng = rand::thread_rng();
        for _ in te_lengths {
            let insertion_position: u64 = rng.gen_range(0..self.genome_size);
            if insertion_position > self.exon_end_range {
                self.te_in_noncoding += 1;
                println!("TE landed in {}, no mutation", insertion_position);
            } else {
                self.te_in_exons += 1;
                println!("TE landed in {}, Mutation!", insertion_position);
            }
        }
        self.store_results();
    }
    // Store the results of each round in a hashmap
    fn store_results(&self) {
        if let Ok(mut results) = RESULTS.lock() {
            results.insert(self.simulation_rounds, self.clone()); // Update this line
        }
    }
    // Getter methods
    pub fn get_species(&self) -> &str {
        &self.species
    }
    pub fn get_genome_size(&self) -> u64 {
        self.genome_size
    }

    // Add more getter methods as needed
}
    
pub fn get_te_lengths(num_active_te: u64) -> Vec<u64> {
    let between = Uniform::from(100..=10000);
    let mut rng = rand::thread_rng();
    let mut te_lengths = Vec::with_capacity(num_active_te as usize);
    for _ in 0..num_active_te {
        te_lengths.push(between.sample(&mut rng));
    }
    te_lengths
}

pub fn select_first_species() -> Result<Record, Box<dyn std::error::Error>> {
    let path = Path::new("Data/Species_data.csv");
    let records = convert_csv_to_list(path.to_str().unwrap())?;
    records.first().cloned().ok_or_else(|| "No species found in the CSV file".into())
}

pub fn run_simulation(num_rounds: u64) -> Result<Vec<SimulationParam>, Box<dyn std::error::Error>> {
    let selected_species = select_first_species()?;
    let mut param = SimulationParam::new(&selected_species);
    let mut simulation_history = Vec::with_capacity(num_rounds as usize);
    for _ in 0..num_rounds {
        param.run_simulation_round();
        simulation_history.push(param.clone());
    }
    Ok(simulation_history)
}