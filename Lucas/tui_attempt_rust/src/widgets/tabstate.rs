// src/widgets/tabstate.rs
use crate::widgets::app_widgets::AppWidget;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
use ratatui::style::{Style, Color};
use ratatui::text::{Span, Line};
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
    // pub starting_tes: String,       // Store user input for Starting TEs
    // pub sim_rounds: String,         // Store user input for Simulation Rounds
    // pub cpu_cores: String,          // Store user input for CPU Cores
    // pub output_dir: String,         // Store user input for Output Directory
    // pub active_input: usize,        // Track which input field is active
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
                // starting_tes: "500".to_string(),
                // sim_rounds: "10000".to_string(),
                // cpu_cores: "16".to_string(),
                // output_dir: "Results".to_string(),
                // active_input: 0 
        } 
        }

        pub fn load_species_data() -> HashMap<String, SpeciesData> {
            // Replace with path to your JSON file
            let file_path = "species_data.json"; 
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
            0 => self.render_home_widgets(),  // Render widgets for the Home tab
            1 => self.render_sim_widgets(),   // Render widgets for the Simulation tab
            _ => self.render_home_widgets(),
        }
    }

    pub fn render_header(&self) -> Paragraph {
        // Choose header text and colors based on the active tab index
        let (header_text, title_color, text_color, bg_color) = match self.index {
            0 => (
                get_ascii_logo(),         // Text for Home tab
                Color::Green,             // Block title color for Home tab
                Color::Blue,              // Text color for Home tab
                Color::Black,             // Background color for Home tab
            ),
            1 => (
                get_ascii_sim(),          // Text for Simulation tab
                Color::Yellow,            // Block title color for Simulation tab
                Color::Red,               // Text color for Simulation tab
                Color::Black,             // Background color for Simulation tab
            ),
            _ => (
                get_ascii_logo(),         // Default to Home logo
                Color::White,
                Color::Gray,
                Color::Black,
            ),
        };

        // Create the block using the dynamic colors
        Paragraph::new(header_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    // .title("Header Title")
                    .style(Style::default().bg(bg_color))
                    .border_style(Style::default().fg(title_color)), // Use title color
            )
            .style(Style::default().fg(text_color).bg(bg_color)) // Apply text color and background color
    }

     /// Helper function to render widgets for the Home tab
    pub fn render_home_widgets(&self) -> Vec<AppWidget> {
        // Create a List widget for the species
        let species_items: Vec<ListItem> = self
            .species
            .iter()
            .map(|species_name| ListItem::new(Span::from(species_name.clone()))) // Use `Span::from` for each species
            .collect();

        // Create a List widget with species items
        let species_list = List::new(species_items)
            .block(Block::default().borders(Borders::ALL).title("Species List"))
            .style(Style::default().fg(Color::Black).bg(Color::White))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow)); // Highlighted item style

         // Create Species Info block
        let selected_species = self.species.get(self.list_state.selected().unwrap_or(0)).unwrap();
        let species_info = self.species_info.get(selected_species).cloned().unwrap_or(SpeciesData::default());
        let species_info_text = format!(
            "Species: {}\nGenome Size: {}\nExon Size: {}\nExon/Genome Ratio: {}",
            species_info.name, species_info.genome_size, species_info.exon_size, species_info.exon_ratio
        );
        let info_block = Paragraph::new(species_info_text)
            .block(Block::default().borders(Borders::ALL).title("Species Info"))
            .style(Style::default().bg(Color::Blue).fg(Color::White));
 
         // Create Simulation Settings block
        let sim_settings_text = "Simulation Settings\nStarting TEs: 500\nSimulation Rounds: 10000\nCPU cores: 16\nOutput Directory Path: Results";
        let sim_settings_block = Paragraph::new(sim_settings_text)
            .block(Block::default().borders(Borders::ALL).title("Simulation Settings"))
            .style(Style::default().bg(Color::Green).fg(Color::White));
 
        vec![
            AppWidget::SpeciesList(species_list),
            AppWidget::InfoBlock(info_block),
            AppWidget::SettingsBlock(sim_settings_block),
        ]
    }

    /// Helper function to render widgets for the Simulation tab
    fn render_sim_widgets(&self) -> Vec<AppWidget> {
        let simulation_info = Paragraph::new("Simulation Info Block");
        let settings_block = Paragraph::new("Simulation Settings Block");

        vec![
            AppWidget::InfoBlock(simulation_info),
            AppWidget::SettingsBlock(settings_block),
        ]
    }

    /// Render a footer for the UI
    pub fn render_footer(&self) -> Paragraph {
        Paragraph::new("Created by: Jake & Lucas, Version: 1.0")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Black)
            .bg(Color::White))
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % 2;
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

        /// Move the selected item up in the list
        pub fn scroll_up(&mut self) {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.species.len() - 1 // Wrap around to the bottom
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    
        /// Move the selected item down in the list
        pub fn scroll_down(&mut self) {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= self.species.len() - 1 {
                        0 // Wrap around to the top
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
}

/// Structure to hold species data details
#[derive(Debug, Clone, Default)]
pub struct SpeciesData {
    pub name: String,
    pub genome_size: String,
    pub exon_size: String,
    pub exon_ratio: String,
}