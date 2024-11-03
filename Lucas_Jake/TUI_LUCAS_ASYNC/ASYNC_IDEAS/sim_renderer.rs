// use crate::widgets::app_widgets::AppWidget;
// use crate::widgets::run_simulation::SimulationParam;
use crate::widgets::tabstate::SpeciesData;
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use std::time::{Instant};
use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;
use std::sync::Arc;

// // Move these functions outside
pub fn render_run_time(run_time: &str) -> Paragraph<'static> {
    Paragraph::new(Span::raw(format!("Run Time: {}", run_time)))
        .block(Block::default().borders(Borders::ALL).title(""))
        .style(Style::default().fg(Color::White).bg(Color::Black))
}

pub fn render_current_round(current_round: usize, total_rounds: usize) -> Paragraph<'static> {
    Paragraph::new(Span::raw(format!(
        "Current Round: {} / {}",
        current_round, total_rounds
    )))
    .block(Block::default().borders(Borders::ALL).title(""))
    .style(Style::default().fg(Color::White).bg(Color::Black))
}

pub fn render_mutation_chart<'a>(tab_state: &'a TabState) -> Chart<'a> {
    let datasets = vec![
        Dataset::default()
            .name("Mutations")
            .style(Style::default().fg(Color::Cyan))
            .data(&tab_state.mutation_data), // Reference the data in TabState
    ];

    Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Number of Mutations"),
        )
        .x_axis(
            Axis::default()
                .title("Simulation Rounds")
                .bounds([0.0, 100.0]),
        )
        .y_axis(Axis::default().title("# of Mutations").bounds([0.0, 100.0]))
        .style(Style::default().fg(Color::White).bg(Color::Black))
}

pub fn render_probability_chart<'a>(tab_state: &'a TabState) -> Chart<'a> {
    let datasets = vec![
        Dataset::default()
            .name("Probability")
            .style(Style::default().fg(Color::Yellow))
            .data(&tab_state.probability_data), // Reference the data in TabState
    ];

    Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Probability of Mutation"),
        )
        .x_axis(
            Axis::default()
                .title("Simulation Rounds")
                .bounds([0.0, 100.0]),
        )
        .y_axis(Axis::default().title("Probability").bounds([0.0, 1.0]))
}


