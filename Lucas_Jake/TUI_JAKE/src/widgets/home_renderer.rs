//home_renderer.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::tabstate::TabState;
use crate::widgets::tabstate::SpeciesData;
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Line};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

/// Helper function to render widgets for the Home tab
pub fn render_home_widgets(tab_state: &TabState) -> Vec<AppWidget> {
    // Create a List widget for the species
    let species_items: Vec<ListItem> = tab_state
        .species
        .iter()
        .map(|species_name| ListItem::new(Span::from(species_name.clone()))) // Use `Span::from` for each species
        .collect();

    // Create a List widget with species items
    let species_list = List::new(species_items)
        .block(Block::default().borders(Borders::ALL).title("Species List\n"))
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow)); // Highlighted item style

    let sim_settings_lines = vec![
        // Line::from(Span::styled("[Use <Tab> to scroll]", Style::default().fg(Color::White).add_modifier(Modifier::BOLD))),
        Line::from(""),  // Empty line to create space
        Line::from(Span::styled(
            format!("Starting TEs: {}", tab_state.starting_tes),
            Style::default()
                .fg(if tab_state.active_input == 0 { Color::Yellow } else { Color::White })
                .bg(if tab_state.active_input == 0 { Color::Blue } else { Color::Green }),
        )),
        Line::from(Span::styled(
            format!("Simulation Rounds: {}", tab_state.sim_rounds),
            Style::default()
                .fg(if tab_state.active_input == 1 { Color::Yellow } else { Color::White })
                .bg(if tab_state.active_input == 1 { Color::Blue } else { Color::Green }),
        )),
        Line::from(Span::styled(
            format!("CPU Cores: {}", tab_state.cpu_cores),
            Style::default()
                .fg(if tab_state.active_input == 2 { Color::Yellow } else { Color::White })
                .bg(if tab_state.active_input == 2 { Color::Blue } else { Color::Green }),
        )),
        Line::from(Span::styled(
            format!("Output Directory Path: {}", tab_state.output_dir),
            Style::default()
                .fg(if tab_state.active_input == 3 { Color::Yellow } else { Color::White })
                .bg(if tab_state.active_input == 3 { Color::Blue } else { Color::Green }),
        )),
    ];


     // Create Species Info block
    let selected_species = tab_state.species.get(tab_state.list_state.selected().unwrap_or(0)).unwrap();

    let species_info = tab_state.species_info.get(selected_species).cloned().unwrap_or(SpeciesData::default());

    let species_info_text = format!(
        "\nSpecies: {}\nGenome Size: {}\nExon Size: {}\nExon/Genome Ratio: {}",
        species_info.name, species_info.genome_size, species_info.exon_size, species_info.exon_ratio
    );

    let info_block = Paragraph::new(species_info_text)
        .block(Block::default().borders(Borders::ALL).title("Species Info\n"))
        .style(Style::default().bg(Color::Blue).fg(Color::White));

    // Use the Spans to create a Paragraph for Simulation Settings
    let sim_settings_block = Paragraph::new(sim_settings_lines)
        .block(Block::default().borders(Borders::ALL).title("Simulation Settings [Use <Tab> to scroll]"))
        .style(Style::default().bg(Color::Green).fg(Color::White));
        // .add_modifier(Modifier::BOLD));

    vec![
        AppWidget::SpeciesList(species_list),
        AppWidget::InfoBlock(info_block),
        AppWidget::SettingsBlock(sim_settings_block),
    ]
}