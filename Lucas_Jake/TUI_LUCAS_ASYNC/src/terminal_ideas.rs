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
use ratatui::Frame;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend, Backend},
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
    let simulation_tab_state = Arc::clone(&tab_state);
    thread::spawn(move || loop {
        // thread::sleep(std::time::Duration::from_millis(3));
        let mut state = simulation_tab_state.lock().unwrap();
        if state.simulation_start_triggered {
            if state.has_more_rounds() {
                state.run_simulation_round();
                // thread::sleep(Duration::from_millis(10));
                state.update_simulation();
            } else {
                state.simulation_start_triggered = false;
                if let Err(e) = state.export_to_csv() {
                    eprintln!("Failed to export to CSV: {:?}", e);
                }
            }
        }
    });

    loop {
        // terminal.draw(|f| {
            // Set up the general layout
        //     let chunks = setup_chunks(f.area());
        //     let mut state = tab_state.lock().unwrap();
        //     render_ui(&mut state, f, chunks);
        // })?;
            // Conditionally set layout based on the active tab
        //     let tab_and_info_chunks = if state.index == 0 {
        //         // Home tab: 100% width for tabs
        //         Layout::default()
        //             .direction(Direction::Horizontal)
        //             .constraints([Constraint::Percentage(100)]) // 100% for tabs, no right-side info
        //             .split(chunks[0])
        //     } else {
        //         // Simulation tab: 50% for tabs, 50% for Run Time(progress bar) and Current Round
        //         Layout::default()
        //             .direction(Direction::Horizontal)
        //             .constraints([
        //                 Constraint::Percentage(50), // Tabs section
        //                 Constraint::Percentage(50), // Info (Run Time + Current Round) section
        //             ])
        //             .split(chunks[0])
        //     };

        //     // Render the Tabs
        //     let tabs = state.render();
        //     f.render_widget(tabs, tab_and_info_chunks[0]);

        //     // Render the Header (logo, etc.)
        //     let header = render_header(&state);
        //     f.render_widget(header, chunks[1]);

        //     if state.index == 1 {
        //         let info_chunks = Layout::default()
        //             .direction(Direction::Horizontal)
        //             .constraints([
        //                 Constraint::Percentage(50), // Progress Bar block
        //                 Constraint::Percentage(50), // Current Round block
        //             ])
        //             .split(tab_and_info_chunks[1]);

        //         // Render the Progress Bar
        //         let progress_paragraph = Paragraph::new(Span::styled(
        //             &state.progress_bar,
        //             Style::default().fg(Color::Green),
        //         ))
        //         .block(
        //             Block::default()
        //                 .borders(Borders::ALL)
        //                 .title("Simulation Progress"),
        //         );
        //         f.render_widget(progress_paragraph, info_chunks[0]);

        //         // Render the Current Round in the second half of info_chunks
        //         let current_round_block =
        //             render_current_round(state.current_round, state.simulation_rounds);
        //         f.render_widget(current_round_block, info_chunks[1]);
        //     }

        //     // Render content for the Home tab
        //     if state.index == 0 {
        //         // Split the main content area for Home tab into three chunks (SpeciesList, InfoBlock, SettingsBlock)
        //         let content_chunks = Layout::default()
        //             .direction(Direction::Horizontal)
        //             .constraints([
        //                 Constraint::Percentage(32), // SpeciesList
        //                 Constraint::Percentage(36), // InfoBlock
        //                 Constraint::Percentage(32), // SettingsBlock
        //             ])
        //             .split(chunks[2]);

        //         // Retrieve the widgets for the Home tab content (SpeciesList, InfoBlock, SettingsBlock)
        //         let content_blocks = render_home_widgets(&mut state);

        //         // Render each block in the corresponding chunk
        //         content_blocks.iter().enumerate().for_each(|(i, block)| {
        //             if let Some(chunk) = content_chunks.get(i) {
        //                 match block {
        //                     AppWidget::SpeciesList(list) => {
        //                         f.render_stateful_widget(
        //                             list.clone(),
        //                             *chunk,
        //                             &mut state.list_state,
        //                         );
        //                     }
        //                     AppWidget::InfoBlock(info) => {
        //                         f.render_widget(info.clone(), *chunk);
        //                     }
        //                     AppWidget::SettingsBlock(settings) => {
        //                         f.render_widget(settings.clone(), *chunk);
        //                     }
        //                     _ => {}
        //                 }
        //             }
        //         });
        //     } else {
        //         // Render all widgets for the Simulation tab
        //         render_sim_widgets(&state, f, chunks[2]);
        //     }

        //     // Render the Footer
        //     let footer = render_footer();
        //     f.render_widget(footer, chunks[3]);

        //     // Render the Key Bindings
        //     let key_bindings = render_key_bindings();
        //     f.render_widget(key_bindings, chunks[4]);
        // })?;

        // Check if the simulation should be running
        // Update the simulation
        // take 2
        let mut state = tab_state.lock().unwrap(); // Lock the state for simulation logic

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = crossterm::event::read()? {
        // if let Ok(event) = event::read() {
        //     match event {
                // Event::Key(key) => match key.code {
                match key.code {
                    KeyCode::Char('q') => break, // Quit the application
                    // Navigate between tabs
                    KeyCode::Right => {
                        next(&mut state);
                        let selected_species = state
                            .filtered_species
                            .get(state.list_state.selected().unwrap_or(0))
                            .cloned();

                        if let Some(selected_species) = selected_species {
                            if selected_species != state.current_species {
                                // Reset simulation results only if a new species is selected
                                if state.simulation_rounds_completed > 0 {
                                    state.needs_redraw = true;
                                    state.reset_simulation_results();
                                }
                                state.current_species = selected_species;
                            }
                        }

                        // If switching to the Simulation tab, ensure fields are properly reset
                        if state.index == 1 {
                            // No need to re-check `selected_species` since it was handled above
                            if state.simulation_rounds_completed > 0 {
                                state.needs_redraw = true;
                                state.reset_simulation_results();
                            }
                        }
                    }

                    // KeyCode::Right => {
                    //     next(&mut state);
                    //     if state.index == 1 {
                    //         // Set `current_species` to the selected species based on the filtered list
                    //         if let Some(selected_species) = state
                    //             .filtered_species
                    //             .get(state.list_state.selected().unwrap_or(0))
                    //         {
                    //             // Check if the selected species is different and if the simulation has run before
                    //             if *selected_species != state.current_species
                    //                 && state.simulation_rounds_completed > 0
                    //             {
                    //                 // Reset simulation results as a new species is selected and the simulation was run before
                    //                 state.reset_simulation_results();
                    //             }
                    //             state.current_species = selected_species.clone();
                    //         }
                    //     }
                    // }
                    KeyCode::Left => {
                        previous(&mut state);
                        let selected_species = state
                            .filtered_species
                            .get(state.list_state.selected().unwrap_or(0))
                            .cloned();

                        if let Some(selected_species) = selected_species {
                            if selected_species != state.current_species {
                                // Reset simulation results only if a new species is selected
                                if state.simulation_rounds_completed > 0 {
                                    state.needs_redraw = true;
                                    state.reset_simulation_results();
                                }
                                state.current_species = selected_species;
                            }
                        }

                        // If switching to the Simulation tab, ensure fields are properly reset
                        if state.index == 1 {
                            // No need to re-check `selected_species` since it was handled above
                            if state.simulation_rounds_completed > 0 {
                                state.needs_redraw = true;
                                state.reset_simulation_results();
                            }
                        }
                    }
                    // KeyCode::Left => {
                    //     previous(&mut state);
                    //     if state.index == 1 {
                    //         if let Some(selected_species) = state
                    //             .filtered_species
                    //             .get(state.list_state.selected().unwrap_or(0))
                    //         {
                    //             // Check if the selected species is different and if the simulation has run before
                    //             if *selected_species != state.current_species
                    //                 && state.simulation_rounds_completed > 0
                    //             {
                    //                 // Reset simulation results as a new species is selected and the simulation was run before
                    //                 state.reset_simulation_results();
                    //             }
                    //             state.current_species = selected_species.clone();
                    //         }
                    //     }
                    // }
                    KeyCode::Down => {
                        if state.index == 0 {
                            if state.search_mode && !state.filtered_species.is_empty() {
                                let filtered_species = state.filtered_species.clone();
                                scroll_down_with_list(&mut state.list_state, &filtered_species);
                            } else {
                                scroll_down(&mut state);
                            }
                        }
                    }
                    KeyCode::Up => {
                        if state.index == 0 {
                            if state.search_mode && !state.filtered_species.is_empty() {
                                let filtered_species = state.filtered_species.clone();
                                scroll_up_with_list(&mut state.list_state, &filtered_species);
                            } else {
                                scroll_up(&mut state);
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
                    KeyCode::Char(c) => {
                        if state.index == 1 && c == 's' && (!state.search_mode || state.search_mode)
                        {
                            // Start the simulation on the Simulation page if 's' is pressed
                            if !state.simulation_start_triggered {
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
                            } else {
                                // Handle regular input for simulation settings if not in search mode
                                match state.active_input {
                                    0 => state.starting_tes.push(c),      // Edit Starting TEs
                                    1 => state.sim_rounds.push(c),        // Edit Simulation Rounds
                                    2 => state.cpu_cores.push(c),         // Edit CPU cores
                                    3 => state.sim_mobility_prob.push(c), // Edit Probability of TE Mutation
                                    4 => state.output_dir.push(c),        // Edit Output Directory

                                    _ => {}
                                }
                            }
                        }
                    }

                    // Handle backspace
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

                    KeyCode::Esc => {
                        state.search_mode = false;
                        state.search_query.clear();
                        state.filtered_species.clear();
                        state.list_state.select(Some(0));
                    }

                    // Cycle through simulation settings inputs
                    KeyCode::Tab => {
                        if state.index == 0 {
                            state.active_input += 1;
                            if state.active_input > 4 {
                                state.active_input = 0;
                            }
                        }
                    }

                    // Check if the simulation should be running and update the simulation
                    if state.simulation_start_triggered {
                        if state.has_more_rounds() {
                            state.run_simulation_round();
                            state.update_simulation();
                            state.needs_redraw = true; // Mark for redraw after simulation update
                        } else {
                            state.simulation_start_triggered = false;
                                if let Err(e) = state.export_to_csv() {
                                    eprintln!("Failed to export to CSV: {:?}", e);
                                }
                            state.needs_redraw = true; // Redraw to update UI after simulation ends
                        }
                    }

                    // Render the UI only if needed
                    if state.needs_redraw {
                        // It's important to drop the lock before calling `terminal.draw`
                        drop(state); // Unlock before rendering

                        terminal.draw(|f| {
                            let chunks = setup_chunks(f.area());
                            let mut state = tab_state.lock().unwrap(); // Lock the state again for rendering
                            render_ui(&mut state, f, chunks);
                        })?;

                        // Lock the state again to reset the flag
                        let mut state = tab_state.lock().unwrap();
                        state.needs_redraw = false;
                    }

                    _ => {}
                },
                _ => {}
            }
        }
    }

    // Restore terminal state on exit
    //   restore_terminal(&mut terminal)?;
    //   Ok(())

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

fn render_ui(
    state: &mut TabState,
    f: &mut Frame,
    chunks: Vec<ratatui::layout::Rect>,
) {
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
                        f.render_stateful_widget(
                            list.clone(),
                            *chunk,
                            &mut state.list_state,
                        );
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