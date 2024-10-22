// terminal.rs
use crate::widgets::tabstate::TabState;
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::footer_renderer::render_footer;
use crate::widgets::header_renderer::render_header;
use crate::widgets::key_bindings_renderer::render_key_bindings;
use crate::widgets::input_handler::{next, previous, scroll_up, scroll_down};

use std::{error::Error, io};
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
            Constraint::Min(8),  // Header (logo)
            Constraint::Min(15),    // Main content area
            Constraint::Length(3),  // Footer area
            Constraint::Length(10),  // Key Bindings area
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
            let chunks = setup_chunks(f.area());

            // Render the Tabs
            let tabs = tab_state.render();
            f.render_widget(tabs, chunks[0]);

            // Render the Header
            let header = render_header(&tab_state);
            f.render_widget(header, chunks[1]);

            // // Render the run time
            // if let Some(start_time) = tab_state.start_time {
            //     let run_time = render_run_time(start_time);
            //     f.render_widget(run_time, chunks[0]); // Assuming you're rendering it in a chunk
            // }

            //  Render the content area based on the active tab
            // let content_blocks = tab_state.render_home_widgets();
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20), 
                    Constraint::Percentage(40), 
                    Constraint::Percentage(40)
                ])
                .split(chunks[2]);

            let content_blocks = tab_state.render_content();
            content_blocks.iter().enumerate().for_each(|(i, block)| {
                if let Some(chunk) = content_chunks.get(i) {
                    match block {
                        AppWidget::SpeciesList(list) => f.render_stateful_widget(list.clone(), *chunk, &mut tab_state.list_state),
                        AppWidget::InfoBlock(info) => f.render_widget(info.clone(), *chunk),
                        AppWidget::SettingsBlock(settings) => f.render_widget(settings.clone(), *chunk),
                        // AppWidget::MutationChart(chart) => f.render_widget(chart.clone(), *chunk),
                        // AppWidget::ProbabilityChart(chart) => f.render_widget(chart.clone(), *chunk),
                        // AppWidget::Chart(chart) => f.render_widget(chart.clone(), *chunk),  // Render the chart
                        _ => {}
                    }
                }
            });

            let footer = render_footer();
            f.render_widget(footer, chunks[3]);

            // Render the Key Bindings
            let key_bindings = render_key_bindings();
            f.render_widget(key_bindings, chunks[4]);
        })?;

        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => next(&mut tab_state),  // Call the function from input_handler
                    KeyCode::Left => previous(&mut tab_state),  // Call the function from input_handler
                    KeyCode::Down => scroll_down(&mut tab_state),  // Call the function from input_handler
                    KeyCode::Up => scroll_up(&mut tab_state),  // Call the function from input_handler

                    // Use Later to start simulation, but only want enter to work on simulation page. maybe i can use
                    // point and click here. 

                    // KeyCode::Enter => {
                    //     if tab_state.active_input == 0 { // Assuming active_input == 0 refers to the "Start Simulation" button
                    //         tab_state.start_simulation();  // Trigger the simulation
                    //     }
                    // }

                    // Handle text input for the active field
                    KeyCode::Char(c) => {
                        match tab_state.active_input {
                            0 => tab_state.starting_tes.push(c),  // Edit Starting TEs
                            1 => tab_state.sim_rounds.push(c),    // Edit Simulation Rounds
                            2 => tab_state.cpu_cores.push(c),     // Edit CPU cores
                            3 => tab_state.output_dir.push(c),    // Edit Output Directory
                            _ => {}
                        }
                    }
                    // Remove the last character from the active field
                    KeyCode::Backspace => {
                        match tab_state.active_input {
                            0 => { tab_state.starting_tes.pop(); }
                            1 => { tab_state.sim_rounds.pop(); }
                            2 => { tab_state.cpu_cores.pop(); }
                            3 => { tab_state.output_dir.pop(); }
                            _ => {}
                        }
                    }
                    // Switch to the next field using Tab
                    KeyCode::Tab => {
                        tab_state.active_input = (tab_state.active_input + 1) % 4; // Rotate through the four fields
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
