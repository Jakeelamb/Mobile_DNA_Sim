use crate::widgets::app_widgets::AppWidget;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Span, Line};

pub fn render_sim_widgets(tab_state: &TabState) -> Vec<AppWidget> {
    // Dynamically populate simulation info
    let species_info_text = vec![
        Span::raw(format!("Species: {}\n", tab_state.current_species)),
        Span::raw(format!("Start Genome Size: {}\n", tab_state.start_genome_size)),
        Span::raw(format!("Start Exon Size: {}\n", tab_state.start_exon_size)),
        Span::raw(format!("Exon/Genome Ratio: {}\n", tab_state.exon_genome_ratio)),
        Span::raw(format!("# of Simulation Rounds completed: {}\n", tab_state.simulation_rounds_completed)),
        Span::raw(format!("# of TEs mobilized: {}\n", tab_state.tes_mobilized)),
        Span::raw(format!("# of Mutations: {}\n", tab_state.mutations)),
        Span::raw(format!("Current Genome Size: {}\n", tab_state.current_genome_size)),
        Span::raw(format!("Current Exon/Genome Ratio: {}\n", tab_state.current_exon_genome_ratio)),
        Span::raw(format!("Current Probability of TE causing Mutation: {}\n", tab_state.probability_of_te_mutation)),
    ];

    // Create a Paragraph with the dynamic information
    let simulation_info = Paragraph::new(species_info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Simulation Info"),
    );

    let settings_block = Paragraph::new("Simulation Settings Block")
        .block(Block::default().borders(Borders::ALL).title("Settings"));

    vec![
        AppWidget::InfoBlock(start_button),
        AppWidget::InfoBlock(simulation_info),
        AppWidget::SettingsBlock(settings_block),
    ]
}