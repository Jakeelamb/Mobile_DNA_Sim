use std::io::{self}; //, BufRead};
use ratatui::style::{Style, Color};
use ratatui::text::{Span, Text}; //
use ratatui::{
    // backend::Backend, // Import Backend trait
    widgets::{Tabs, Block, Paragraph, Borders, List, ListState, ListItem}, // Import the necessary widgets
};
use serde_json::Value;
// use serde::Deserialize;
use std::fs;


use crate::logo::get_ascii_logo; // Import the get_ascii_logo function
use crate::logo::get_ascii_sim; // Import the get_ascii_sim function

pub enum Widget {
    SpeciesList(List<'static>),
    LogoBlock(Paragraph<'static>),
}

pub struct TabState {
    pub selected: usize,
    // pub scroll: u16,
    pub species: Vec<String>,
    pub list_state: ListState,
}

impl TabState {
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



    // fn render_widgets(&mut self, f: &mut ratatui::Frame, area: Rect) {
    //     let widgets = self.render_content(); // Call render_content to get the widgets
    //     for widget in widgets {
    //         match widget {
    //             Widget::SpeciesList(list) => {
    //                 f.render_stateful_widget(list, area, &mut self.list_state); // Render the List
    //             },
    //             Widget::LogoBlock(paragraph) => {
    //                 f.render_widget(paragraph, area); // Render the Paragraph
    //             },
    //         }
    //     }
    // }


    
    pub fn render(&self) -> Tabs {
        let titles = ["Home", "Simulation"];
        let tabs: Vec<Span> = titles.iter().map(|&t| Span::from(t)).collect();
    
        Tabs::new(tabs)
            .select(self.selected)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black)) // Highlight color for selected tab
            .divider(" ") // Optional: Adds a space between tabs
    }

    // New render_content method
    pub fn render_content(&mut self) -> Vec<Widget> {
        match self.selected {
            0 => {
                // Create a list of species from the CSV file
                let file_content: Vec<ListItem> = self.species.iter()
                    .map(|s| ListItem::new(Text::from(s.clone())))
                    .collect();
    
                // Determine the number of items to display based on the height of the available area
                let visible_items = 10; // Adjust based on your layout
                let total_items = file_content.len();
                let start_index = self.list_state.selected().unwrap_or(0).saturating_sub(visible_items / 2);
                // println!("Start index: {}", start_index); // Debug: Log the start index
                let end_index = (start_index + visible_items).min(total_items);
    
                // Slice the items to only show the visible portion
                let visible_content = &file_content[start_index..end_index];
    
                // Create a list with a block
                let species_list = List::new(visible_content.to_vec())
                    .block(Block::default().title("Species").borders(Borders::ALL))
                    .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black)) // Highlight style for selected item
                    .highlight_symbol(">>");
    
                // Create a logo block with the ASCII logo
                let ascii_logo = get_ascii_logo();
                let logo_block = Paragraph::new(ascii_logo)
                    .block(Block::default()
                        .title("Logo")
                        .style(Style::default().bg(Color::Blue))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White)));
    
                // Return the widgets as the new enum variant
                vec![
                    Widget::LogoBlock(logo_block),
                    Widget::SpeciesList(species_list),
                ]
            },
            1 => {
                // Simulation tab content
                let sim_logo = get_ascii_sim();
                let simulation_block = Paragraph::new(sim_logo)
                    .block(Block::default()
                        .title("Logo")
                        .style(Style::default().bg(Color::Green))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White)));
    
                let more_info_block = Paragraph::new("Here you can simulate various processes.")
                    .block(Block::default()
                        .title("More Info")
                        .style(Style::default().bg(Color::Green))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White)));
    
                vec![Widget::LogoBlock(simulation_block), Widget::LogoBlock(more_info_block)]
            },
            _ => vec![], // Return an empty vector for any unexpected index
        }
    }
}

// use ratatui::{
//     layout::{Constraint, Layout, Rect},
//     widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Text},
//     Frame,
// };
// use serde_json::Value;
// use std::{fs, io};

