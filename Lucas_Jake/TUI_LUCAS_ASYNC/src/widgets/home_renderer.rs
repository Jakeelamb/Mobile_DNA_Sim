//home_renderer.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::tabstate::SpeciesData;
use crate::widgets::tabstate::TabState;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn render_home_widgets(tab_state: &mut TabState) -> Vec<AppWidget> {
    // Determine the title for the species list block
    let species_list_title = if tab_state.search_mode {
        format!("Search: {}", tab_state.search_query)
    } else {
        "Species List - Press '/' to search".to_string()
    };

    // let species_items: Vec<ListItem> =
    //     if tab_state.search_mode && !tab_state.search_query.is_empty() {
    //         tab_state.filtered_species.iter()
    //     } else {
    //         tab_state.species.iter()
    //     }
    //     .map(|species_name| ListItem::new(Span::from(species_name.clone())))
    //     .collect();
    // Update filtered species based on search query
    if tab_state.search_mode && !tab_state.search_query.is_empty() {
        tab_state.filtered_species = tab_state
            .species
            .iter()
            .filter(|name| {
                name.to_lowercase()
                    .contains(&tab_state.search_query.to_lowercase())
            })
            .cloned()
            .collect();
    } else {
        tab_state.filtered_species = tab_state.species.clone(); // Reset to full list if no query
    }

    // Ensure the selected index is within bounds of the filtered list
    if tab_state.filtered_species.is_empty() {
        tab_state.list_state.select(None); // If there are no filtered species, clear the selection
    } else {
        let selected_index = tab_state.list_state.selected().unwrap_or(0);
        if selected_index >= tab_state.filtered_species.len() {
            tab_state.list_state.select(Some(0)); // Reset to first item if out of bounds
        }
    }

    // Create list items for display
    let species_items: Vec<ListItem> = tab_state
        .filtered_species
        .iter()
        .map(|species_name| ListItem::new(Span::from(species_name.clone())))
        .collect();

    // Create the species list widget
    let species_list = List::new(species_items)
        .block(Block::default().borders(Borders::ALL).title(Span::styled(
            species_list_title,
            if tab_state.search_mode {
                Style::default().fg(Color::Yellow).bg(Color::Black)
            } else {
                Style::default()
            },
        )))
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    // Create the species list widget with the search query in the title, no extra search bar item
    // let species_list = List::new(species_items)
    //     .block(Block::default().borders(Borders::ALL).title(Span::styled(
    //         species_list_title,
    //         if tab_state.search_mode {
    //             Style::default().fg(Color::Yellow).bg(Color::Black)
    //         } else {
    //             Style::default()
    //         },
    //     )))
    //     .style(Style::default().fg(Color::Black).bg(Color::White))
    //     .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    // Get the selected species for displaying info, ensuring we handle an empty list gracefully
    let selected_species = tab_state.filtered_species.get(tab_state.list_state.selected().unwrap_or(0));
    let species_info = match selected_species {
        Some(species) => tab_state.species_info.get(species).cloned().unwrap_or(SpeciesData::default()),
        None => SpeciesData::default(), // Default info if no species is selected
    };

    // let selected_species = tab_state
    //     .species
    //     .get(tab_state.list_state.selected().unwrap_or(0))
    //     .unwrap();
    
    // let species_info = tab_state
    //     .species_info
    //     .get(selected_species)
    //     .cloned()
    //     .unwrap_or(SpeciesData::default());

    fn format_size(size: f64) -> String {
        if size > 1_000_000.0 {
            format!("{:.1} Mb", size / 1_000_000.0)
        } else if size >= 1_000.0 {
            format!("{:.1} Kb", size / 1_000.0)
        } else {
            format!("{:.0} b", size)
        }
    }

    fn format_percentage(ratio: f64) -> String {
        format!("{:.2}%", ratio * 100.0)
    }

    let genome_size_formatted = format_size(species_info.genome_size);
    let exon_size_formatted = format_size(species_info.exon_size);
    let exon_ratio_formatted = format_percentage(species_info.exon_ratio);

    let species_info_text = format!(
        "\nSpecies: {}\nGenome Size: {}\nExon Size: {}\nExon/Genome Ratio: {}",
        species_info.species, genome_size_formatted, exon_size_formatted, exon_ratio_formatted
    );

    let info_block = Paragraph::new(species_info_text)
        .block(Block::default().borders(Borders::ALL).title("Species Info"))
        .style(Style::default().bg(Color::Blue).fg(Color::White));

    // Create the Simulation Settings block
    let sim_settings_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Starting TEs: {}", tab_state.starting_tes),
            Style::default()
                .fg(if tab_state.active_input == 0 {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(if tab_state.active_input == 0 {
                    Color::Blue
                } else {
                    Color::Green
                }),
        )),
        Line::from(Span::styled(
            format!("Simulation Rounds: {}", tab_state.sim_rounds),
            Style::default()
                .fg(if tab_state.active_input == 1 {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(if tab_state.active_input == 1 {
                    Color::Blue
                } else {
                    Color::Green
                }),
        )),
        Line::from(Span::styled(
            format!("CPU Cores: {}", tab_state.cpu_cores),
            Style::default()
                .fg(if tab_state.active_input == 2 {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(if tab_state.active_input == 2 {
                    Color::Blue
                } else {
                    Color::Green
                }),
        )),
        Line::from(Span::styled(
            format!("Output Directory Path: {}", tab_state.output_dir),
            Style::default()
                .fg(if tab_state.active_input == 3 {
                    Color::Yellow
                } else {
                    Color::White
                })
                .bg(if tab_state.active_input == 3 {
                    Color::Blue
                } else {
                    Color::Green
                }),
        )),
    ];

    let sim_settings_block = Paragraph::new(sim_settings_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sim. Settings [Use <Tab> to scroll]"),
        )
        .style(Style::default().bg(Color::Green).fg(Color::White));

    // Return all widgets as AppWidgets
    vec![
        AppWidget::SpeciesList(species_list),
        AppWidget::InfoBlock(info_block),
        AppWidget::SettingsBlock(sim_settings_block),
    ]
}
