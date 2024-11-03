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

/// Holds the state and information for the application
#[derive(Clone)]  
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
    
     //new run simulation idea
    pub simulation_start_triggered: bool,
    pub rounds_completed: usize,
    pub total_mutations: usize,
    pub simulation_rounds: usize, // Number of rounds to run when starting
     // Any others?
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
            simulation_rounds_completed: 0,
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
            start_time: None,
            run_time: "00:00:00".to_string(),
            current_round: 0,
            mutation_data: vec![
                (0.0, 10.0),
                (1.0, 20.0),
                (2.0, 30.0),
                (3.0, 40.0),
                (4.0, 50.0),
            ], // Example mutation data
            probability_data: vec![(0.0, 0.1), (1.0, 0.2), (2.0, 0.3), (3.0, 0.4), (4.0, 0.5)], // Example probability data

            // Search Bar
            search_mode: false,
            search_query: String::new(),
            filtered_species: vec![],

            // New run simulation idea
            simulation_start_triggered: false,
            rounds_completed: 0,
            total_mutations: 0,
            simulation_rounds: 1000,
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