pub fn render_sim_widgets(tab_state: &mut TabState, f: &mut Frame, area: Rect) {
// pub fn render_sim_widgets(tab_state: &Arc<Mutex<TabState>>, f: &mut Frame, area: Rect) {    // Create layout for the main simulation section (info and charts)
    // let state = tab_state.blocking_lock();

    let layout_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left side (Simulation Info)
            Constraint::Percentage(50), // Right side (Graphs)
        ])
        .split(area);

    // Left side layout (for Start Simulation button and Simulation Info)
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // Start Simulation button
            Constraint::Percentage(80), // Simulation Info
        ])
        .split(layout_chunks[0]);

    // Right side layout (for Graphs block and charts)
    let right_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(2),  // Graphs title block
            Constraint::Percentage(49), // Mutation chart
            Constraint::Percentage(49), // Probability chart
        ])
        .split(layout_chunks[1]);

    // Render Start Simulation button
    let start_button = Paragraph::new(Span::styled(
        "Start Simulation",
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Gray).fg(Color::White))
            .border_style(Style::default().fg(Color::Gray))
            .title(""),
    );
    f.render_widget(start_button, left_chunks[0]);

    // Get the selected species for Simulation Info, using filtered list if search mode is active
    let selected_species_list = if tab_state.search_mode && !tab_state.filtered_species.is_empty() {
        &tab_state.filtered_species
    } else {
        &tab_state.species
    };

    const EMPTY_STRING: &String = &String::new();

    let selected_species = selected_species_list
        .get(tab_state.list_state.selected().unwrap_or(0))
        .unwrap_or(EMPTY_STRING);

    let species_info = tab_state
        .species_info
        .get(selected_species)
        .cloned()
        .unwrap_or(SpeciesData::default());

    // Check if the simulation should start and spawn an async task if so
    // let mut tab_state_lock = tab_state.blocking_lock();
    // if !tab_state_lock.simulation_start_triggered {
    //     tab_state_lock.simulation_start_triggered = true;
    // } else {
    //     // If `simulation_start_triggered` is already true, we don't start a new simulation.
    //     return;
    // } // `tab_state_lock` goes out of scope here, releasing the lock

    //Now, spawn the async task after the lock is released
    // tokio::spawn(run_simulation(Arc::clone(&tab_state)));

    // Display updated simulation info
    let info_text = vec![
        Line::from(format!("Rounds Completed: {}", tab_state.rounds_completed)),
        Line::from(format!("Total Mutations: {}", tab_state.total_mutations)),
        Line::from(format!("Run Time: {}", tab_state.run_time)),
        // More simulation data as needed
    ];

    let info_paragraph = Paragraph::new(info_text)
        .block(
            Block::default()
                .title("Simulation Info")
                .borders(Borders::ALL),
        )
        .style(Style::default().bg(Color::Red).fg(Color::White));

    f.render_widget(info_paragraph, left_chunks[1]);

    // Render additional simulation information
    let species_info_text = vec![
        Line::from(vec![Span::raw(format!(
            "Species: {}\n",
            species_info.species
        ))]),
        Line::from(vec![Span::raw(format!(
            "Start Genome Size: {}\n",
            species_info.genome_size
        ))]),
        Line::from(vec![Span::raw(format!(
            "Start Exon Size: {}\n",
            species_info.exon_size
        ))]),
        Line::from(vec![Span::raw(format!(
            "Exon/Genome Ratio: {}\n",
            species_info.exon_ratio
        ))]),
        Line::from(vec![Span::raw(format!(
            "# of Simulation Rounds completed: {}\n",
            tab_state.simulation_rounds_completed
        ))]),
        Line::from(vec![Span::raw(format!(
            "# of TEs mobilized: {}\n",
            tab_state.tes_mobilized
        ))]),
        Line::from(vec![Span::raw(format!(
            "# of Mutations: {}\n",
            tab_state.mutations
        ))]),
        Line::from(vec![Span::raw(format!(
            "Current Genome Size: {}\n",
            tab_state.current_genome_size
        ))]),
        Line::from(vec![Span::raw(format!(
            "Current Exon/Genome Ratio: {}\n",
            tab_state.current_exon_genome_ratio
        ))]),
        Line::from(vec![Span::raw(format!(
            "Current Probability of TE causing Mutation: {}\n",
            tab_state.probability_of_te_mutation
        ))]),
    ];

    let simulation_info = Paragraph::new(species_info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Simulation Info")
            .title_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Color::Red).fg(Color::White)),
    );
    f.render_widget(simulation_info, left_chunks[1]);

    // Render Graphs Title Block
    let settings_block = Paragraph::new("Mutation and Probability Graphs").block(
        Block::default()
            .borders(Borders::ALL)
            .title("Graphs")
            .title_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Color::Gray).fg(Color::White)),
    );
    f.render_widget(settings_block, right_chunks[0]);

    // Render Mutation Chart
    let mutation_chart = render_mutation_chart(&tab_state);
    f.render_widget(mutation_chart, right_chunks[1]);

    // Render Probability Chart
    let probability_chart = render_probability_chart(&tab_state);
    f.render_widget(probability_chart, right_chunks[2]);
}


pub async fn run_simulation(tab_state: Arc<Mutex<TabState>>) {
    {
        let mut state = tab_state.lock().await;
        state.start_time = Some(Instant::now());
    }

    for _ in 0..tab_state.lock().await.simulation_rounds {
        // Simulate a round with a delay
        sleep(Duration::from_millis(100)).await;

        let mut state = tab_state.lock().await;
        state.rounds_completed += 1;

        // Update runtime in real-time
        if let Some(start_time) = state.start_time {
            let elapsed = start_time.elapsed();
            state.run_time = format!(
                "{:02}:{:02}:{:02}",
                elapsed.as_secs() / 3600,
                (elapsed.as_secs() % 3600) / 60,
                elapsed.as_secs() % 60
            );
        }
    }

    let mut state = tab_state.lock().await;
    state.simulation_start_triggered = false;
    // tab_state.lock().await.simulation_start_triggered = false;
}