//header_renderer.rs
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color};
use crate::logo::{get_ascii_logo, get_ascii_sim};
use crate::widgets::tabstate::TabState;



pub fn render_header(tab_state: &TabState) -> Paragraph <'static> {
    // Choose header text and colors based on the active tab index
    let (header_text, title_color, text_color, bg_color) = match tab_state.index {
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