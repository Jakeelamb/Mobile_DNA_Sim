// src/widgets/tabstate.rs
use crate::widgets::app_widgets::{get_ascii_logo, get_ascii_sim};
use crate::widgets::utils::create_block;
use ratatui::style::{Style, Color};
use ratatui::text::{Span, Line};
use ratatui::widgets::{Tabs, Block, Borders, Paragraph, ListState, ListItem, List};
use crate::widgets::app_widgets::{AppWidget};

pub struct TabState {
    pub index: usize,
    pub species: Vec<String>, // List of species for the home tab
    pub list_state: ListState, // State management for the species list
}

impl TabState {
    pub fn new() -> TabState {
        let mut state = ListState::default();
        state.select(Some(0)); // Start with the first item selected
        TabState {
            index: 0,
            species: vec![
                "Bulbasaur".to_string(),
                "Ivysaur".to_string(),
                "Venusaur".to_string(),
                "Charmander".to_string(),
                "Squirtle".to_string(),
                "Jigglypuff".to_string(),
                "Gengar".to_string(),
                "Snorlax".to_string(),
                "Pikachu".to_string(),
                "Eevee".to_string(),
                "Mewtwo".to_string(),
                "Lucario".to_string(),
                "Greninja".to_string(),
                "Zacian".to_string(),
                "Zamazenta".to_string(),
                "Calyrex".to_string(),
                "Urshifu".to_string(),
                "Zarude".to_string(),
                "Regieleki".to_string(),
                "Regidrago".to_string(),
                "Glastrier".to_string(),
                "Spectrier".to_string(),
                "Cinderace".to_string(),
            ],
            list_state: state,  // Initialize list state
        }
    }

    pub fn render(&self) -> Tabs {
        let titles = ["Home", "Simulation"];
        let tab_titles: Vec<Line> = titles.iter().map(|t| Line::from(Span::raw(*t))).collect(); // Updated to use `Line`
        Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.index)
            .highlight_style(Style::default().fg(Color::Yellow))
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

     /// Render the main content area based on the active tab
    pub fn render_content(&self) -> Vec<AppWidget> {
        match self.index {
            0 => self.render_home_widgets(),  // Home tab content
            1 => self.render_sim_widgets(),   // Simulation tab content
            _ => self.render_home_widgets(),
        }
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

        let logo_block = Paragraph::new("Home Logo Block");

        vec![
            AppWidget::SpeciesList(species_list), // Use List for SpeciesList
            AppWidget::LogoBlock(logo_block),
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