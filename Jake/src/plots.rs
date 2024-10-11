use std::error::Error;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
pub struct Plotter {
    pub rounds: usize,
    pub max_mutations: i32,
    pub mutation_data: Vec<(f64, f64)>,
}

impl Plotter {
    pub fn new(num_rounds: usize, initial_max_mutations: i32) -> Self {
        Plotter {
            rounds: num_rounds,
            max_mutations: initial_max_mutations,
            mutation_data: Vec::new(),
        }
    }

    pub fn update(&mut self, new_data: (i32, i32)) {
        self.mutation_data.push((new_data.0 as f64, new_data.1 as f64));
        if new_data.1 > self.max_mutations {
            self.max_mutations = new_data.1;
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the main loop
        let res = self.run_app(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }

    fn run_app<B: ratatui::backend::Backend>(&self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|f| {
                let area = f.area();
                let chart = Chart::new(vec![Dataset::default()
                    .name("Mutations")
                    .marker(symbols::Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&self.mutation_data)])
                .block(Block::default().title("Mutation Count vs Round").borders(Borders::ALL))
                .x_axis(Axis::default()
                    .title("Round Number")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, self.rounds as f64])
                    .labels(vec![
                        Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(self.rounds.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                    ]))
                .y_axis(Axis::default()
                    .title("Mutations")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, self.max_mutations as f64])
                    .labels(vec![
                        Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                        Span::styled(self.max_mutations.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                    ]));
                f.render_widget(chart, area);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}