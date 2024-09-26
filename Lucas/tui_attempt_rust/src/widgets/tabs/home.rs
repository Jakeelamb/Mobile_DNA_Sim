// home.rs
use ratatui::widgets::{List, ListItem, Paragraph};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use crate::logo::get_ascii_logo; // Import the logo function
use super::Widget; // `super` refers to the parent module, which is `tabs.rs`.
use ratatui::widgets::ListState;
use crate::widgets::tabs::utils::{create_block};

pub fn create_home_widgets(species: &Vec<String>, list_state: &mut ListState) -> Vec<Widget> {
    // Create a list of species from the CSV file
    let file_content: Vec<ListItem> = species.iter()
        .map(|s| ListItem::new(Text::from(s.clone())))
        .collect();

    // Determine the number of items to display based on the height of the available area
    let visible_items = 10; // Adjust based on your layout
    let total_items = file_content.len();
    let start_index = list_state.selected().unwrap_or(0).saturating_sub(visible_items / 2);
    let end_index = (start_index + visible_items).min(total_items);

    // Slice the items to only show the visible portion
    let visible_content = &file_content[start_index..end_index];

    // Create a list with a block
    let species_list = List::new(visible_content.to_vec())
        .block(create_block("Species", Color::Black))
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black)) // Highlight style for selected item
        .highlight_symbol(">>");

    // Create a logo block with the ASCII logo
    let ascii_logo = get_ascii_logo();
    let logo_block = Paragraph::new(ascii_logo)
        .block(create_block("Logo", Color::Blue));

    // Return the widgets as the new enum variant
    vec![
        Widget::LogoBlock(logo_block),
        Widget::SpeciesList(species_list),
    ]
}