// sim.rs

use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Color, Style};
use crate::logo::get_ascii_sim; // Import the simulation logo function
use super::Widget; // Import the Widget enum

pub fn create_sim_widgets() -> Vec<Widget> {
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

    vec![
        Widget::LogoBlock(simulation_block), 
        Widget::LogoBlock(more_info_block),
    ]
}