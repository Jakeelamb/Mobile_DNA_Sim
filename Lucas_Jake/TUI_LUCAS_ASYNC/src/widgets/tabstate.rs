// tabstate.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::home_renderer::render_home_widgets;
use crate::widgets::sim_renderer::render_sim_widgets;
use csv::Writer;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListState;
use ratatui::widgets::{Block, Borders, Tabs};
use serde::Deserialize;
use serde_json::Value; // Import Value from serde_json for JSON parsing
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Instant;

/// Holds the state and information for the application
pub struct TabState {
    pub index: usize,
    pub species: Vec<String>,  // List of species for the home tab
    pub list_state: ListState, // State management for the species list
    pub species_info: HashMap<String, SpeciesData>, // Species data for the info block
    pub starting_tes: String,  // Store user input for Starting TEs
    pub sim_rounds: String,    // Store user input for Simulation Rounds
    pub cpu_cores: String,     // Store user input for CPU Cores
    pub output_dir: String,    // Store user input for Output Directory
    pub active_input: usize,   // Track which input field is active

    pub current_species: String,   // Current species for simulation
    pub start_genome_size: String, // Start genome size for simulation
    pub start_exon_size: String,   // Start exon size for simulation
    pub exon_genome_ratio: String,
    pub simulation_rounds_completed: usize,
    pub mutations: usize,
    pub current_genome_size: String,
    pub current_exon_genome_ratio: String,
    pub probability_of_te_mutation: f64,
    pub start_time: Option<Instant>,
    pub run_time: String,                  // Holds the formatted run time
    pub current_round: usize,              // Holds the current simulation round
    pub mutation_data: Vec<(f64, f64)>,    // Store mutation data (round, # of mutations)
    pub probability_data: Vec<(f64, f64)>, // Store mutation probability data

    //Search Bar
    pub search_mode: bool,
    pub search_query: String,
    pub filtered_species: Vec<String>,

    pub simulation_start_triggered: bool,
    pub rounds_completed: usize,
    pub total_mutations: usize,
    pub simulation_rounds: usize,

    // progress graph / spinner
    pub spinner_index: usize,
    pub progress_bar: String,

    // Round data
    pub round_data: Vec<RoundData>,

    // Simulation-specific fields
    pub num_active_te: usize,    // Number of active TEs in the simulation
    pub te_lengths: Vec<usize>,  // Lengths of the active TEs
    pub te_static: usize,        // Number of TEs that did not mobilize
    pub tes_mobilized: usize,    // Number of TEs that mobilized
    pub te_in_exons: usize,      // Number of TEs inserted into exons
    pub te_in_noncoding: usize,  // Number of TEs inserted into non-coding regions
    pub exon_start_range: usize, // Start exon range as usize
    pub exon_end_range: usize,   // End exon range as usize
    pub noncoding_start_range: usize, // Start non-coding range as usize
    // pub noncoding_start_range: String, // exon size + 1
    pub noncoding_end_range: usize, // genome size
    pub sim_mobility_prob: String,  // Store user input for TE Mobility Probability
}

#[derive(Deserialize, Debug)]
pub struct SpeciesRecord {
    pub species: String,
}

/// Load species names from a JSON file and return a Vec<String>
pub fn load_species_list(file_path: &str) -> Vec<String> {
    let data = fs::read_to_string(file_path).expect("Unable to read JSON file");
    let species_records: Vec<SpeciesRecord> =
        serde_json::from_str(&data).expect("Unable to parse JSON file");
    species_records
        .into_iter()
        .map(|record| record.species)
        .collect()
}

/// Load detailed species data from a JSON file
pub fn load_species_data() -> HashMap<String, SpeciesData> {
    let file_path = "data/species_all.json";
    let mut species_data = HashMap::new();

    if let Ok(data) = std::fs::read_to_string(file_path) {
        let species_records: Vec<SpeciesData> =
            serde_json::from_str(&data).expect("Unable to parse species data");

        for record in species_records {
            species_data.insert(record.species.clone(), record);
        }
    } else {
        println!("Failed to read species data file");
    }

    species_data
}

impl TabState {
    // Generate TE lengths based on a normal distribution
    pub fn generate_te_lengths(&self, num_active_te: usize) -> Vec<usize> {
        let mean_length = 5000.0;
        let std_dev = 2000.0;
        let normal = Normal::new(mean_length, std_dev).unwrap();
        let mut rng = rand::thread_rng();

        (0..num_active_te)
            .map(|_| {
                let length = (normal.sample(&mut rng) as f64).round() as i32;
                length.clamp(100, 10000) as usize
            })
            .collect()
    }

