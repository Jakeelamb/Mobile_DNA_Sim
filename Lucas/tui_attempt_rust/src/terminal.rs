use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph, Tabs},
    Terminal,
};

use std::{fs, io, error::Error};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// use crate::logo::{get_ascii_logo, get_ascii_sim};
use crate::widgets::tabs::TabState;
use crate::widgets::tabs; // Import the `tabs` module

// pub fn run_app() -> Result<(), Box<dyn Error>> {
//     // Setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // Initialize the tab state
//     let mut tab_state = TabState::new();

//     loop {
//          Handle terminal events and draw
//          if let Ok(event) = event::read() {
//             match event {
//                 Event::Key(key) => {
//                     match key.code {
//                         KeyCode::Char('q') => break, // Exit on 'q'
//                         KeyCode::Right => tab_state.next(), // Cycle to the next tab
//                         KeyCode::Left => tab_state.previous(), // Cycle to the previous tab
//                         KeyCode::Down => {
//                             if tab_state.selected == 0 { // Only scroll if on the species list tab
//                                 tab_state.scroll_down(); // Scroll down in the list
//                             }
//                         }
//                         KeyCode::Up => {
//                             if tab_state.selected == 0 { // Only scroll if on the species list tab
//                                 tab_state.scroll_up(); // Scroll up in the list
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//                 _ => {}
//             }
//         }
//         terminal.draw(|f| {
//             let size = f.area(); // The total available space in the terminal
    
//             // Split the space vertically into chunks
//             let chunks = Layout::default()
//                 .direction(Direction::Vertical)
//                 .constraints([
//                     Constraint::Length(3), // Allocate 3 lines for the tabs
//                     Constraint::Min(10),  // Ensure the first block gets at least 20 lines (e.g., for the logo)
//                     Constraint::Percentage(15), // Remaining space for other content
//                 ].as_ref())
//                 .split(size); // Split the available space into chunks
    
//             // Render the Tabs widget in the first chunk (top 3 lines)
//             let tabs = tab_state.render(); // Get the tabs
//             f.render_widget(tabs, chunks[0]); // Render the tabs

//             // Get the content blocks (Paragraphs, List, etc.) from `render_content`
//             let content_blocks = tab_state.render_content();
    
//             // Render each block in the respective chunk
//             content_blocks.iter().enumerate().for_each(|(i, block)| {
//                 let chunk = *chunks.get(i+1).unwrap_or(&chunks[1]); // Dereference to pass a `Rect` instead of `&Rect`
//                 match block {
//                     tabs::Widget::SpeciesList(list) => {
//                         f.render_widget(list.clone(), chunk);
//                     }
//                     tabs::Widget::LogoBlock(paragraph) => {
//                         f.render_widget(paragraph.clone(), chunk);
//                     }
//                 }
//             });
//         })?;
//     }
//         // Handle user input (e.g., navigating the tabs or exiting the app)
//         // if let Event::Key(key) = event::read()? {
//         //     match key.code {
//         //         KeyCode::Char('q') => break, // Exit on 'q'
//         //         KeyCode::Right => tab_state.next(), // Cycle to the next tab
//         //         KeyCode::Left => tab_state.previous(), // Cycle to the previous tab
//         //         KeyCode::Down => tab_state.scroll_down(), // Scroll down in the list
//         //         KeyCode::Up => tab_state.scroll_up(), // Scroll up in the list
//         //         _ => {}
//         //     }
//         // }

//     // Restore terminal (Clean up terminal state)
//     disable_raw_mode()?;
//     execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
//     terminal.show_cursor()?;
//     Ok(())
// }
pub fn run_app() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize the tab state
    let mut tab_state = TabState::new();

    loop {
        terminal.draw(|f| {
            let size = f.size(); // Use f.size() to get the area

            // Split the space vertically into chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Allocate 3 lines for the tabs
                    Constraint::Min(10),  // Ensure the first block gets at least 10 lines (e.g., for the logo)
                    Constraint::Percentage(85), // Remaining space for other content
                ].as_ref())
                .split(size); // Split the available space into chunks

            // Render the Tabs widget in the first chunk (top 3 lines)
            let tabs = tab_state.render(); // Get the tabs
            f.render_widget(tabs, chunks[0]); // Render the tabs

            // Get the content blocks (Paragraphs, List, etc.) from `render_content`
            let content_blocks = tab_state.render_content();

            // Render each block in the respective chunk
            content_blocks.iter().enumerate().for_each(|(i, block)| {
                let chunk = *chunks.get(i + 1).unwrap_or(&chunks[1]); // Ensure we do not panic
                match block {
                    tabs::Widget::SpeciesList(list) => {
                        f.render_widget(list.clone(), chunk);
                    }
                    tabs::Widget::LogoBlock(paragraph) => {
                        f.render_widget(paragraph.clone(), chunk);
                    }
                }
            });
        })?;

        // Handle user input (e.g., navigating the tabs or exiting the app)
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break, // Exit on 'q'
                    KeyCode::Right => tab_state.next(), // Cycle to the next tab
                    KeyCode::Left => tab_state.previous(), // Cycle to the previous tab
                    KeyCode::Down => tab_state.scroll_down(), // Scroll down in the list
                    KeyCode::Up => tab_state.scroll_up(), // Scroll up in the list
                    _ => {}
                }
            }
        }
    }

    // Restore terminal (Clean up terminal state)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}