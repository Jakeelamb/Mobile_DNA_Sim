use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::style::{Style, Color, Modifier};

use ratatui::text::{Span, Text};

pub fn render_key_bindings() -> Paragraph<'static> {
    let mut text = Text::default();

    // Header for Home Page
    text.extend(vec![
        Span::styled(
            "[Home Page]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ),
        Span::raw("\n[q] Exit                  [Right Arrow] Next Page      [Left Arrow] Previous Theme\n\n"),
    ]);

    // Header for Species List
    text.extend(vec![
        Span::styled(
            "[Species List]",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("\n[Up Arrow] Scroll Up      [Down Arrow] Scroll Down     [/] Search      [Esc] Exit Search\n\n"),
    ]);

    // Header for Simulation Settings
    text.extend(vec![
        Span::styled(
            "[Simulation Settings]",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("\n[Tab] Cycle Settings      [Backspace] Delete Editable Settings\n\n"),
    ]);

    // Header for Simulation Page
    text.extend(vec![
        Span::styled(
            "[Simulation Page]",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ),
        Span::raw("\n[s] Start Simulation      [Left Arrow] Back to Home Page\n"),
    ]);

    // Render the styled text
    Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Key Bindings")
                .title_alignment(ratatui::layout::Alignment::Center),
        )
        .style(Style::default().bg(Color::Black).fg(Color::White)) // Set background color
        .alignment(ratatui::layout::Alignment::Left) // Align text to the left
}
// pub fn render_key_bindings() -> Paragraph<'static> {
//     let key_bindings_text = "
// [Home Page]           [q] Exit                  [Right Arrow] Next Page      [Left Arrow] Previous Theme
// [Species List]        [Up Arrow] Scroll Up      [Down Arrow] Scroll Down     [/] Search      [Esc] Exit Search
// [Simulation Settings] [Tab] Cycle Settings      [Backspace] Delete Editable Settings

// [Simulation Page]     [s] Start Simulation      [Left Arrow] Back to Home Page
// ";

//     Paragraph::new(key_bindings_text)
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .title("Key Bindings")
//                 .title_alignment(ratatui::layout::Alignment::Center)
//         )
//         .style(
//             Style::default()
//                 .fg(Color::White) // Subtle, pleasant text color
//                 .bg(Color::DarkGray)     // High contrast background
//                 .add_modifier(Modifier::BOLD) // Highlight importance of keybindings
//         )
//         .alignment(ratatui::layout::Alignment::Left) // Align text to the left for readability
//         .wrap(ratatui::widgets::Wrap { trim: true }) // Wrap text to the next line if it exceeds the width of the terminal
//     }

// pub fn render_key_bindings() -> Paragraph <'static> {
//     let key_bindings_text = "
//     [Home Page]           | [q] Exit                             | [Right Arrow] Simulation Page  | [Left Arrow] Previous theme
//     [Species List]        | [Up Arrow] Up                          | [Down Arrow] Down              | [/] Search  | [Esc] Exit search
//     [Simulation Settings] | [Tab] Scrolls through Sim settings   | [delete/backspace] deletes editable settings

//     [Simulation Page]     | [s] Start simulation                 | [Left] Back to home to select new species
//     ";
//         Paragraph::new(key_bindings_text)
//             .block(Block::default().borders(Borders::ALL).title("Tab list"))
//             .style(Style::default().fg(Color::White).bg(Color::Gray)
//             .add_modifier(ratatui::style::Modifier::BOLD)
//             .add_modifier(ratatui::style::Modifier::ITALIC)
//             .add_modifier(ratatui::style::Modifier::DIM)
//             .add_modifier(ratatui::style::Modifier::UNDERLINED)
//             .add_modifier(ratatui::style::Modifier::SLOW_BLINK))
//             // .add_modifier(ratatui::style::Modifier::RAPID_BLINK))
// }