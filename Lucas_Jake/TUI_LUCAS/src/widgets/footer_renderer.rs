//footer_renderer.rs

use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color};

/// Render a footer for the UI
pub fn render_footer() -> Paragraph<'static> {
    Paragraph::new("Created by: Jake & Lucas, Version: 1.0")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Black).bg(Color::White))
}