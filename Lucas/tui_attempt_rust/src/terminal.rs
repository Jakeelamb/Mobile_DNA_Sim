use crate::widgets::tabstate::TabState;
use crate::widgets::app_widgets::AppWidget;
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
    widgets::Widget,
};

/// Set up the terminal with Crossterm backend
fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?; // Enable raw mode
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Layout setup for the terminal chunks
fn setup_chunks(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Combined height for Tabs and Header
            Constraint::Length(12),  // Header (logo)
            Constraint::Min(15),    // Main content area
            Constraint::Length(3),  // Footer area
        ])
        .split(area)
        .to_vec() // Convert Rc<[Rect]> to Vec<Rect>
}

/// Main application function
pub fn run_app() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let mut tab_state = TabState::new(); // Initialize tab state

    loop {
        terminal.draw(|f| {
            let chunks = setup_chunks(f.area());

            // Render the Tabs
            let tabs = tab_state.render();
            f.render_widget(tabs, chunks[0]);

            // Render the Header
            let header = tab_state.render_header();
            f.render_widget(header, chunks[1]);

            // Render the content area based on the active tab
            let content_blocks = tab_state.render_home_widgets();
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25), 
                    Constraint::Percentage(50), 
                    Constraint::Percentage(25)
                ])
                .split(chunks[2]);

            content_blocks.iter().enumerate().for_each(|(i, block)| {
                if let Some(chunk) = content_chunks.get(i) {
                    match block {
                        AppWidget::SpeciesList(list) => f.render_stateful_widget(list.clone(), *chunk, &mut tab_state.list_state),
                        AppWidget::LogoBlock(logo) => f.render_widget(logo.clone(), *chunk),
                        AppWidget::InfoBlock(info) => f.render_widget(info.clone(), *chunk),
                        AppWidget::SettingsBlock(settings) => f.render_widget(settings.clone(), *chunk),
                        AppWidget::FooterBlock(footer) => f.render_widget(footer.clone(), *chunk),
                    }
                }
            });

            let footer = tab_state.render_footer();
            f.render_widget(footer, chunks[3]);
        })?;

        // Capture key events for scrolling and tab navigation
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => tab_state.next(),
                    KeyCode::Left => tab_state.previous(),
                    KeyCode::Down => tab_state.scroll_down(), // Scroll down in the species list
                    KeyCode::Up => tab_state.scroll_up(),     // Scroll up in the species list
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