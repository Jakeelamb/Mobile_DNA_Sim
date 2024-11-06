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
                // Simulation tab: 50% for tabs, 50% for Run Time(progress bar) and Current Round
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50), // Tabs section
                        Constraint::Percentage(50), // Info (Run Time + Current Round) section
                    ])
                    .split(chunks[0])
            };

            // Render the Tabs
            let tabs = tab_state.render();
            f.render_widget(tabs, tab_and_info_chunks[0]);

            // Render the Header (logo, etc.)
            let header = render_header(&tab_state);
            f.render_widget(header, chunks[1]);

            if tab_state.index == 1 {
                let info_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50), // Progress Bar block
                        Constraint::Percentage(50), // Current Round block
                    ])
                    .split(tab_and_info_chunks[1]);

                // Render the Progress Bar
                let progress_paragraph = Paragraph::new(Span::styled(
                    &tab_state.progress_bar,
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
                    render_current_round(tab_state.current_round, tab_state.simulation_rounds);
                f.render_widget(current_round_block, info_chunks[1]);

                // // Update the simulation
                // // take 2
                // if tab_state.simulation_start_triggered {
                //     if tab_state.has_more_rounds() {
                //         // Run one round of the simulation and update the UI
                //         tab_state.run_simulation_round();
                //         tab_state.update_simulation();  // This updates the progress bar, round count, etc.
                //     } else {
                //         // Once all rounds are completed, stop the simulation
                //         tab_state.simulation_start_triggered = false;

                //         // Export to CSV after the simulation finishes
                //         if let Err(e) = tab_state.export_to_csv() {
                //             eprintln!("Failed to export to CSV: {:?}", e);
                //         }
                //     }
                // }
                //take 1
                // if tab_state.simulation_start_triggered {
                //     tab_state.update_simulation();

                //     // Check if the simulation has completed all rounds
                //     if !tab_state.has_more_rounds() {
                //         tab_state.simulation_start_triggered = false; // Stop the simulation

                //         // Export to CSV once simulation completes
                //         if let Err(e) = tab_state.export_to_csv() {
                //             eprintln!("Failed to export to CSV: {:?}", e);
                //         }
                //     }
                // }
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
                                f.render_stateful_widget(
                                    list.clone(),
                                    *chunk,
                                    &mut tab_state.list_state,
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

        // Check if the simulation should be running
        // Update the simulation
        // take 2
        if tab_state.simulation_start_triggered {
            if tab_state.has_more_rounds() {
                // Run one round of the simulation and update the UI
                tab_state.run_simulation_round();
                tab_state.update_simulation(); // This updates the progress bar, round count, etc.
            } else {
                // Once all rounds are completed, stop the simulation
                tab_state.simulation_start_triggered = false;

                // Export to CSV after the simulation finishes
                if let Err(e) = tab_state.export_to_csv() {
                    eprintln!("Failed to export to CSV: {:?}", e);
                }
            }
        }
        // if tab_state.simulation_start_triggered {
        //     if tab_state.has_more_rounds() {
        //         tab_state.run_simulation_round();
        //         tab_state.update_simulation();
        //     } else {
        //         tab_state.simulation_start_triggered = false; // Stop the simulation when rounds complete
        //     }
        // }

        // Add a delay to slow down the simulation updates
        // thread::sleep(Duration::from_millis(100));

        // Handle user input events
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break, // Quit the application
                    // Navigate between tabs
                    KeyCode::Right => {
                        next(&mut tab_state);
                        if tab_state.index == 1 {
                            // Only exit search mode if desired, but don't clear the search results
                            // tab_state.search_mode = false;

                            // Set `current_species` to the selected species based on the filtered list
                            if let Some(selected_species) = tab_state
                                .filtered_species
                                .get(tab_state.list_state.selected().unwrap_or(0))
                            {
                                tab_state.current_species = selected_species.clone();
                            }
                        }
                    }
                    KeyCode::Left => {
                        previous(&mut tab_state);
                        if tab_state.index == 1 {
                            // Only exit search mode, leave results.
                            // tab_state.search_mode = false;

                            // Set `current_species` to the selected species based on the filtered list
                            if let Some(selected_species) = tab_state
                                .filtered_species
                                .get(tab_state.list_state.selected().unwrap_or(0))
                            {
                                tab_state.current_species = selected_species.clone();
                            }
                        }
                    }

                    KeyCode::Down => {
                        // only allow scrolling if on the Home page (index == 0)
                        if tab_state.index == 0 {
                            if tab_state.search_mode && !tab_state.filtered_species.is_empty() {
                                scroll_down_with_list(
                                    &mut tab_state.list_state,
                                    &tab_state.filtered_species,
                                );
                            } else {
                                scroll_down(&mut tab_state);
                            }
                        }
                    }
                    KeyCode::Up => {
                        // only allow scrolling if on the Home page (index == 0)
                        if tab_state.index == 0 {
                            if tab_state.search_mode && !tab_state.filtered_species.is_empty() {
                                scroll_up_with_list(
                                    &mut tab_state.list_state,
                                    &tab_state.filtered_species,
                                );
                            } else {
                                scroll_up(&mut tab_state);
                            }
                        }
                    }

                    // Handle entering search mode
                    KeyCode::Char('/') => {
                        // Enter search mode only if on the Home page
                        if tab_state.index == 0 {
                            tab_state.search_mode = true;
                            tab_state.search_query.clear();
                            tab_state.list_state.select(Some(0));
                        }
                    }

                    // Handle regular character input
                    KeyCode::Char(c) => {
                        if tab_state.index == 1
                            && c == 's'
                            && (!tab_state.search_mode || tab_state.search_mode)
                        {
                            // Start the simulation on the Simulation page if 's' is pressed
                            if !tab_state.simulation_start_triggered {
                                tab_state.start_simulation(); // Set start time and reset rounds
                                tab_state.simulation_start_triggered = true;
                            }
                        } else if tab_state.index == 0 {
                            // Handle search mode and settings input only on the Home page
                            if tab_state.search_mode {
                                // Add the character to the search query in search mode
                                tab_state.search_query.push(c);
                                tab_state.filtered_species = tab_state
                                    .species
                                    .iter()
                                    .filter(|name| {
                                        name.to_lowercase()
                                            .contains(&tab_state.search_query.to_lowercase())
                                    })
                                    .cloned()
                                    .collect();

                                // Keep the current selection within the filtered list
                                tab_state.list_state.select(Some(0));
                            } else {
                                // Handle regular input for simulation settings if not in search mode
                                match tab_state.active_input {
                                    0 => tab_state.starting_tes.push(c),      // Edit Starting TEs
                                    1 => tab_state.sim_rounds.push(c), // Edit Simulation Rounds
                                    2 => tab_state.cpu_cores.push(c),  // Edit CPU cores
                                    3 => tab_state.sim_mobility_prob.push(c), // Edit Probability of TE Mutation
                                    4 => tab_state.output_dir.push(c), // Edit Output Directory

                                    _ => {}
                                }
                            }
                        }
                    }

                    // Handle backspace
                    KeyCode::Backspace => {
                        if tab_state.search_mode {
                            tab_state.search_query.pop();
                            tab_state.filtered_species = tab_state
                                .species
                                .iter()
                                .filter(|name| {
                                    name.to_lowercase()
                                        .contains(&tab_state.search_query.to_lowercase())
                                })
                                .cloned()
                                .collect();
                        } else if tab_state.index == 0 {
                            // Handle backspace for simulation settings input (if on the Home page)
                            match tab_state.active_input {
                                0 => {
                                    tab_state.starting_tes.pop();
                                }
                                1 => {
                                    tab_state.sim_rounds.pop();
                                }
                                2 => {
                                    tab_state.cpu_cores.pop();
                                }
                                3 => {
                                    tab_state.sim_mobility_prob.pop();
                                }
                                4 => {
                                    tab_state.output_dir.pop();
                                }
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

                    // Cycle through simulation settings inputs
                    KeyCode::Tab => {
                        if tab_state.index == 0 {
                            // Cycle through indices 1 to 4
                            tab_state.active_input += 1;
                            if tab_state.active_input > 4 {
                                tab_state.active_input = 0; // Wrap around to the first editable field
                            }
                        }
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
