use std::io::{self}; //, BufRead};
use ratatui::style::{Style, Color};
use ratatui::text::{Span, Text}; //
use ratatui::{
    backend::Backend, // Import Backend trait
    Frame, // Import Frame type
    layout::Rect,
    widgets::{Tabs, Block, Paragraph, Borders, List, ListState, ListItem},
};
use serde_json::Value;
use serde::Deserialize;
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

    pub fn scroll_up(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        if current > 0 {
            self.list_state.select(Some(current - 1));
        }
    }

    pub fn scroll_down(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        if current < self.species.len().saturating_sub(1) {
            self.list_state.select(Some(current + 1));
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % 2; // Now cycling between two tabs
    }

    pub fn previous(&mut self) {
        self.selected = (self.selected + 1) % 2; // Now cycling between two tabs
    }


    fn render_widgets(&mut self, f: &mut ratatui::Frame, area: Rect) {
        let widgets = self.render_content(); // Call render_content to get the widgets
        for widget in widgets {
            match widget {
                Widget::SpeciesList(list) => {
                    f.render_stateful_widget(list, area, &mut self.list_state); // Render the List
                },
                Widget::LogoBlock(paragraph) => {
                    f.render_widget(paragraph, area); // Render the Paragraph
                },
            }
        }
    }


    
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
    pub fn render_content(&self) -> Vec<Widget> {
        match self.selected {
            0 => {
                // Create a list of species from the CSV file
                let file_content = self.species.iter()
                    .map(|s| ListItem::new(Text::from(s.clone()))) // Create ListItems for each species
                    .collect::<Vec<_>>();
                // let file_content: Vec<ListItem> = self.species.iter().map(|s| ListItem::new(s.clone())).collect();
    
                // Create a list with a block
                let species_list = List::new(file_content)
                    .block(Block::default().title("Species").borders(Borders::ALL))
                    .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black)) // Highlight style for selected item
                    .highlight_symbol(">>"); // Optional: Customize the highlight symbol
    
    
                // Create a logo block with the ASCII logo
                let ascii_logo = get_ascii_logo();
                let logo_block = Paragraph::new(ascii_logo)
                    .block(Block::default()
                        .title("Logo")
                        .style(Style::default().bg(Color::Blue))
                        .borders(ratatui::widgets::Borders::ALL)
                        .border_style(Style::default().fg(Color::White))); // Customize border style if needed
    
                // Return the widgets as the new enum variant
                vec![Widget::LogoBlock(logo_block), Widget::SpeciesList(species_list),] // Use enum to return mixed types
            },
            1 => {
                // Simulation tab content (same as before)
                let sim_logo = get_ascii_sim();
                let simulation_block = Paragraph::new(sim_logo)
                    .block(Block::default()
                        .title("Logo")
                        .style(Style::default().bg(Color::Green))
                        .borders(ratatui::widgets::Borders::ALL)
                        .border_style(Style::default().fg(Color::White)));
    
                let more_info_block = Paragraph::new("Here you can simulate various processes.")
                    .block(Block::default()
                        .title("More Info")
                        .style(Style::default().bg(Color::Green))
                        .borders(ratatui::widgets::Borders::ALL)
                        .border_style(Style::default().fg(Color::White)));
    
                vec![Widget::LogoBlock(simulation_block), Widget::LogoBlock(more_info_block)] // Adjust as necessary
            },
            _ => vec![], // Return an empty vector for any unexpected index
        }
    }
}

// use std::io::{self}; // Import necessary I/O traits
// use ratatui::style::{Style, Color};
// use ratatui::text::{Span, Text};
// use ratatui::{
//     backend::Backend,
//     Frame,
//     layout::{Rect, Constraint, Layout},
//     widgets::{Tabs, Block, Paragraph, Borders, List, ListState, ListItem}, // Import List and other widgets
// };
// use serde_json::Value;
// use serde::Deserialize;
// use std::fs;

// use crate::logo::get_ascii_logo;
// use crate::logo::get_ascii_sim;

// pub enum Widget {
//     SpeciesList(List<'static>),  // Ensure List is properly imported
//     LogoBlock(Paragraph<'static>),
// }

// pub struct TabState {
//     pub selected: usize,
//     pub home_tab: HomeTab,
//     pub sim_tab: SimTab,
// }

// impl TabState {
//     pub fn new() -> Self {
//         let home_tab = HomeTab::new();
//         let sim_tab = SimTab;

//         TabState { selected: 0, home_tab, sim_tab }
//     }

//     pub fn next(&mut self) {
//         self.selected = (self.selected + 1) % 2; // Now cycling between two tabs
//     }

//     pub fn previous(&mut self) {
//         self.selected = if self.selected == 0 {
//             1
//         } else {
//             0
//         }; // Switch to the other tab
//     }

//     pub fn render(&self) -> Tabs {
//         let titles = ["Home", "Simulation"];
//         let tabs: Vec<Span> = titles.iter().map(|&t| Span::from(t)).collect();

//         Tabs::new(tabs)
//             .select(self.selected)
//             .block(Block::default().borders(Borders::ALL).title("Tabs"))
//             .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black)) // Highlight color for selected tab
//             .divider(" ") // Optional: Adds a space between tabs
//     }

//     pub fn render_content(&self) -> Vec<Widget> {
//         match self.selected {
//             0 => {
//                 let species_list = List::new(self.home_tab.render())
//                     .block(Block::default().title("Species").borders(Borders::ALL))
//                     .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
//                     .highlight_symbol(">>");

//                 let logo_block = self.home_tab.render_logo();

//                 vec![Widget::LogoBlock(logo_block), Widget::SpeciesList(species_list)]
//             },
//             1 => {
//                 let logo_block = self.sim_tab.render_logo();
//                 let more_info_block = self.sim_tab.render_more_info();

//                 vec![Widget::LogoBlock(logo_block), Widget::LogoBlock(more_info_block)]
//             },
//             _ => vec![], // Return an empty vector for any unexpected index
//         }
//     }
// }