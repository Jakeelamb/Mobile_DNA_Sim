// tabs.rs
use std::io::{self};
use ratatui::style::{Style, Color};
use ratatui::text::{Span};
use ratatui::widgets::{Tabs, Block, Paragraph, Borders, List, ListState};
use serde_json::Value;
use std::fs;

mod home; //  directly references home.rs in the same directory.
mod sim;
mod utils; // Declare utils module
pub mod tabstate; // Declare tabstate module
// Import the TabState struct
use tabstate::TabState; // Import TabState from tabstate module
use home::create_home_widgets; // Now this will resolve correctly.
use sim::create_sim_widgets; // Now this will resolve correctly.

pub enum Widget {
    SpeciesList(List<'static>),
    LogoBlock(Paragraph<'static>),
}

impl TabState {    
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
            0 => create_home_widgets(&self.species, &mut self.list_state),
            1 => create_sim_widgets(),
            _ => vec![], // Return an empty vector for any unexpected index
        }
    }
}


// Do i need this later?
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