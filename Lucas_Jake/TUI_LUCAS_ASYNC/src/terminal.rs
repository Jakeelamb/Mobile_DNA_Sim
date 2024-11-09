// // terminal.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::footer_renderer::render_footer;
use crate::widgets::header_renderer::render_header;
use crate::widgets::home_renderer::render_home_widgets;
use crate::widgets::input_handler::{next, previous, scroll_down, scroll_up};
use crate::widgets::key_bindings_renderer::render_key_bindings;
use crate::widgets::sim_renderer::render_sim_widgets;
use crate::widgets::sim_renderer::{render_current_round, render_run_time}; //, render_mutation_chart, render_probability_chart};
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, ListState, Paragraph};
use ratatui::widgets::{Gauge, Tabs};
use ratatui::Frame;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Terminal,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{error::Error, io};

/// Restores the terminal state to its original settings.
fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

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
    let tab_state = Arc::new(Mutex::new(TabState::new()));

    // Spawn simulation logic thread
    let simulation_tab_state = Arc::clone(&tab_state);
    thread::spawn(move || loop {
        if let Ok(mut state) = simulation_tab_state.try_lock() {
            if state.simulation_start_triggered {
                if state.has_more_rounds() {
                    state.run_simulation_round();
                    state.update_simulation();
                    // state.needs_redraw = true; // Mark for UI update
                } else {
                    state.simulation_start_triggered = false;
                    if let Err(e) = state.export_to_csv() {
                        eprintln!("Failed to export to CSV: {:?}", e);
                    }
                    // state.needs_redraw = true; // Mark for final update
                }
            }
        }
        thread::sleep(Duration::from_millis(2)); // NOTE! If this line commented out, no need to move mouse.
    });

    // Main application loop
    loop {
        // Render UI
        terminal.draw(|f| {
            let chunks = setup_chunks(f.area());
            let mut state = tab_state.lock().unwrap();
            render_ui(&mut state, f, chunks);
        })?;

        // Handle user input
        if let Ok(event) = event::read() {
            let mut state = tab_state.lock().unwrap();
            if handle_event(&mut state, event) {
                break; // Exit application on quit signal
            }
            // state.needs_redraw = true; // Mark for UI update
        }

        // Small delay to avoid high CPU usage
        thread::sleep(Duration::from_millis(10));
    }

    // Cleanup terminal
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

