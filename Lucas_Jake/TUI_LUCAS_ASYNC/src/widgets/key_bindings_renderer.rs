use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color};

pub fn render_key_bindings() -> Paragraph <'static> {
    let key_bindings_text = "
    [Home Page]            | [q] Exit   | [Right] Simulation Page  | [Left] Previous theme
    [Select Species]      | [Up Arrow] | [Down Arrow] [/] Search | [Esc] Exit search
    [Simulation Settings] | [Tab] Select Sim settings   | [Left] Previous theme
    [Start Simulation [Enter]\n

    [Simulation Page]     | [s] Start simulation   | [Left] Back to home to select new species
    ";
        Paragraph::new(key_bindings_text)
            .block(Block::default().borders(Borders::ALL).title("Tab list"))
            .style(Style::default().fg(Color::White).bg(Color::Gray))
}