    // simulation logic:
    pub fn run_simulation_round(&mut self) {
        let genome_size = self.current_genome_size.parse::<usize>().unwrap();
        let te_mobilize_threshold: f64 = self.sim_mobility_prob.parse().unwrap();
        let mut rng = rand::thread_rng();

        for _ in 0..self.num_active_te {
            let te_length = *self.te_lengths.choose(&mut rng).unwrap();

            if rng.gen::<f64>() < te_mobilize_threshold {
                let te_position = rng.gen_range(0..genome_size);

                if te_position <= self.exon_end_range {
                    self.exon_end_range += te_length;
                    self.noncoding_start_range = self.exon_end_range + 1;
                    self.noncoding_end_range += te_length;
                    self.te_in_exons += 1;
                } else {
                    self.noncoding_end_range += te_length;
                    self.te_in_noncoding += 1;
                }

                self.tes_mobilized += 1;
                self.current_genome_size = (genome_size + te_length).to_string();
            } else {
                self.te_static += 1;
            }
        }

        // Update the exon/genome ratio
        let genome_size_float = self.current_genome_size.parse::<f64>().unwrap_or(1.0); // Ensure no division by zero
        self.current_exon_genome_ratio =
            (self.exon_end_range as f64 / genome_size_float).to_string();

        // Log data for the current round
        self.round_data.push(RoundData {
            round: self.current_round,
            genome_size: genome_size as f64,
            exon_start_range: self.exon_start_range,
            exon_end_range: self.exon_end_range,
            noncoding_start_range: self.noncoding_start_range,
            noncoding_end_range: self.noncoding_end_range,
            active_te: self.num_active_te,
            te_mobilized: self.tes_mobilized,
            te_static: self.te_static,
            te_in_exons: self.te_in_exons,
            te_in_noncoding: self.te_in_noncoding,
            te_mobilize_prob: te_mobilize_threshold,
        });
    }
    // pub fn run_simulation_round(&mut self) {
    //     let genome_size = self.current_genome_size.parse::<usize>().unwrap();
    //     let te_mobilize_threshold: f64 = self.sim_mobility_prob.parse().unwrap();
    //     let mut rng = rand::thread_rng();

    //     for _ in 0..self.num_active_te {
    //         let te_length = *self.te_lengths.choose(&mut rng).unwrap();

    //         if rng.gen::<f64>() < te_mobilize_threshold {
    //             let te_position = rng.gen_range(0..genome_size);

    //             if te_position <= self.exon_end_range {
    //                 self.exon_end_range += te_length;
    //                 self.noncoding_start_range = self.exon_end_range + 1;
    //                 self.noncoding_end_range += te_length;
    //                 self.te_in_exons += 1;
    //             } else {
    //                 self.noncoding_end_range += te_length;
    //                 self.te_in_noncoding += 1;
    //             }

    //             self.tes_mobilized += 1;
    //             self.current_genome_size = (genome_size + te_length).to_string();
    //         } else {
    //             self.te_static += 1;
    //         }
    //     }
    // }

