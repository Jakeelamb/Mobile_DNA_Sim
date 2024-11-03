// // terminal.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::footer_renderer::render_footer;
use crate::widgets::header_renderer::render_header;
use crate::widgets::home_renderer::render_home_widgets;
use crate::widgets::input_handler::{next, previous, scroll_down, scroll_up};
use crate::widgets::key_bindings_renderer::render_key_bindings;
use crate::widgets::sim_renderer::{render_current_round, render_run_time}; //, render_mutation_chart, render_probability_chart};
use crate::widgets::sim_renderer::render_sim_widgets;
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, ListState};


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Terminal,
};
use std::{error::Error, io};

/// Set up the terminal with Crossterm backend
fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn setup_chunks(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Combined height for Tabs and Header
            Constraint::Min(8),     // Header (logo)
            Constraint::Min(15),    // Main content area
            Constraint::Length(3),  // Footer area
            Constraint::Length(10), // Key Bindings area
        ])
        .split(area)
        .to_vec() // Convert Rc<[Rect]> to Vec<Rect>
}

/// Main application function
pub fn run_app() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let mut tab_state = TabState::new();

    loop {
        terminal.draw(|f| {
            // Set up the general layout
            let chunks = setup_chunks(f.area());

            // Conditionally set layout based on the active tab
            let tab_and_info_chunks = if tab_state.index == 0 {
                // Home tab: 100% width for tabs
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)]) // 100% for tabs, no right-side info
                    .split(chunks[0])
            } else {
                // Simulation tab: 70% for tabs, 30% for Run Time and Current Round
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(70), // Tabs section
                        Constraint::Percentage(30), // Info (Run Time + Current Round) section
                    ])
                    .split(chunks[0])
            };

            // Render the Tabs
            let tabs = tab_state.render();
            f.render_widget(tabs, tab_and_info_chunks[0]);

            // Render the Header (logo, etc.)
            let header = render_header(&tab_state);
            f.render_widget(header, chunks[1]);

            // Only render Run Time and Current Round on the Simulation tab (index 1)
            if tab_state.index == 1 {
                let info_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50), // Run Time block
                        Constraint::Percentage(50), // Current Round block
                    ])
                    .split(tab_and_info_chunks[1]);

                // Render the Run Time and Current Round next to the tabs
                let run_time_block = render_run_time(&tab_state.run_time);
                f.render_widget(run_time_block, info_chunks[0]);

                let current_round_block = render_current_round(tab_state.current_round, 10000);
                f.render_widget(current_round_block, info_chunks[1]);
            }

            // Render content for the Home tab
            if tab_state.index == 0 {
                // Split the main content area for Home tab into three chunks (SpeciesList, InfoBlock, SettingsBlock)
                let content_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(32), // SpeciesList
                        Constraint::Percentage(36), // InfoBlock
                        Constraint::Percentage(32), // SettingsBlock
                    ])
                    .split(chunks[2]);

                // Retrieve the widgets for the Home tab content (SpeciesList, InfoBlock, SettingsBlock)
                let content_blocks = render_home_widgets(&mut tab_state);

                // Render each block in the corresponding chunk
                content_blocks.iter().enumerate().for_each(|(i, block)| {
                    if let Some(chunk) = content_chunks.get(i) {
                        match block {
                            AppWidget::SpeciesList(list) => {
                                f.render_stateful_widget(list.clone(), *chunk, &mut tab_state.list_state);
                            }
                            AppWidget::InfoBlock(info) => {
                                f.render_widget(info.clone(), *chunk);
                            }
                            AppWidget::SettingsBlock(settings) => {
                                f.render_widget(settings.clone(), *chunk);
                            }
                            _ => {}
                        }
                    }
                });
            } else {
                // Render all widgets for the Simulation tab
                render_sim_widgets(&tab_state, f, chunks[2]);
            }

            // Render the Footer
            let footer = render_footer();
            f.render_widget(footer, chunks[3]);

            // Render the Key Bindings
            let key_bindings = render_key_bindings();
            f.render_widget(key_bindings, chunks[4]);
        })?;

        // Handle user input events
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break, // Quit the application
                    KeyCode::Right => next(&mut tab_state), // Navigate between tabs
                    KeyCode::Left => previous(&mut tab_state), // Navigate between tabs
                    // KeyCode::Down => scroll_down(&mut tab_state), // Scroll down the species list
                    // KeyCode::Up => scroll_up(&mut tab_state), // Scroll up the species list
                    KeyCode::Down => {
                        // Scroll down the list
                        if tab_state.search_mode && !tab_state.filtered_species.is_empty() {
                            scroll_down_with_list(&mut tab_state.list_state, &tab_state.filtered_species);
                        } else {
                            scroll_down(&mut tab_state);
                        }
                    }
                    KeyCode::Up => {
                        // Scroll up the list
                        if tab_state.search_mode && !tab_state.filtered_species.is_empty() {
                            scroll_up_with_list(&mut tab_state.list_state, &tab_state.filtered_species);
                        } else {
                            scroll_up(&mut tab_state);
                        }
                    }

                    KeyCode::Char('/') => {
                        // Enter search mode
                        tab_state.search_mode = true;
                        tab_state.search_query.clear();
                        // tab_state.filtered_species = tab_state.species.clone();
                        tab_state.list_state.select(Some(0));
                    }

                    KeyCode::Char(c) => {
                        if tab_state.search_mode {
                            // Handle search query input
                            tab_state.search_query.push(c);
                            tab_state.filtered_species = tab_state.species
                                .iter()
                                .filter(|name| name.to_lowercase().contains(&tab_state.search_query.to_lowercase()))
                                .cloned()
                                .collect();
                            tab_state.list_state.select(Some(0));
                        } else {
                            // Handle regular input for simulation settings
                            match tab_state.active_input {
                                0 => tab_state.starting_tes.push(c), // Edit Starting TEs
                                1 => tab_state.sim_rounds.push(c),   // Edit Simulation Rounds
                                2 => tab_state.cpu_cores.push(c),    // Edit CPU cores
                                3 => tab_state.output_dir.push(c),   // Edit Output Directory
                                _ => {}
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        if tab_state.search_mode {
                            // Handle search query backspace
                            tab_state.search_query.pop();
                            tab_state.filtered_species = tab_state.species
                                .iter()
                                .filter(|name| name.to_lowercase().contains(&tab_state.search_query.to_lowercase()))
                                .cloned()
                                .collect();
                        } else {
                            // Handle backspace for simulation settings input
                            match tab_state.active_input {
                                0 => { tab_state.starting_tes.pop(); }
                                1 => { tab_state.sim_rounds.pop(); }
                                2 => { tab_state.cpu_cores.pop(); }
                                3 => { tab_state.output_dir.pop(); }
                                _ => {}
                            }
                        }
                    }

                    KeyCode::Esc => {
                        // Exit search mode
                        tab_state.search_mode = false;
                        tab_state.search_query.clear();
                        tab_state.filtered_species.clear();
                        tab_state.list_state.select(Some(0));
                    }

                    KeyCode::Tab => {
                        // Cycle through simulation settings inputs
                        tab_state.active_input = (tab_state.active_input + 1) % 4;
                    }

                    _ => {}
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

    /// Scroll down in the species list, keeping the selection within bounds
fn scroll_down_with_list(state: &mut ListState, items: &[String]) {
    let i = match state.selected() {
        Some(i) => {
            if i >= items.len() - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}

/// Scroll up in the species list, keeping the selection within bounds
fn scroll_up_with_list(state: &mut ListState, items: &[String]) {
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                items.len() - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}