// pub enum Widget {
//     SpeciesList(List<'static>),
//     LogoBlock(Paragraph<'static>),
// }

// pub struct TabState {
//     pub selected: usize,
//     pub species: Vec<String>,
//     pub list_state: ListState,
// }

// impl TabState {
//     pub fn load_species_from_json(file_path: &str) -> io::Result<Vec<String>> {
//         // Read the JSON file into a string
//         let data = fs::read_to_string(file_path).map_err(|e| {
//             eprintln!("Error opening file {}: {}", file_path, e);
//             e
//         })?;

//         // Parse the JSON data
//         let json: Value = serde_json::from_str(&data).map_err(|e| {
//             eprintln!("Error parsing JSON: {}", e);
//             io::Error::new(io::ErrorKind::Other, "JSON parse error")
//         })?;

//         // Extract the species names
//         let species = json.as_array()
//             .unwrap_or(&vec![]) // If it's not an array, return an empty vector
//             .iter()
//             .filter_map(|item| item.get("species").and_then(|s| s.as_str()).map(String::from))
//             .collect();

//         Ok(species)
//     }

//     pub fn new() -> Self {
//         let species = Self::load_species_from_json("data/species.json")
//             .unwrap_or_else(|_| {
//                 println!("Error loading species");
//                 vec!["Error loading species".to_string()]
//             });
        
//         let mut list_state = ListState::default(); // Initialize the ListState
//         list_state.select(Some(0)); // Select the first item in the list

//         TabState { selected: 0, list_state, species }
//     }

//     pub fn scroll_up(&mut self) {
//         let current = self.list_state.selected().unwrap_or(0);
//         if current > 0 {
//             self.list_state.select(Some(current - 1));
//         }
//     }

//     pub fn scroll_down(&mut self) {
//         let current = self.list_state.selected().unwrap_or(0);
//         if current < self.species.len().saturating_sub(1) {
//             self.list_state.select(Some(current + 1));
//         }
//     }

//     pub fn next(&mut self) {
//         self.selected = (self.selected + 1) % 2; // Cycle between two tabs
//     }

//     pub fn previous(&mut self) {
//         self.selected = (self.selected + 1) % 2; // Cycle between two tabs
//     }

//     // New render method to layout widgets
//     pub fn render(&self, f: &mut Frame, area: Rect) {
//         // Define layout constraints for left and right sections
//         let constraints = [
//             Constraint::Percentage(40), // Left side (species list)
//             Constraint::Percentage(60), // Right side (other content)
//         ];

//         let chunks = Layout::horizontal(constraints).split(area);

//         // Render species list on the left side
//         let species_list = self.create_species_list_widget();
//         f.render_stateful_widget(species_list, chunks[0], &mut self.list_state);

//         // Render other content on the right side
//         let other_content = self.create_other_content_widget();
//         f.render_widget(other_content, chunks[1]);
//     }

//     fn create_species_list_widget(&self) -> List<'static> {
//         let items: Vec<ListItem> = self.species.iter()
//             .map(|s| ListItem::new(Text::from(s.clone())))
//             .collect();

//         List::new(items)
//             .block(Block::default().title("Species").borders(Borders::ALL))
//             .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black)) // Highlight style for selected item
//             .highlight_symbol(">>") // Highlight symbol
//     }

//     fn create_other_content_widget(&self) -> Paragraph<'static> {
//         // Create another widget for the right side content
//         Paragraph::new("Your other content here...")
//             .block(Block::default().title("Other Content").borders(Borders::ALL))
//     }

//     pub fn render_tabs(&self) -> Tabs<'static> {
//         let titles = ["Home", "Simulation"];
//         let tabs: Vec<Span> = titles.iter().map(|&t| Span::from(t)).collect();
    
//         Tabs::new(tabs)
//             .select(self.selected)
//             .block(Block::default().borders(Borders::ALL).title("Tabs"))
//             .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black)) // Highlight color for selected tab
//             .divider(" ") // Optional: Adds a space between tabs
//     }
// }