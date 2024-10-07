//home.rs

use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::style::{Style, Color};
use ratatui::text::Span;
use crate::widgets::app_widgets::AppWidget;
use ratatui::widgets::ListState;

pub fn create_home_widgets(species: &Vec<String>, list_state: &ListState) -> Vec<AppWidget> {
    // Create a ListItem for each species
    let species_items: Vec<ListItem> = species
        .iter()
        .map(|species_name| ListItem::new(Span::from(species_name.clone())))
        .collect();

    // Create a List widget with species items and apply the highlight style
    let species_list = List::new(species_items)
        .block(Block::default().borders(Borders::ALL).title("Species List"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow)) // Highlighted item style
        .highlight_symbol(">> "); // Symbol to indicate the highlighted item

    let logo_block = Paragraph::new("Home Logo Block");

    vec![
        AppWidget::SpeciesList(species_list), // Use List for SpeciesList
        AppWidget::LogoBlock(logo_block),
    ]
}