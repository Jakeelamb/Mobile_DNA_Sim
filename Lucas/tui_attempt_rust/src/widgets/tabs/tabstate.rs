use ratatui::widgets::ListState;
use std::io;
use std::fs;
use serde_json::Value;

pub struct TabState {
    pub selected: usize,
    pub species: Vec<String>,
    pub list_state: ListState,
}

impl TabState {
    pub fn new() -> Self {
        let species = Self::load_species_from_json("data/species.json")
            .unwrap_or_else(|_| {
                println!("Error loading species");
                vec!["Error loading species".to_string()]
            });
        
        let mut list_state = ListState::default(); // Initialize the ListState
        list_state.select(Some(0)); // Select the first item in the list

        TabState { selected: 0, list_state, species }
    }

    pub fn load_species_from_json(file_path: &str) -> io::Result<Vec<String>> {
        // Read the JSON file into a string
        let data = fs::read_to_string(file_path).map_err(|e| {
            eprintln!("Error opening file {}: {}", file_path, e);
            e
        })?;

        // Parse the JSON data
        let json: Value = serde_json::from_str(&data).map_err(|e| {
            eprintln!("Error parsing JSON: {}", e);
            io::Error::new(io::ErrorKind::Other, "JSON parse error")
        })?;

        // Extract the species names
        let species = json.as_array()
            .unwrap_or(&vec![]) // If it's not an array, return an empty vector
            .iter()
            .filter_map(|item| item.get("species").and_then(|s| s.as_str()).map(String::from))
            .collect();

        Ok(species)
    }

    pub fn scroll_down(&mut self) {
        // Ensure this updates immediately when the down key is pressed
        if let Some(selected) = self.list_state.selected() {
            let new_selected = if selected >= self.species.len() - 1 {
                selected // Stay at the last item
            } else {
                selected + 1 // Move down
            };
            // println!("Scroll down: {}", new_selected); // Debug: Log the new index
            self.list_state.select(Some(new_selected));
        }
    }
    
    pub fn scroll_up(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let new_selected = if selected == 0 {
                0 // Stay at the first item
            } else {
                selected - 1 // Move up
            };
            // println!("Scroll up: {}", new_selected); // Debug: Log the new index
            self.list_state.select(Some(new_selected));
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % 2; // Now cycling between two tabs
    }

    pub fn previous(&mut self) {
        self.selected = (self.selected + 1) % 2; // Now cycling between two tabs
    }
}
