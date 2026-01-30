use crate::widgets::simulation::SimulationParam;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, ListState, Tabs};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

/// Holds the state and information for the application
pub struct TabState {
    pub index: usize,
    pub species: Vec<String>,
    pub list_state: ListState,
    pub species_info: HashMap<String, SpeciesData>,
    pub starting_tes: String,
    pub sim_rounds: String,
    pub cpu_cores: String,
    pub output_dir: String,
    pub active_input: usize,
    pub current_species: String,
    pub tes_mobilized: usize,
    pub current_genome_size: String,
    pub start_time: Option<Instant>,
    pub run_time: String,
    pub current_round: usize,
    pub mutation_data: Vec<(f64, f64)>,
    pub probability_data: Vec<(f64, f64)>,
    pub simulation_param: Option<SimulationParam>,
    pub te_lengths_histogram: Vec<(usize, usize)>,
    pub deletion_stats: Vec<(usize, usize, usize)>,
    pub exon_ratio_history: Vec<(usize, f64)>,
    pub genome_size_history: Vec<(usize, usize)>,
    pub search_mode: bool,
    pub search_query: String,
    pub filtered_species: Vec<String>,
    pub simulation_start_triggered: bool,
    pub simulation_rounds: usize,
    pub progress_bar: String,
    pub round_data: Vec<RoundData>,
    pub te_in_exons: usize,
    pub te_in_noncoding: usize,
    pub sim_mobility_prob: String,
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
        state.select(Some(0));

        // Load species list from JSON file
        let species = load_species_list("data/species.json");
        let species_info = load_species_data();

        TabState {
            index: 0,
            species,
            list_state: state,
            species_info,
            current_species: String::new(),
            tes_mobilized: 0,
            current_genome_size: "0".to_string(),
            starting_tes: "500".to_string(),
            sim_rounds: "30".to_string(),
            cpu_cores: "16".to_string(),
            output_dir: "results".to_string(),
            active_input: 0,
            start_time: None,
            run_time: "00:00:00".to_string(),
            current_round: 0,
            mutation_data: Vec::new(),
            probability_data: Vec::new(),
            simulation_param: None,
            te_lengths_histogram: Vec::new(),
            deletion_stats: Vec::new(),
            exon_ratio_history: Vec::new(),
            genome_size_history: Vec::new(),
            search_mode: false,
            search_query: String::new(),
            filtered_species: Vec::new(),
            simulation_start_triggered: false,
            simulation_rounds: 30,
            progress_bar: String::new(),
            round_data: Vec::new(),
            te_in_exons: 0,
            te_in_noncoding: 0,
            sim_mobility_prob: "0.5".to_string(),
        }
    }

    pub fn render(&self) -> Tabs<'_> {
        let titles = ["Home", "Simulation"];
        let tab_titles: Vec<Line> = titles.iter().map(|t| Line::from(Span::raw(*t))).collect();
        Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.index)
            .highlight_style(Style::default().fg(Color::Yellow))
    }

    // Set the start time to now when the simulation starts
    pub fn start_simulation(&mut self) {
        if let Some(species_info) = self.species_info.get(&self.current_species) {
            // Initialize simulation parameters from species data
            let mut param = SimulationParam::new(
                species_info.genome_size as usize,
                species_info.exon_size as usize,
            );
            param.active_te = self.starting_tes.parse().unwrap_or(500);
            param.te_mobilize_prob = self.sim_mobility_prob.parse().unwrap_or(0.5);

            self.simulation_param = Some(param);
            self.start_time = Some(Instant::now());
            self.current_round = 0;
            self.simulation_rounds = self.sim_rounds.parse().unwrap_or(30);
            
            // Clear history
            self.te_lengths_histogram.clear();
            self.deletion_stats.clear();
            self.exon_ratio_history.clear();
            self.genome_size_history.clear();
            self.mutation_data.clear();
            self.probability_data.clear();
            self.round_data.clear();
            
            self.simulation_start_triggered = true;
            self.progress_bar = "[                    ] 0%".to_string();
        }
    }

    pub fn run_simulation_round(&mut self) -> bool {
        if let Some(param) = &mut self.simulation_param {
            // Run one round of simulation
            let mutations = param.run_simulation_round();

            // Update statistics
            self.te_in_exons = param.te_in_exons;
            self.te_in_noncoding = param.te_in_noncoding;
            self.tes_mobilized = param.te_mobilized;
            self.current_genome_size = param.genome_size.to_string();

            // Update history
            self.exon_ratio_history.push((self.current_round, param.get_exon_ratio()));
            self.genome_size_history.push((self.current_round, param.genome_size));

            // Update mutation data for charts
            self.mutation_data.push((self.current_round as f64, mutations as f64));
            self.probability_data.push((self.current_round as f64, param.get_mutation_rate()));

            // Record round data for export
            self.round_data.push(RoundData {
                round: self.current_round,
                genome_size: param.genome_size as f64,
                exon_length: param.exon_length,
                noncoding_length: param.noncoding_length,
                active_te: param.active_te,
                te_mobilize_prob: param.te_mobilize_prob,
                te_mobilized: param.te_mobilized,
                te_static: param.active_te.saturating_sub(param.te_mobilized),
                te_in_exons: param.te_in_exons,
                te_in_noncoding: param.te_in_noncoding,
            });

            self.current_round += 1;
            true
        } else {
            false
        }
    }

    pub fn update_simulation(&mut self) {
        if self.simulation_start_triggered {
            // Update progress bar
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
        }
    }

    pub fn has_more_rounds(&self) -> bool {
        self.current_round < self.simulation_rounds
    }
    pub fn export_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}_results.csv", self.output_dir, self.current_species);
        let mut writer = csv::Writer::from_path(file_path)?;

        // Write headers
        writer.write_record([
            "Round",
            "Genome Size",
            "Exon Length",
            "Non-Coding Length",
            "Active TEs",
            "TE Mobilize Prob",
            "TEs Mobilized",
            "TEs Static",
            "TEs in Exons",
            "TEs in Non-Coding"
        ])?;

        // Write data for each round
        for round_data in &self.round_data {
            writer.write_record(&[
                round_data.round.to_string(),
                round_data.genome_size.to_string(),
                round_data.exon_length.to_string(),
                round_data.noncoding_length.to_string(),
                round_data.active_te.to_string(),
                round_data.te_mobilize_prob.to_string(),
                round_data.te_mobilized.to_string(),
                round_data.te_static.to_string(),
                round_data.te_in_exons.to_string(),
                round_data.te_in_noncoding.to_string(),
            ])?;
        }

        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RoundData {
    pub round: usize,
    pub genome_size: f64,
    pub exon_length: usize,
    pub noncoding_length: usize,
    pub active_te: usize,
    pub te_mobilize_prob: f64,
    pub te_mobilized: usize,
    pub te_static: usize,
    pub te_in_exons: usize,
    pub te_in_noncoding: usize,
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