use crate::widgets::app_widgets::AppWidget;
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Axis, Chart, Dataset};
use ratatui::symbols::Marker;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_sim_widgets(tab_state: &TabState) -> Vec<AppWidget> {
    let start_button = Paragraph::new(Span::styled(
        "Start Simulation",
        Style::default()
            .fg(Color::White)
            .bg(Color::Gray)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::ALL).title(""));


    // Convert the Vec<Span> into Vec<Line> (or directly into Text)
    let species_info_text = vec![
        Line::from(vec![Span::raw(format!(
            "Species: {}\n",
            tab_state.current_species
        ))]),
        Line::from(vec![Span::raw(format!(
            "Start Genome Size: {}\n",
            tab_state.start_genome_size
        ))]),
        Line::from(vec![Span::raw(format!(
            "Start Exon Size: {}\n",
            tab_state.start_exon_size
        ))]),
        Line::from(vec![Span::raw(format!(
            "Exon/Genome Ratio: {}\n",
            tab_state.exon_genome_ratio
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

    // Create a Paragraph with the dynamic information
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

    let settings_block = Paragraph::new("Simulation Settings Block").block(
        Block::default()
            .borders(Borders::ALL)
            .title("Settings")
            .title_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Color::Red).fg(Color::White)),
    );

    vec![
        AppWidget::InfoBlock(start_button),
        AppWidget::InfoBlock(simulation_info),
        AppWidget::SettingsBlock(settings_block),
        // AppWidget::Chart(chart),
    ]
}