    pub fn export_to_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}_results.csv", self.output_dir, self.current_species);
        let mut wtr = csv::Writer::from_path(&file_path)?;

        // Write headers
        wtr.write_record(&[
            "Round",
            "Species",
            "Genome Size",
            "Exon Start Range",
            "Exon End Range",
            "Non-Coding Start Range",
            "Non-Coding End Range",
            "Active TEs",
            "TEs Mobilized",
            "TEs Static",
            "TEs in Exons",
            "TEs in Non-Coding",
            "TE Mobilize Probability",
        ])?;

        // Write data for each round
        for round_data in &self.round_data {
            wtr.write_record(&[
                round_data.round.to_string(),
                self.current_species.clone(),
                round_data.genome_size.to_string(),
                round_data.exon_start_range.to_string(),
                round_data.exon_end_range.to_string(),
                round_data.noncoding_start_range.to_string(),
                round_data.noncoding_end_range.to_string(),
                round_data.active_te.to_string(),
                round_data.te_mobilized.to_string(),
                round_data.te_static.to_string(),
                round_data.te_in_exons.to_string(),
                round_data.te_in_noncoding.to_string(),
                round_data.te_mobilize_prob.to_string(),
            ])?;
        }

        wtr.flush()?; // Ensure all data is written to disk
        Ok(())
    }

    // Take 5
    // pub fn export_to_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     let file_path = format!("{}/{}_results.csv", self.output_dir, self.current_species);

    //     // Create the CSV writer
    //     let mut wtr = csv::Writer::from_path(&file_path)?;

    //     // Write the headers for all the required fields
    //     wtr.write_record(&[
    //         "Round",
    //         "Species",
    //         "Genome Size",
    //         "Exon Start Range",
    //         "Exon End Range",
    //         "Noncoding Start Range",
    //         "Noncoding End Range",
    //         "Active TE",
    //         "TE Mobilized",
    //         "TE Static",
    //         "TE in Exons",
    //         "TE in Noncoding",
    //         "TE Mobilize Probability",
    //     ])?;

    //     // Write the data for each round
    //     for round_data in &self.round_data {
    //         // Retrieve genome size and exon size from species info
    //         let species_info = self.species_info.get(&self.current_species).unwrap();

    //         // Parse `start_exon_size` as an integer, handle errors if necessary
    //         let exon_start_range = 0;
    //         let exon_end_range = self.start_exon_size.parse::<usize>().unwrap_or(0);
    //         let genome_size = species_info.genome_size;

    //         // Calculate the non-coding start range as `exon_start_range + 1`
    //         let non_coding_start = species_info.exon_size as usize + 1;

    //         // Write each row to the CSV file
    //         wtr.write_record(&[
    //             round_data.round.to_string(),
    //             self.current_species.clone(),
    //             genome_size.to_string(),
    //             exon_start_range.to_string(),
    //             species_info.exon_size.to_string(),
    //             non_coding_start.to_string(),
    //             genome_size.to_string(),
    //             round_data.active_te.to_string(),
    //             round_data.te_mobilized.to_string(),
    //             round_data.te_static.to_string(),
    //             round_data.te_in_exons.to_string(),
    //             round_data.te_in_noncoding.to_string(),
    //             self.sim_mobility_prob.clone(),
    //         ])?;
    //         // round_data.te_mobilize_prob.to_string(), --Leave in case add complexity to the algorithm.
    //     }

    //     wtr.flush()?; // Ensure all data is written to disk
    //     Ok(())
    // }

    pub fn new() -> TabState {
        let mut state = ListState::default();
        state.select(Some(0)); // Start with the first item selected

        // Load species list from JSON file
        let species = load_species_list("data/species.json");
        let species_info = load_species_data();

        TabState {
            index: 0,
            species,
            list_state: state,
            species_info,
            current_species: "T-rex".to_string(),
            start_genome_size: "3000".to_string(),
            start_exon_size: "1000".to_string(),
            exon_genome_ratio: "0.33".to_string(),
            tes_mobilized: 0,
            mutations: 0,
            current_genome_size: "3000".to_string(),
            current_exon_genome_ratio: "0.33".to_string(),
            probability_of_te_mutation: 0.01,

            starting_tes: "500".to_string(),
            // sim_rounds: "10000".to_string(),
            cpu_cores: "16".to_string(),
            sim_mobility_prob: "0.5".to_string(),
            output_dir: "Results".to_string(),
            active_input: 0,
            mutation_data: vec![(0.0, 10.0), (1.0, 20.0), (2.0, 30.0), (3.0, 40.0)], // Example mutation data
            probability_data: vec![(0.0, 0.1), (1.0, 0.2), (2.0, 0.3), (3.0, 0.4)], // Example probability data

            // Search Bar
            search_mode: false,
            search_query: String::new(),
            filtered_species: vec![],

            // Simulation control
            start_time: None,
            run_time: "00:00:00".to_string(),
            current_round: 0,
            simulation_rounds_completed: 0,
            simulation_start_triggered: false,
            rounds_completed: 0,
            total_mutations: 0,
            simulation_rounds: 30,
            sim_rounds: "30".to_string(),

            // progress graph / spinner
            spinner_index: 0,
            progress_bar: String::new(),

            // Round data
            round_data: vec![],

            //simulation specific fields
            num_active_te: 0,
            te_lengths: vec![],
            te_static: 0,
            te_in_exons: 0,
            te_in_noncoding: 0,
            exon_start_range: 0,
            exon_end_range: 0,
            noncoding_start_range: 0,
            noncoding_end_range: 0,
            // Ranges
            // exon_start_range: "0".to_string(),
            // non_coding_start_range: "50".to_string(),
        }
    }

    pub fn render(&self) -> Tabs {
        let titles = ["Home", "Simulation"];
        let tab_titles: Vec<Line> = titles.iter().map(|t| Line::from(Span::raw(*t))).collect();
        Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.index)
            .highlight_style(Style::default().fg(Color::Yellow))
    }

    pub fn render_content(&mut self) -> Vec<AppWidget> {
        match self.index {
            0 => render_home_widgets(self), // Render widgets for the Home tab
            _ => render_home_widgets(self),
        }
    }

    // Set the start time to now when the simulation starts
    pub fn start_simulation(&mut self) {
        // Initialize simulation parameters based on current species
        if let Some(species_info) = self.species_info.get(&self.current_species) {
            self.current_genome_size = species_info.genome_size.to_string();
            self.exon_start_range = 0;
            self.exon_end_range = species_info.exon_size as usize; // Use the initial exon size as the end of the exon range
            self.noncoding_start_range = self.exon_end_range + 1;
            self.noncoding_end_range = species_info.genome_size as usize;
            self.tes_mobilized = 0;
            self.te_static = 0;
            self.te_in_exons = 0;
            self.te_in_noncoding = 0;

            // Parse the number of active TEs and simulation rounds from user inputs
            self.num_active_te = self.starting_tes.parse::<usize>().unwrap_or(0);
            self.simulation_rounds = self.sim_rounds.parse::<usize>().unwrap_or(30);
            self.sim_mobility_prob = self.sim_mobility_prob.clone();

            // Generate TE lengths with a normal distribution
            self.te_lengths = self.generate_te_lengths(self.num_active_te);

            // Set up round-tracking
            self.current_round = 0;
            self.round_data.clear();
            self.simulation_start_triggered = true;
            self.start_time = Some(Instant::now());

            // Initialize progress bar and button
            self.progress_bar = "[                    ] 0%".to_string();
        }
    }
    // pub fn start_simulation(&mut self) {
    //     self.start_time = Some(Instant::now());
    //     self.current_round = 0;
    // }

    pub fn update_simulation(&mut self) {
        if self.has_more_rounds() {
            self.run_simulation_round(); // Execute one round of the simulation
            self.current_round += 1;

            // Update progress bar based on current round
            let progress = (self.current_round as f64 / self.simulation_rounds as f64).min(1.0);
            let bar_length = (progress * 20.0).round() as usize;
            self.progress_bar = format!(
                "[{}{}] {:.0}%",
                "=".repeat(bar_length),
                " ".repeat(20 - bar_length),
                progress * 100.0
            );

            // Update runtime
            if let Some(start_time) = self.start_time {
                let elapsed = start_time.elapsed();
                self.run_time = format!(
                    "{:02}:{:02}:{:02}",
                    elapsed.as_secs() / 3600,
                    (elapsed.as_secs() % 3600) / 60,
                    elapsed.as_secs() % 60
                );
            }
        } else {
            self.simulation_start_triggered = false;
        }
    }
    // pub fn update_simulation(&mut self) {
    //     if self.has_more_rounds() {
    //         self.current_round += 1;

    //         // Example values for the new round; these would be calculated by your algorithm
    //         let new_round_data = RoundData {
    //             round: self.current_round,
    //             genome_size: self.current_genome_size.parse().unwrap_or(3000.0),
    //             exon_start_range: 0,         // Replace with actual calculation
    //             exon_end_range: 1000,        // Replace with actual calculation
    //             noncoding_start_range: 1001, // Replace with actual calculation
    //             noncoding_end_range: 3000,   // Replace with actual calculation
    //             active_te: 500,              // Replace with actual value
    //             te_mobilized: 20,            // Replace with actual mobilization count
    //             te_static: 480,              // Replace with actual static count
    //             te_in_exons: 10,             // Replace with count of TEs in exons
    //             te_in_noncoding: 10,         // Replace with count of TEs in non-coding regions
    //             te_mobilize_prob: self.probability_of_te_mutation,
    //         };

    //         // Add round data to the collection
    //         self.round_data.push(new_round_data);

    //         // Update progress bar based on current round
    //         let progress = (self.current_round as f64 / self.simulation_rounds as f64).min(1.0);
    //         let bar_length = (progress * 20.0).round() as usize; // Adjust length as desired
    //         self.progress_bar = format!(
    //             "[{}{}] {:.0}%",
    //             "=".repeat(bar_length),      // Filled part
    //             " ".repeat(20 - bar_length), // Empty part
    //             progress * 100.0
    //         );
    //         // Update runtime
    //         if let Some(start_time) = self.start_time {
    //             let elapsed = start_time.elapsed();
    //             self.run_time = format!(
    //                 "{:02}:{:02}:{:02}",
    //                 elapsed.as_secs() / 3600,
    //                 (elapsed.as_secs() % 3600) / 60,
    //                 elapsed.as_secs() % 60
    //             );
    //         }
    //     }
    // }

    pub fn has_more_rounds(&self) -> bool {
        self.current_round < self.simulation_rounds
    }
}

// ------ SPECIES DATA STRUCT ------
/// Structure to hold species data details
#[derive(Deserialize, Debug, Clone, Default)] // Add `Clone` and `Default`
pub struct SpeciesData {
    pub species: String,
    #[serde(rename = "genome Size")] // Handles the space in the key
    pub genome_size: f64,
    #[serde(rename = "exon size")]
    pub exon_size: f64,
    #[serde(rename = "exon/genome ratio")]
    pub exon_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct RoundData {
    pub round: usize,
    pub genome_size: f64,
    pub exon_start_range: usize,
    pub exon_end_range: usize,
    pub noncoding_start_range: usize,
    pub noncoding_end_range: usize,
    pub active_te: usize,
    pub te_mobilized: usize,
    pub te_static: usize,
    pub te_in_exons: usize,
    pub te_in_noncoding: usize,
    pub te_mobilize_prob: f64,
}
