// tabstate.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::home_renderer::render_home_widgets;
use crate::widgets::sim_renderer::render_sim_widgets;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListState;
use ratatui::widgets::{Block, Borders, Tabs};
use serde::Deserialize;
use serde_json::Value; // Import Value from serde_json for JSON parsing
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use csv::Writer;
use std::io::Error;

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
    pub tes_mobilized: usize,
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
}

#[derive(Deserialize, Debug)]
pub struct SpeciesRecord {
    pub species: String,
}

/// Load species names from a JSON file and return a Vec<String>
pub fn load_species_list(file_path: &str) -> Vec<String> {
    let data = fs::read_to_string(file_path).expect("Unable to read JSON file");
    let species_records: Vec<SpeciesRecord> = serde_json::from_str(&data).expect("Unable to parse JSON file");
    species_records.into_iter().map(|record| record.species).collect()
}

/// Load detailed species data from a JSON file
pub fn load_species_data() -> HashMap<String, SpeciesData> {
    let file_path = "data/species_all.json";
    let mut species_data = HashMap::new();

    if let Ok(data) = std::fs::read_to_string(file_path) {
        let species_records: Vec<SpeciesData> = serde_json::from_str(&data).expect("Unable to parse species data");

        for record in species_records {
            species_data.insert(record.species.clone(), record);
        }
    } else {
        println!("Failed to read species data file");
    }

    species_data
}

impl TabState {

    // Take 4
    pub fn export_to_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Construct the full file path
        let file_path = format!("{}/{}_results.csv", self.output_dir, self.current_species);
        
        // Create the CSV writer
        let mut wtr = csv::Writer::from_path(&file_path)?;
        
        // Write the headers and data
        wtr.write_record(&["Species", "Genome Size", "Exon Size", "Exon/Genome Ratio", "Number of Rounds", "Run Time"])?;
        wtr.write_record(&[
            &self.current_species,
            &self.species_info.get(&self.current_species).unwrap_or(&SpeciesData::default()).genome_size.to_string(),
            &self.species_info.get(&self.current_species).unwrap_or(&SpeciesData::default()).exon_size.to_string(),
            &self.species_info.get(&self.current_species).unwrap_or(&SpeciesData::default()).exon_ratio.to_string(),
            &self.current_round.to_string(),
            &self.run_time,
        ])?;
        
