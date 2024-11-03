// // terminal.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::footer_renderer::render_footer;
use crate::widgets::header_renderer::render_header;
use crate::widgets::home_renderer::render_home_widgets;
use crate::widgets::input_handler::{next, previous, scroll_down, scroll_up};
use crate::widgets::key_bindings_renderer::render_key_bindings;
use crate::widgets::sim_renderer::{render_current_round, render_run_time}; //, render_mutation_chart, render_probability_chart};
use crate::widgets::sim_renderer::{render_sim_widgets, run_simulation};
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph, ListState};
use tokio::sync::Mutex;
use ratatui::Frame;
use futures::executor;

use std::{sync::Arc, error::Error};
use tokio::time::sleep;
use std::time::Duration;


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
use std::{io};

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

/// Run the terminal application
// #[tokio::main]
pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let tab_state = Arc::new(Mutex::new(TabState::new()));

    loop {
        let tab_state_clone = Arc::clone(&tab_state);

        // Attempt to acquire the lock without blocking
        if let Ok(mut state) = tab_state_clone.try_lock() {
            // Perform rendering within `draw`
            terminal.draw(|f| {
                render_ui(f, &mut *state);
            })?;
        }

        // Handle user input events
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => {
                        let tab_state_clone = Arc::clone(&tab_state);
                        let mut state = tab_state_clone.lock().await;
                        if !state.simulation_start_triggered {
                            state.simulation_start_triggered = true;
                            tokio::spawn(run_simulation(Arc::clone(&tab_state)));
                        }
                    }
                    KeyCode::Down => {
                        let mut state = tab_state.lock().await;
                        if state.search_mode && !state.filtered_species.is_empty() {
                            let filtered_species = state.filtered_species.clone();
                            scroll_down_with_list(&mut state.list_state, &filtered_species);
                        } else {
                            scroll_down(&mut state);
                        }
                    }
                    KeyCode::Up => {
                        let mut state = tab_state.lock().await;
                        if state.search_mode && !state.filtered_species.is_empty() {
                            let filtered_species = state.filtered_species.clone();
                            scroll_up_with_list(&mut state.list_state, &filtered_species);
                        } else {
                            scroll_up(&mut state);
                        }
                    }
                    KeyCode::Char('/') => {
                        let mut state = tab_state.lock().await;
                        state.search_mode = true;
                        state.search_query.clear();
                        state.list_state.select(Some(0));
                    }
                    KeyCode::Char(c) => {
                        let mut state = tab_state.lock().await;
                        if state.search_mode {
                            state.search_query.push(c);
                            state.filtered_species = state.species
                                .iter()
                                .filter(|name| name.to_lowercase().contains(&state.search_query.to_lowercase()))
                                .cloned()
                                .collect();
                            state.list_state.select(Some(0));
                        } else {
                            match state.active_input {
                                0 => state.starting_tes.push(c),
                                1 => state.sim_rounds.push(c),
                                2 => state.cpu_cores.push(c),
                                3 => state.output_dir.push(c),
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        let mut state = tab_state.lock().await;
                        if state.search_mode {
                            state.search_query.pop();
                            state.filtered_species = state.species
                                .iter()
                                .filter(|name| name.to_lowercase().contains(&state.search_query.to_lowercase()))
                                .cloned()
                                .collect();
                        } else {
                            match state.active_input {
                                0 => { state.starting_tes.pop(); }
                                1 => { state.sim_rounds.pop(); }
                                2 => { state.cpu_cores.pop(); }
                                3 => { state.output_dir.pop(); }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Esc => {
                        let mut state = tab_state.lock().await;
                        state.search_mode = false;
                        state.search_query.clear();
                        state.filtered_species.clear();
                        state.list_state.select(Some(0));
                    }
                    KeyCode::Tab => {
                        let mut state = tab_state.lock().await;
                        state.active_input = (state.active_input + 1) % 4;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Delay to allow async tasks to process in the background
        sleep(Duration::from_millis(50)).await;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
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

// `render_ui` function 
// `run_simulation` function and other helper functions go here (it is in sim_renderer now.)


/// Render the UI
pub fn render_ui(f: &mut Frame, tab_state: &mut TabState) {
// pub fn render_ui(f: &mut Frame, tab_state: &Arc<Mutex<TabState>>) {
    // let mut state = tab_state.blocking_lock(); //.expect("Failed to lock TabState");l
    // let mut state = tab_state.blocking_lock(); //.expect("Failed to lock TabState");

    let chunks = setup_chunks(f.area());

    let tab_and_info_chunks = if tab_state.index == 0 {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(chunks[0])
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(chunks[0])
    };

    let tabs = tab_state.render();
    f.render_widget(tabs, tab_and_info_chunks[0]);

    let header = render_header(tab_state); // Pass a reference to the locked state
    f.render_widget(header, chunks[1]);

    if tab_state.index == 1 {
        let info_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(tab_and_info_chunks[1]);

        let run_time_block = render_run_time(&tab_state.run_time);
        f.render_widget(run_time_block, info_chunks[0]);

        let current_round_block = render_current_round(tab_state.current_round, 10000);
        f.render_widget(current_round_block, info_chunks[1]);
    }

    if tab_state.index == 0 {
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(32), Constraint::Percentage(36), Constraint::Percentage(32)])
            .split(chunks[2]);

        let content_blocks = render_home_widgets(tab_state);

        content_blocks.iter().enumerate().for_each(|(i, block)| {
            if let Some(chunk) = content_chunks.get(i) {
                match block {
                    AppWidget::SpeciesList(list) => f.render_stateful_widget(list.clone(), *chunk, &mut tab_state.list_state),
                    AppWidget::InfoBlock(info) => f.render_widget(info.clone(), *chunk),
                    AppWidget::SettingsBlock(settings) => f.render_widget(settings.clone(), *chunk),
                    _ => {}
                }
            }
        });
    } else {
        render_sim_widgets(tab_state, f, chunks[2]);
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