fn render_ui(state: &mut TabState, f: &mut Frame, chunks: Vec<ratatui::layout::Rect>) {
    // let start_button_style = if state.simulation_start_triggered {
    //     Style::default()
    //         .fg(Color::Black) // Text color for running state
    //         .bg(Color::LightGreen) // Background color for running state
    //         .add_modifier(Modifier::BOLD | Modifier::ITALIC) // Emphasis for running state
    // } else {
    //     Style::default()
    //         .fg(Color::White) // Text color for idle state
    //         .bg(Color::Blue) // Background color for idle state
    //         .add_modifier(Modifier::BOLD) // Emphasis for idle state
    // };

    // // Render Start Simulation button
    // let start_button = Paragraph::new(Span::styled(
    //     if state.simulation_start_triggered {
    //         "Simulation Running"
    //     } else {
    //         "Start Simulation"
    //     },
    //     start_button_style,
    // ))
    // .block(
    //     Block::default()
    //         .borders(Borders::ALL)
    //         .style(Style::default().bg(Color::Gray).fg(Color::White))
    //         .border_style(Style::default().fg(Color::Gray))
    //         .title(""),
    // );
    // f.render_widget(start_button, chunks[0]); // Render button in the first chunk

    // // Render Progress Bar
    // let progress = if state.simulation_rounds > 0 {
    //     state.simulation_rounds_completed as f64 / state.simulation_rounds as f64
    // } else {
    //     0.0
    // };
    // let progress_bar = Gauge::default()
    //     .block(Block::default().title("Progress"))
    //     .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //     .ratio(progress); // Render progress as a fraction
    // f.render_widget(progress_bar, chunks[1]); // Render progress bar in the second chunk

    // Conditionally set layout based on the active tab
    let tab_and_info_chunks = if state.index == 0 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)]) // 100% for tabs, no right-side info
            .split(chunks[0])
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Tabs section
                Constraint::Percentage(50), // Info (Run Time + Current Round) section
            ])
            .split(chunks[0])
    };

    // Render the Tabs
    let tabs = state.render();
    f.render_widget(tabs, tab_and_info_chunks[0]);

    // Render the Header (logo, etc.)
    let header = render_header(state);
    f.render_widget(header, chunks[1]);

    if state.index == 1 {
        // Call chart rendering function for the Simulation tab

        let info_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Progress Bar block
                Constraint::Percentage(50), // Current Round block
            ])
            .split(tab_and_info_chunks[1]);

        // Render the Progress Bar
        let progress_paragraph = Paragraph::new(Span::styled(
            &state.progress_bar,
            Style::default().fg(Color::Green),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Simulation Progress"),
        );
        f.render_widget(progress_paragraph, info_chunks[0]);

        // Render the Current Round in the second half of info_chunks
        let current_round_block =
            render_current_round(state.current_round, state.simulation_rounds);
        f.render_widget(current_round_block, info_chunks[1]);
    }

    if state.index == 0 {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(32), // SpeciesList
                Constraint::Percentage(36), // InfoBlock
                Constraint::Percentage(32), // SettingsBlock
            ])
            .split(chunks[2]);

        let content_blocks = render_home_widgets(state);

        content_blocks.iter().enumerate().for_each(|(i, block)| {
            if let Some(chunk) = content_chunks.get(i) {
                match block {
                    AppWidget::SpeciesList(list) => {
                        f.render_stateful_widget(list.clone(), *chunk, &mut state.list_state);
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
        render_sim_widgets(state, f, chunks[2]);
    }

    // Render the Footer
    let footer = render_footer();
    f.render_widget(footer, chunks[3]);

    // Render the Key Bindings
    let key_bindings = render_key_bindings();
    f.render_widget(key_bindings, chunks[4]);
}

// state.reset_simulation_results();
// state.needs_redraw = true;

// if state.needs_redraw {
//     terminal.draw(|f| {
//         let chunks = setup_chunks(f.area());
//         render_application_ui(&state, f, chunks);
//     })?;
//     state.needs_redraw = false; // Reset the flag
// }

// Render the UI only if needed
// {
//     let mut state = tab_state.lock().unwrap();
//     if state.needs_redraw {
//         drop(state); // Release lock
//         terminal.draw(|f| {
//             let chunks = setup_chunks(f.area());
//             let mut state = tab_state.lock().unwrap(); // Re-lock for rendering
//             render_ui(&mut state, f, chunks);
//         })?;
//         state = tab_state.lock().unwrap(); // Re-lock to reset the flag
//         state.needs_redraw = false;
//     }
// }

fn handle_event(state: &mut TabState, event: Event) -> bool {
    match event {
        Event::Key(key) => match key.code {
            // Quit the application
            KeyCode::Char('q') => return true,

            // Navigate between tabs
            KeyCode::Right => {
                next(state);
                let selected_species = state
                    .filtered_species
                    .get(state.list_state.selected().unwrap_or(0))
                    .cloned();

                if let Some(selected_species) = selected_species {
                    if selected_species != state.current_species {
                        // Reset simulation results only if a new species is selected
                        state.current_species = selected_species;
                    }
                }
            }
            // next(state);
            // handle_tab_change(state);
            KeyCode::Left => {
                next(state);
                let selected_species = state
                    .filtered_species
                    .get(state.list_state.selected().unwrap_or(0))
                    .cloned();

                if let Some(selected_species) = selected_species {
                    if selected_species != state.current_species {
                        // Reset simulation results only if a new species is selected
                        state.current_species = selected_species;
                    }
                }
            }
            // previous(state);
            // handle_tab_change(state);

            // Handle scrolling
            KeyCode::Down => {
                if state.index == 0 {
                    if state.search_mode && !state.filtered_species.is_empty() {
                        let filtered_species = state.filtered_species.clone();
                        scroll_down_with_list(&mut state.list_state, &filtered_species);
                    } else {
                        scroll_down(state);
                    }
                }
            }
            KeyCode::Up => {
                if state.index == 0 {
                    if state.search_mode && !state.filtered_species.is_empty() {
                        let filtered_species = state.filtered_species.clone();
                        scroll_up_with_list(&mut state.list_state, &filtered_species);
                    } else {
                        scroll_up(state);
                    }
                }
            }

            // Handle entering search mode
            KeyCode::Char('/') => {
                if state.index == 0 {
                    state.search_mode = true;
                    state.search_query.clear();
                    state.list_state.select(Some(0));
                }
            }

            // Handle regular character input
            // KeyCode::Char(c) => {
            //     match state.index {
            //         // Simulation Tab Logic
            //         1 => {
            //             if c == 's' && !state.search_mode {
            //                 if !state.simulation_start_triggered {
            //                     // Start simulation logic
            //                     state.reset_simulation_results();
            //                     state.start_simulation();
            //                     state.simulation_start_triggered = true;
            //                     println!("Simulation started.");
            //                 }
            //             }
            //         }

            //         // Home Tab Logic
            //         0 => {
            //             if state.search_mode {
            //                 // If in search mode, update the search query
            //                 state.search_query.push(c);
            //                 state.filtered_species = state
            //                     .species
            //                     .iter()
            //                     .filter(|name| {
            //                         name.to_lowercase()
            //                             .contains(&state.search_query.to_lowercase())
            //                     })
            //                     .cloned()
            //                     .collect();
            //                 state.list_state.select(Some(0)); // Reset selection
            //             } else if let Some(active_field) = (0..=4).contains(&state.active_input).then_some(state.active_input) {
            //                 // Handle input for active fields (only valid fields 0-4)
            //                 match active_field {
            //                     0 => state.starting_tes.push(c),      // Edit Starting TEs
            //                     1 => state.sim_rounds.push(c),        // Edit Simulation Rounds
            //                     2 => state.cpu_cores.push(c),         // Edit CPU cores
            //                     3 => state.sim_mobility_prob.push(c), // Edit Probability of TE Mutation
            //                     4 => state.output_dir.push(c),        // Edit Output Directory
            //                     _ => unreachable!(), // This should never be reached due to bounds checking
            //                 }
            //             } else {
            //                 println!("No active input field or invalid state for editing: {}", state.active_input);
            //             }
            //         }

            //         // Unknown State Index
            //         _ => {
            //             println!("Unhandled state index: {}", state.index);
            //         }
            //     }
            // }
            KeyCode::Char(c) => {
                if state.index == 1 && c == 's' {
                    // Start the simulation on the Simulation page if 's' is pressed
                    if !state.simulation_start_triggered {
                        state.reset_simulation_results();
                        state.start_simulation(); // Set start time and reset rounds
                        state.simulation_start_triggered = true;
                    }
                } else if state.index == 0 {
                    // Handle search mode and settings input only on the Home page
                    if state.search_mode {
                        // Add the character to the search query in search mode
                        state.search_query.push(c);
                        state.filtered_species = state
                            .species
                            .iter()
                            .filter(|name| {
                                name.to_lowercase()
                                    .contains(&state.search_query.to_lowercase())
                            })
                            .cloned()
                            .collect();

                        // Keep the current selection within the filtered list
                        state.list_state.select(Some(0));
                    } else if c == 's' {
                        // Ignore 's' keypress when not in search mode on the Home page
                    } else {
                        // Handle regular input for simulation settings if not in search mode
                        match state.active_input {
                            0 => state.starting_tes.push(c),      // Edit Starting TEs
                            1 => state.sim_rounds.push(c),        // Edit Simulation Rounds
                            2 => state.cpu_cores.push(c),         // Edit CPU cores
                            3 => state.sim_mobility_prob.push(c), // Edit Probability of TE Mutation
                            4 => state.output_dir.push(c),        // Edit Output Directory

                            _ => println!("Invalid active input field: {}", state.active_input),
                        }
                    }
                }
            }

            //OG CODE
            // KeyCode::Char(c) => {
            //     if state.index == 1 && c == 's' && !state.search_mode {
            //         // Start the simulation on the Simulation page if 's' is pressed
            //         if !state.simulation_start_triggered {
            //             state.reset_simulation_results();
            //             state.start_simulation(); // Set start time and reset rounds
            //             state.simulation_start_triggered = true;
            //             // state.needs_redraw = true; // Force UI redraw
            //         }
            //     } else if state.index == 0 {
            //         // Handle search mode and settings input only on the Home page
            //         if state.search_mode {
            //             // Add the character to the search query in search mode
            //             state.search_query.push(c);
            //             state.filtered_species = state
            //                 .species
            //                 .iter()
            //                 .filter(|name| {
            //                     name.to_lowercase()
            //                         .contains(&state.search_query.to_lowercase())
            //                 })
            //                 .cloned()
            //                 .collect();

            //             // Keep the current selection within the filtered list
            //             state.list_state.select(Some(0));
            //         } else {
            //             // Handle regular input for simulation settings if not in search mode
            //             match state.active_input {
            //                 0 => state.starting_tes.push(c),      // Edit Starting TEs
            //                 1 => state.sim_rounds.push(c),        // Edit Simulation Rounds
            //                 2 => state.cpu_cores.push(c),         // Edit CPU cores
            //                 3 => state.sim_mobility_prob.push(c), // Edit Probability of TE Mutation
            //                 4 => state.output_dir.push(c),        // Edit Output Directory

            //                 _ => {}
            //             }
            //         }
            //     }
            // }

            // Handle backspace
            KeyCode::Backspace => {
                if state.search_mode {
                    state.search_query.pop();
                    state.filtered_species = state
                        .species
                        .iter()
                        .filter(|name| {
                            name.to_lowercase()
                                .contains(&state.search_query.to_lowercase())
                        })
                        .cloned()
                        .collect();
                } else if state.index == 0 {
                    match state.active_input {
                        0 => {
                            state.starting_tes.pop();
                        }
                        1 => {
                            state.sim_rounds.pop();
                        }
                        2 => {
                            state.cpu_cores.pop();
                        }
                        3 => {
                            state.sim_mobility_prob.pop();
                        }
                        4 => {
                            state.output_dir.pop();
                        }
                        _ => {}
                    }
                }
            }

            // Exit search mode
            KeyCode::Esc => {
                state.search_mode = false;
                state.search_query.clear();
                state.filtered_species.clear();
                state.list_state.select(Some(0));
            }

            // Cycle through simulation settings inputs
            KeyCode::Tab => {
                if state.index == 0 {
                    state.active_input = (state.active_input + 1) % 5;
                }
            }
            _ => {}
        },
        _ => {}
    }

    // Return false to indicate the application should keep running
    false
}

// fn handle_tab_change(state: &mut TabState) {
// }