        wtr.flush()?; // Ensure all data is written to disk
        Ok(())
    }


    // Take 3
    // pub fn export_to_csv(&self) -> Result<(), Error> {
    //     // Construct the file path
    //     let directory_path = Path::new(&self.output_dir);
    //     let file_path = directory_path.join(format!("{}.csv", self.current_species));

    //     // Create the directory if it doesn't exist
    //     if !directory_path.exists() {
    //         fs::create_dir_all(directory_path)?;
    //     }

    //     // Create the CSV file
    //     let mut file = File::create(file_path)?;

    //     // Write CSV headers
    //     writeln!(file, "Species,Genome Size,Exon Size,Exon/Genome Ratio,Number of Rounds,Run Time")?;

    //     // Fetch species data
    //     if let Some(species_info) = self.species_info.get(&self.current_species) {
    //         // println!("Exporting data for species: {}", self.current_species); // Debug print

    //         // // Debug prints for values
    //         // println!("Genome Size: {}", species_info.genome_size);
    //         // println!("Exon Size: {}", species_info.exon_size);
    //         // println!("Exon/Genome Ratio: {}", species_info.exon_ratio);
    //         // println!("Current Round: {}", self.current_round);
    //         // println!("Run Time: {}", self.run_time);

    //         // Write data to CSV
    //         writeln!(
    //             file,
    //             "{},{},{},{},{},{}",
    //             self.current_species,
    //             species_info.genome_size,
    //             species_info.exon_size,
    //             species_info.exon_ratio,
    //             self.current_round,
    //             self.run_time
    //         )?;
    //     } else {
    //         println!("Warning: Species info not found for '{}'", self.current_species);
    //     }

    //     Ok(())
    // }

    // Take 2
    // pub fn export_to_csv(&self) -> Result<(), Error> {
    //     // Construct the file path
    //     let directory_path = Path::new(&self.output_dir);
    //     let file_path = directory_path.join(format!("{}.csv", self.current_species));

    //     // Create the directory if it doesn't exist
    //     if !directory_path.exists() {
    //         fs::create_dir_all(directory_path)?;
    //     }

    //     // Create the CSV file
    //     let mut file = File::create(file_path)?;

    //     // Write CSV headers
    //     writeln!(file, "Species,Genome Size,Exon Size,Exon/Genome Ratio,Number of Rounds,Run Time")?;

    //     // Write data
    //     if let Some(species_info) = self.species_info.get(&self.current_species) {
    //         writeln!(
    //             file,
    //             "{},{},{},{},{},{}",
    //             self.current_species,
    //             species_info.genome_size,
    //             species_info.exon_size,
    //             species_info.exon_ratio,
    //             self.current_round,
    //             self.run_time
    //         )?;
    //     } else {
    //         println!("Warning: Species info not found for '{}'", self.current_species);
    //     }

    //     Ok(())
    // }

    // Take 1
    // pub fn export_to_csv(&self) {
    //     // Use the output directory and the species name to create a file path
    //     let file_path = format!("{}/{}_simulation_data.csv", self.output_dir, self.current_species);

    //     // Create the file
    //     let path = Path::new(&file_path);
    //     let mut wtr = Writer::from_path(path).expect("Failed to create CSV file");

    //     // Write headers to the CSV file
    //     wtr.write_record(&["Species", "Genome Size", "Exon Size", "Exon/Genome Ratio", "Rounds Completed", "Run Time"])
    //         .expect("Failed to write CSV headers");

    //     // Get the current species data
    //     if let Some(species_data) = self.species_info.get(&self.current_species) {
    //         // Write the row with the species data and current simulation stats
    //         wtr.write_record(&[
    //             &species_data.species,
    //             &species_data.genome_size.to_string(),
    //             &species_data.exon_size.to_string(),
    //             &species_data.exon_ratio.to_string(),
    //             &self.simulation_rounds_completed.to_string(),
    //             &self.run_time,
    //         ])
    //         .expect("Failed to write CSV data row");
    //     }

    //     // Flush and close the writer
    //     wtr.flush().expect("Failed to flush CSV writer");
    //     println!("Data successfully exported to {}", file_path);
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
            sim_rounds: "10000".to_string(),
            cpu_cores: "16".to_string(),
            output_dir: "Results".to_string(),
            active_input: 0,
            mutation_data: vec![
                (0.0, 10.0),
                (1.0, 20.0),
                (2.0, 30.0),
                (3.0, 40.0),
            ], // Example mutation data
            probability_data: vec![(0.0, 0.1), (1.0, 0.2), (2.0, 0.3), (3.0, 0.4)], // Example probability data

            // Search Bar
            search_mode: false,
            search_query: String::new(),
            filtered_species: vec![],

            // New run simulation idea
            start_time: None,
            run_time: "00:00:00".to_string(),
            current_round: 0,
            simulation_rounds_completed: 0,
            simulation_start_triggered: false,
            rounds_completed: 0,
            total_mutations: 0,
            simulation_rounds: 30,
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
        self.start_time = Some(Instant::now());
        self.current_round = 0;
    }
    // pub fn start_simulation(&mut self) {
    //     self.start_time = Some(Instant::now());
    // }

    pub fn update_simulation(&mut self) {
        // Only increment if there are more rounds left
        if self.has_more_rounds() {
            self.current_round += 1;
        }
    
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            self.run_time = format!(
                "{:02}:{:02}:{:02}",
                elapsed.as_secs() / 3600,
                (elapsed.as_secs() % 3600) / 60,
                elapsed.as_secs() % 60
            );
        }
    }

    pub fn has_more_rounds(&self) -> bool {
        self.current_round < self.simulation_rounds
    }

}

// ------ SPECIES DATA STRUCT ------
/// Structure to hold species data details
#[derive(Deserialize, Debug, Clone, Default)]  // Add `Clone` and `Default`
pub struct SpeciesData {
    pub species: String,
    #[serde(rename = "genome Size")] // Handles the space in the key
    pub genome_size: f64,
    #[serde(rename = "exon size")]
    pub exon_size: f64,
    #[serde(rename = "exon/genome ratio")]
    pub exon_ratio: f64,
}