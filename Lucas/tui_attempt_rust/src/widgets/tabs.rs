// tabs.rs
use std::io::{self};
use ratatui::style::{Style, Color};
use ratatui::text::{Span};
use ratatui::widgets::{Tabs, Block, Paragraph, Borders, List, ListItem};

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
            _ => {
                create_home_widgets(&self.species, &mut self.list_state)
            }
        }
    }
}

    // pub fn render_list(&self) -> List<'static> {
    //     // Create ListItems from the species vector
    //     let items: Vec<ListItem> = self.species.iter().enumerate().map(|(i, species)| {
    //         // Check if this item is selected and apply a style
    //         let is_selected = self.list_state.selected() == Some(i);
    //         let style = if is_selected {
    //             Style::default().bg(Color::Yellow).fg(Color::Black) // Highlight style for the selected item
    //         } else {
    //             Style::default() // Default style for unselected items
    //         };
            
    //         ListItem::new(Span::styled(species.clone(), style)) // Use styled Span
    //     }).collect();

    //     // Create and return a List widget
    //     List::new(items)
    //         .block(Block::default().title("Species List").borders(Borders::ALL))
    //         .highlight_symbol(">> ") // Symbol for highlighting
    // }
   


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