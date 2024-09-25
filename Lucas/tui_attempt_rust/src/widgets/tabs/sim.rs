// use ratatui::widgets::{Tabs, Block};
// use ratatui::style::{Color, Style};
// use ratatui::text::{Span, Line};
// use ratatui::layout::Rect;
// use ratatui::buffer::Buffer;

// pub struct TabState {
//     pub selected: usize,
// }

// impl TabState {
//     pub fn new() -> Self {
//         TabState { selected: 0 }
//     }

//     pub fn next(&mut self) {
//         self.selected = (self.selected + 1) % 2; // Adjust number of tabs
//     }

//     pub fn previous(&mut self) {
//         if self.selected > 0 {
//             self.selected -= 1;
//         } else {
//             self.selected = 1; // Adjust to number of tabs - 1
//         }
//     }

//     pub fn render(&self) -> Tabs {
//         let titles = vec![
//             Line::from(Span::styled("Home", Style::default().fg(Color::Red))),
//             Line::from(Span::styled("Simulation", Style::default().fg(Color::Blue))),
//         ];

//         Tabs::new(titles)
//             .select(self.selected)
//             .block(Block::default().borders(ratatui::widgets::Borders::ALL).title("Tabs"))
//             .highlight_style(Style::default().fg(Color::Yellow)) // Highlight selected tab
//             .divider("|") // Divider between tabs
//     }
// }

use ratatui::widgets::{Tabs, Block};
use ratatui::style::{Style, Color};
use ratatui::text::{Span, Line};

pub struct TabState {
    pub selected: usize,
}

impl TabState {
    pub fn new() -> Self {
        TabState { selected: 0 }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % 2; // Now cycling between two tabs
    }

    pub fn previous(&mut self) {
        self.selected = (self.selected + 1) % 2; // Now cycling between two tabs
    }

    pub fn render(&self) -> Tabs {
        let titles = ["Home", "Simulation"];
        let tabs: Vec<_> = titles.iter().map(|&t| Span::from(t)).collect();

        Tabs::new(tabs)
            .select(self.selected)
            .block(Block::default().borders(ratatui::widgets::Borders::ALL).title("Tabs"))
            .highlight_style(Style::default().fg(Color::Yellow))
    }
}