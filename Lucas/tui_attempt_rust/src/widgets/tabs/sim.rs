// sim.rs

use ratatui::widgets::{Paragraph};
use ratatui::style::{Color};
use crate::logo::get_ascii_sim; // Import the simulation logo function
use super::Widget; // Import the Widget enum
use crate::widgets::tabs::utils::{create_block}; // Import the create_block function

pub fn create_sim_widgets() -> Vec<Widget> {
    // Simulation tab content
    let sim_logo = get_ascii_sim();
    let simulation_block = Paragraph::new(sim_logo)
        .block(create_block("Logo", Color::Green));

    let more_info_block = Paragraph::new("Here you can simulate various processes.")
        .block(create_block("More Info", Color::Green));

    vec![
        Widget::LogoBlock(simulation_block), 
        Widget::LogoBlock(more_info_block),
    ]
}