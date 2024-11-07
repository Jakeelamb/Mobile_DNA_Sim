// use crate::widgets::app_widgets::AppWidget;
use crate::widgets::tabstate::SpeciesData;
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Modifier, Style};
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph};
use ratatui::widgets::{BarChart, GraphType};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use std::{thread, time::Duration};

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
            // thread::sleep(Duration::from_millis(30));
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
        tab_state
            .current_exon_genome_ratio
            .parse::<f64>()
            .unwrap_or(0.0),
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
            // tab_state.mutations
            tab_state.te_in_exons
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

    // CHARTS:
    // Render Mutation Chart
    // let mutation_chart = render_mutation_chart(tab_state);
    // f.render_widget(mutation_chart, right_chunks[1]);

    // // Render Probability Chart
    // let probability_chart = render_probability_chart(tab_state);
    // f.render_widget(probability_chart, right_chunks[2]);

    // Prepare data for the chart (Simulation Rounds vs. TEs in Exons)
    let mutations_data: Vec<(f64, f64)> = tab_state
        .round_data
        .iter()
        .map(|data| (data.round as f64, data.te_in_exons as f64)) // Map rounds to TEs in Exons
        .collect();

    let chart = Chart::new(vec![Dataset::default()
        .name("Mutations (TEs in Exons)")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Magenta))
        .graph_type(GraphType::Line) // Smooth line graph
        .data(&mutations_data)]) // Use the prepared data
    .block(
        Block::default()
            .title(Span::styled(
                "Mutations Over Simulation Rounds",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL),
    )
    .x_axis(
        Axis::default()
            .title("Simulation Rounds")
            .style(Style::default().fg(Color::White))
            .bounds(get_x_bounds(&mutations_data))
            .labels(get_x_labels(&mutations_data)),
    )
    .y_axis(
        Axis::default()
            .title("# of Mutations (TEs in Exons)")
            .style(Style::default().fg(Color::White))
            .bounds(get_y_bounds(&mutations_data))
            .labels(get_y_labels(&mutations_data)),
    );

    f.render_widget(chart, right_chunks[1]);

    // Prepare data for TEs in Exons vs. Non-Coding
    // Extract TEs in Exons and Non-Coding
    // Extract TEs in Exons and Non-Coding as u64
    let simulation_running = tab_state.simulation_start_triggered;
    // Before the simulation starts, bar values are zero
    let te_in_exons = if simulation_running {
        tab_state.te_in_exons as u64
    } else {
        0
    };

    let te_in_exons = tab_state.te_in_exons as u64;
    let te_in_noncoding = tab_state.te_in_noncoding as u64;

    // BAR CHART
    let data = vec![
        ("TEs in Exons", te_in_exons),
        ("TEs in Non-Coding", te_in_noncoding),
    ];

    //
    let bar_styles = vec![Color::Magenta, Color::Blue];

    let bar_chart = BarChart::default()
        .block(
            Block::default()
                .title(Span::styled(
                    "[ TE Distribution ]",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .bar_width(17)
        .bar_gap(4)
        .data(&data)
        .style(Style::default().fg(Color::White))
        .value_style(Style::default().fg(Color::Black).bg(Color::Green))
        .label_style(Style::default().fg(Color::Cyan))
        // .bar_style(bar_style);
        .bar_style(Style::default().fg(bar_styles[0]).bg(bar_styles[1]));

    f.render_widget(bar_chart, right_chunks[2]);
}

/// Get the bounds for the x-axis based on the data
fn get_x_bounds(data: &Vec<(f64, f64)>) -> [f64; 2] {
    let min = data.first().map(|(x, _)| *x).unwrap_or(0.0);
    let max = data.last().map(|(x, _)| *x).unwrap_or(1.0);
    [min, max]
}

fn get_y_bounds(data: &Vec<(f64, f64)>) -> [f64; 2] {
    let min = data.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min);
    let max = data
        .iter()
        .map(|&(_, y)| y)
        .fold(f64::NEG_INFINITY, f64::max);
    [min, max]
}

fn get_x_labels(data: &Vec<(f64, f64)>) -> Vec<Span<'static>> {
    vec![
        Span::raw(format!("{}", data.first().map(|(x, _)| x).unwrap_or(&0.0))),
        Span::raw(format!("{}", data.last().map(|(x, _)| x).unwrap_or(&1.0))),
    ]
}

fn get_y_labels(data: &Vec<(f64, f64)>) -> Vec<Span<'static>> {
    let min = data.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min);
    let max = data
        .iter()
        .map(|&(_, y)| y)
        .fold(f64::NEG_INFINITY, f64::max);
    vec![
        Span::raw(format!("{:.1}", min)),
        Span::raw(format!("{:.1}", max)),
    ]
}
