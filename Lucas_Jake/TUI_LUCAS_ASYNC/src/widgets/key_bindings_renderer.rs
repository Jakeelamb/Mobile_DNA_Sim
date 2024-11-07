use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color};

pub fn render_key_bindings() -> Paragraph <'static> {
    let key_bindings_text = "
    [Home Page]           | [q] Exit                             | [Right Arrow] Simulation Page  | [Left Arrow] Previous theme
    [Species List]      | [Up Arrow] Up                        | [Down Arrow] Down              | [/] Search  | [Esc] Exit search
    [Simulation Settings] | [Tab] Scrolls through Sim settings   | [delete/backspace] deletes editable settings

    [Simulation Page]     | [s] Start simulation                 | [Left] Back to home to select new species
    ";
        Paragraph::new(key_bindings_text)
            .block(Block::default().borders(Borders::ALL).title("Tab list"))
            .style(Style::default().fg(Color::White).bg(Color::Gray))
}