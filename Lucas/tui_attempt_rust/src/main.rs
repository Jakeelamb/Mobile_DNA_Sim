use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use std::io;

struct SpeciesData {
    species: String,
    genome_size: usize,
}

fn main() -> Result<(), io::Error> {
    // Enable raw mode to capture input
    enable_raw_mode()?;

    // Set up terminal backend
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    // Example species data
    let species_data = SpeciesData {
        species: "Accipiter nisus".to_string(),
        genome_size: 1190649881,
    };

    // Draw the user interface once
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let species_info = format!(
            "Species: {}\nGenome Size: {}",
            species_data.species, species_data.genome_size
        );
        let block = Block::default().borders(Borders::ALL).title("Genome Info");
        let paragraph = Paragraph::new(species_info).block(block);

        f.render_widget(paragraph, chunks[0]);
    })?;

    // Event loop for handling keypresses
    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break; // Exit the loop on 'q' key
                }
            }
        }
    }

    // Clean up
    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}