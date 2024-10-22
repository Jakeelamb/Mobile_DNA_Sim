// src/widgets/tabs/utils.rs
use ratatui::widgets::{Block, Borders};
use ratatui::style::{Color, Style};

pub fn create_block(title: &str, bg_color: Color) -> Block {
    Block::default()
        .title(title)
        .style(Style::default().bg(bg_color))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
}

// pub fn create_paragraph(content: &str, title: &str, bg_color: Color) -> Paragraph {
//     Paragraph::new(content)
//         .block(create_block(title, bg_color))
// }