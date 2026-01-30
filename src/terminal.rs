use crate::widgets::app_widgets::AppWidget;
use crate::widgets::footer_renderer::render_footer;
use crate::widgets::header_renderer::render_header;
use crate::widgets::home_renderer::render_home_widgets;
use crate::widgets::input_handler::{next, previous, scroll_down, scroll_up};
use crate::widgets::key_bindings_renderer::render_key_bindings;
use crate::widgets::sim_renderer::render_sim_widgets;
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, ListState};
use ratatui::Frame;

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
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use std::{error::Error, io};

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
        .to_vec()
}

pub async fn run_app() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let tab_state = Arc::new(Mutex::new(TabState::new()));

    // Spawn simulation thread
    let simulation_tab_state = Arc::clone(&tab_state);
    tokio::spawn(async move {
        loop {
            if let Ok(mut state) = simulation_tab_state.try_lock() {
                if state.simulation_start_triggered && state.has_more_rounds() {
                    state.run_simulation_round();
                    state.update_simulation();
                    
                    if !state.has_more_rounds() {
                        state.simulation_start_triggered = false;
                        if let Err(e) = state.export_results() {
                            eprintln!("Failed to export results: {}", e);
                        }
                    }
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
    });

    loop {
        terminal.draw(|f| {
            let chunks = setup_chunks(f.area());
            let mut state = tab_state.lock().unwrap();
            render_ui(&mut state, f, chunks);
        })?;

        // Use poll with timeout to allow UI updates without blocking
        if event::poll(Duration::from_millis(50))? {
            if let Ok(event) = event::read() {
                let mut state = tab_state.lock().unwrap();
                if handle_event(&mut state, event) {
                    break;
                }
            }
        }
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

fn scroll_down_with_list(state: &mut ListState, items: &[String]) {
    if items.is_empty() {
        return;
    }
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

fn scroll_up_with_list(state: &mut ListState, items: &[String]) {
    if items.is_empty() {
        return;
    }
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
    let tab_and_info_chunks = if state.index == 0 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(chunks[0])
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[0])
    };

    let tabs = state.render();
    f.render_widget(tabs, tab_and_info_chunks[0]);

    let header = render_header(state);
    f.render_widget(header, chunks[1]);

    if state.index == 1 {
        let info_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(tab_and_info_chunks[1]);

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

        let current_round = Paragraph::new(Span::raw(format!(
            "Round: {}/{}",
            state.current_round,
            state.simulation_rounds
        )))
        .block(Block::default().borders(Borders::ALL).title(""));
        f.render_widget(current_round, info_chunks[1]);
    }

    if state.index == 0 {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(32),
                Constraint::Percentage(36),
                Constraint::Percentage(32),
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
                }
            }
        });
    } else {
        render_sim_widgets(state, f, chunks[2]);
    }

    let footer = render_footer();
    f.render_widget(footer, chunks[3]);

    let key_bindings = render_key_bindings();
    f.render_widget(key_bindings, chunks[4]);
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn handle_event(state: &mut TabState, event: Event) -> bool {
    if let Event::Key(key) = event { match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Right => {
            next(state);
            let selected_species = state
                .filtered_species
                .get(state.list_state.selected().unwrap_or(0))
                .cloned();

            if let Some(selected_species) = selected_species {
                if selected_species != state.current_species {
                    state.current_species = selected_species;
                }
            }
        }
        KeyCode::Left => {
            previous(state);
            let selected_species = state
                .filtered_species
                .get(state.list_state.selected().unwrap_or(0))
                .cloned();

            if let Some(selected_species) = selected_species {
                if selected_species != state.current_species {
                    state.current_species = selected_species;
                }
            }
        }
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
        KeyCode::Char('/') => {
            if state.index == 0 {
                state.search_mode = true;
                state.search_query.clear();
                state.list_state.select(Some(0));
            }
        }
        KeyCode::Char(c) => {
            if state.index == 1 && c == 's' {
                if !state.simulation_start_triggered {
                    state.start_simulation();
                    state.simulation_start_triggered = true;
                }
            } else if state.index == 0 {
                if state.search_mode {
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
                    state.list_state.select(Some(0));
                } else if c == 's' {
                } else {
                    match state.active_input {
                        0 => state.starting_tes.push(c),
                        1 => state.sim_rounds.push(c),
                        2 => state.cpu_cores.push(c),
                        3 => state.sim_mobility_prob.push(c),
                        4 => state.output_dir.push(c),
                        _ => (),
                    }
                }
            }
        }
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
        KeyCode::Tab => {
            if state.index == 0 {
                state.active_input = (state.active_input + 1) % 5;
            }
        }
        _ => {}
    } }
    false
}