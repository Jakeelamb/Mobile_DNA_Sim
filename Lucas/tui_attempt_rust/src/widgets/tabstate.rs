// src/widgets/tabstate.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::sim_renderer::render_sim_widgets;
use crate::widgets::home_renderer::render_home_widgets;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
use ratatui::style::{Style, Color, Modifier};
use ratatui::text::{Span, Line, Text};
use ratatui::widgets::ListState;
use std::collections::HashMap;
use serde_json::Value; // Import Value from serde_json for JSON parsing
use serde::Deserialize;
use std::fs;
use crate::logo::{get_ascii_logo, get_ascii_sim}; // Import the logo functions

/// Holds the state and information for the application
pub struct TabState {
    pub index: usize,
    pub species: Vec<String>,       // List of species for the home tab
    pub list_state: ListState,      // State management for the species list
    pub species_info: HashMap<String, SpeciesData>, // Species data for the info block
    pub starting_tes: String,       // Store user input for Starting TEs
    pub sim_rounds: String,         // Store user input for Simulation Rounds
    pub cpu_cores: String,          // Store user input for CPU Cores
    pub output_dir: String,         // Store user input for Output Directory
    pub active_input: usize,        // Track which input field is active

    pub current_species: String,    // Current species for simulation
    pub start_genome_size: String,  // Start genome size for simulation
    pub start_exon_size: String,    // Start exon size for simulation
    pub exon_genome_ratio: String,
    pub simulation_rounds_completed: usize,
    pub tes_mobilized: usize,
    pub mutations: usize,
    pub current_genome_size: String,
    pub current_exon_genome_ratio: String,
    pub probability_of_te_mutation: f64,
}

#[derive(Deserialize, Debug)]
struct SpeciesRecord {
    species: String,
}

/// Load species names from a JSON file and return a Vec<String>
pub fn load_species_list(file_path: &str) -> Vec<String> {
    // Read the JSON file
    let data = fs::read_to_string(file_path).expect("Unable to read JSON file");
    // Parse the JSON data into a Vec<SpeciesRecord>
    let species_records: Vec<SpeciesRecord> = serde_json::from_str(&data).expect("Unable to parse JSON file");
    // Extract the species names into a Vec<String>
    species_records.into_iter().map(|record| record.species).collect()
}

impl TabState {
    pub fn new() -> TabState {
        let mut state = ListState::default();
        state.select(Some(0)); // Start with the first item selected

         // Load species list from JSON file
        let species = load_species_list("data/species.json");
        let species_info = TabState::load_species_data();

            TabState { index: 0, species, list_state: state, species_info,
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
                active_input: 0
        } 
        }

        pub fn load_species_data() -> HashMap<String, SpeciesData> {
            // Replace with path to your JSON file
            let file_path = "data/species_all.json"; 
            let mut species_data = HashMap::new();
            if let Ok(data) = std::fs::read_to_string(file_path) {
                let parsed: Value = serde_json::from_str(&data).unwrap();
                if let Some(species_map) = parsed.as_object() {
                    for (key, value) in species_map {
                        if let Some(details) = value.as_object() {
                            species_data.insert(
                                key.clone(),
                                SpeciesData {
                                    name: key.clone(),
                                    genome_size: details.get("genome_size").unwrap_or(&Value::String("N/A".to_string())).to_string(),
                                    exon_size: details.get("exon_size").unwrap_or(&Value::String("N/A".to_string())).to_string(),
                                    exon_ratio: details.get("exon_ratio").unwrap_or(&Value::String("N/A".to_string())).to_string(),
                                },
                            );
                        }
                    }
                }
            }
            species_data
        }


    pub fn render(&self) -> Tabs {
        let titles = ["Home", "Simulation"];
        let tab_titles: Vec<Line> = titles.iter().map(|t| Line::from(Span::raw(*t))).collect(); // Updated to use `Line`
        Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.index)
            .highlight_style(Style::default().fg(Color::Yellow))
    }

    pub fn render_content(&self) -> Vec<AppWidget> {
        match self.index {
            0 => render_home_widgets(self),  // Render widgets for the Home tab
            1 => render_sim_widgets(self),   // Render widgets for the Simulation tab (don't need to do self.render_sim_widgets() now)
            _ => render_home_widgets(self),
        }
    }

/// Helper function to render widgets for the Simulation tab
pub fn start_simulation(&mut self) {
    // Implement the simulation start logic here
    println!("yo!!!!!!!!!!");   
    // println!("Simulation started for species: {}", self.current_species);
    // Simulation logic goes here, e.g., modifying state, running the simulation, etc.
}
}
// ------ SPECIES DATA STRUCT ------

/// Structure to hold species data details
#[derive(Debug, Clone, Default)]
pub struct SpeciesData {
    pub name: String,
    pub genome_size: String,
    pub exon_size: String,
    pub exon_ratio: String,
}