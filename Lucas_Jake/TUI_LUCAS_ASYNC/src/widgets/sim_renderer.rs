// use crate::widgets::app_widgets::AppWidget;
use crate::widgets::tabstate::SpeciesData;
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

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

pub fn render_sim_widgets(tab_state: &TabState, f: &mut Frame, area: Rect) {
    // Create layout for the main simulation section (info and charts)
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

    // Determine the button style based on whether the simulation has started
    let start_button_style = if tab_state.simulation_start_triggered {
        Style::default()
            .fg(Color::Black) // Change the text color to indicate it's pressed
            .bg(Color::LightGreen) // Change the background color
            .add_modifier(Modifier::BOLD | Modifier::ITALIC) // Add a modifier for emphasis
    } else {
        Style::default()
            .fg(Color::White)
            .bg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    };

    // Render Start Simulation button
    let start_button = Paragraph::new(Span::styled(
        if tab_state.simulation_start_triggered {
            "Simulation Running" // Change the text to indicate it's running
        } else {
            "Start Simulation"
        },
        start_button_style,
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

    const EMPTY_STRING: &String = &String::new(); // Static reference to an empty string slic

    let selected_species = selected_species_list
        .get(tab_state.list_state.selected().unwrap_or(0))
        .unwrap_or(EMPTY_STRING); // Use the static empty `String` referenc

    let species_info = tab_state
        .species_info
        .get(selected_species)
        .cloned()
        .unwrap_or(SpeciesData::default());

    fn format_percentage(ratio: f64) -> String {
        format!("{:.2}%", ratio * 100.0)
    }
    
    let curr_ratio_formatted = format_percentage(
        tab_state.current_exon_genome_ratio.parse::<f64>().unwrap_or(0.0)
    );

    // Render Simulation Info block
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
            tab_state.current_round
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
            curr_ratio_formatted
        ))]),
        Line::from(vec![Span::raw(format!(
            "Probability of TE causing Mutation: {}\n",
            tab_state.sim_mobility_prob
        ))]),
    ];

    // Progress BAR
    // Calculate the progress based on current round and total rounds
    let progress = (tab_state.current_round as f64 / tab_state.simulation_rounds as f64).min(1.0);
    let bar_length = (progress * 20.0).round() as usize; // 20-character bar length

    // Generate the progress bar as a string
    let progress_bar = format!(
        "[{}{}] {:.0}%",
        "=".repeat(bar_length),      // Filled part
        " ".repeat(20 - bar_length), // Empty part
        progress * 100.0
    );

    // Render Progress Bar
    let progress_paragraph = Paragraph::new(Span::styled(
        progress_bar,
        Style::default().fg(Color::Green).bg(Color::Black),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Simulation Progress"),
    );
    f.render_widget(progress_paragraph, left_chunks[1]);

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
    let mutation_chart = render_mutation_chart(tab_state);
    f.render_widget(mutation_chart, right_chunks[1]);

    // Render Probability Chart
    let probability_chart = render_probability_chart(tab_state);
    f.render_widget(probability_chart, right_chunks[2]);
}

// RUN SIMULATION
// pub async fn run_simulation(tab_state: Arc<Mutex<TabState>>) {
//     {
//         let mut state = tab_state.lock().await;
//         state.start_time = Some(Instant::now());
//     }

//     for _ in 0..tab_state.lock().await.simulation_rounds {
//         // Simulate a round with a delay
//         sleep(Duration::from_millis(100)).await;

//         let mut state = tab_state.lock().await;
//         state.rounds_completed += 1;

//         // Update runtime in real-time
//         if let Some(start_time) = state.start_time {
//             let elapsed = start_time.elapsed();
//             state.run_time = format!(
//                 "{:02}:{:02}:{:02}",
//                 elapsed.as_secs() / 3600,
//                 (elapsed.as_secs() % 3600) / 60,
//                 elapsed.as_secs() % 60
//             );
//         }
//     }

//     let mut state = tab_state.lock().await;
//     state.simulation_start_triggered = false;
//     // tab_state.lock().await.simulation_start_triggered = false;
// }
