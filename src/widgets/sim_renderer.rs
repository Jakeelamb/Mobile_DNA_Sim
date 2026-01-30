use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn render_sim_widgets(tab_state: &TabState, f: &mut Frame, area: Rect) {
    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), // Top section for controls and basic stats
            Constraint::Percentage(70), // Bottom section for charts
        ])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Control panel
            Constraint::Percentage(70), // Basic stats
        ])
        .split(layout_chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left charts
            Constraint::Percentage(50), // Right charts
        ])
        .split(layout_chunks[1]);

    render_control_panel(tab_state, f, top_chunks[0]);
    render_basic_stats(tab_state, f, top_chunks[1]);
    render_charts(tab_state, f, bottom_chunks[0]);
}

fn render_control_panel(tab_state: &TabState, f: &mut Frame, area: Rect) {
    let control_text = vec![
        Line::from(vec![Span::raw(format!(
            "Simulation Progress: {}", 
            tab_state.progress_bar
        ))]),
        Line::from(vec![Span::raw(format!(
            "Runtime: {}", 
            tab_state.run_time
        ))]),
        Line::from(vec![Span::raw(format!(
            "Current Round: {}/{}", 
            tab_state.current_round,
            tab_state.simulation_rounds
        ))]),
    ];

    let control_block = Paragraph::new(control_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Control Panel"))
        .style(Style::default().fg(Color::White));

    f.render_widget(control_block, area);
}

fn render_basic_stats(tab_state: &TabState, f: &mut Frame, area: Rect) {
    let stats_text = if let Some(param) = &tab_state.simulation_param {
        vec![
            Line::from(vec![Span::raw(format!(
                "Genome Size: {}", 
                param.genome_size
            ))]),
            Line::from(vec![Span::raw(format!(
                "Exon Length: {}", 
                param.exon_length
            ))]),
            Line::from(vec![Span::raw(format!(
                "TEs Mobilized: {}", 
                param.te_mobilized
            ))]),
            Line::from(vec![Span::raw(format!(
                "TEs in Exons: {}", 
                param.te_in_exons
            ))]),
            Line::from(vec![Span::raw(format!(
                "Exon Ratio: {:.4}", 
                param.get_exon_ratio()
            ))]),
            Line::from(vec![Span::raw(format!(
                "Mutation Rate: {:.4}", 
                param.get_mutation_rate()
            ))]),
        ]
    } else {
        vec![Line::from(vec![Span::raw("No simulation running")])]
    };

    let stats_block = Paragraph::new(stats_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Statistics"))
        .style(Style::default().fg(Color::White));

    f.render_widget(stats_block, area);
}

fn render_charts(tab_state: &TabState, f: &mut Frame, area: Rect) {
    let charts_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Top charts
            Constraint::Percentage(50), // Bottom charts
        ])
        .split(area);

    // Render Mutation Rate Chart
    let mutation_dataset = vec![Dataset::default()
        .name("Mutations")
        .marker(ratatui::symbols::Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&tab_state.mutation_data)];

    // Calculate dynamic Y-axis bounds for mutations
    let max_mutations = tab_state.mutation_data
        .iter()
        .map(|(_, y)| *y)
        .fold(10.0_f64, f64::max); // Default min of 10 for visibility

    let mutation_chart = Chart::new(mutation_dataset)
        .block(Block::default()
            .title("Mutation Rate")
            .borders(Borders::ALL))
        .x_axis(Axis::default()
            .title("Rounds")
            .bounds([0.0, tab_state.simulation_rounds as f64]))
        .y_axis(Axis::default()
            .title("Mutations")
            .bounds([0.0, max_mutations * 1.1])); // Add 10% padding

    f.render_widget(mutation_chart, charts_layout[0]);

    // Render Genome Size History
    if !tab_state.genome_size_history.is_empty() {
        let genome_data: Vec<(f64, f64)> = tab_state.genome_size_history
            .iter()
            .map(|(round, size)| (*round as f64, *size as f64))
            .collect();

        let genome_dataset = vec![Dataset::default()
            .name("Genome Size")
            .marker(ratatui::symbols::Marker::Dot)
            .style(Style::default().fg(Color::Yellow))
            .data(&genome_data)];

        let genome_chart = Chart::new(genome_dataset)
            .block(Block::default()
                .title("Genome Size Evolution")
                .borders(Borders::ALL))
            .x_axis(Axis::default()
                .title("Rounds")
                .bounds([0.0, tab_state.simulation_rounds as f64]))
            .y_axis(Axis::default()
                .title("Size")
                .bounds([0.0, genome_data.iter().map(|(_, y)| *y).fold(0.0, f64::max)]));

        f.render_widget(genome_chart, charts_layout[1]);
    }
}