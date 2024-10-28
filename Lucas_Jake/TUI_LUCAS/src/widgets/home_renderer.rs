//home_renderer.rs
use crate::widgets::app_widgets::AppWidget;
use crate::widgets::tabstate::TabState;
use crate::widgets::tabstate::SpeciesData;
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Line};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn render_search_bar(search_query: String) -> Paragraph<'static> {
    Paragraph::new(Span::from(format!("Search: {}", search_query)))
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(Style::default().fg(Color::Yellow).bg(Color::Black))
}


pub fn render_home_widgets(tab_state: &mut TabState) -> Vec<AppWidget> {
    // Render the search bar if search mode is enabled
    let search_bar = if tab_state.search_mode {
        Some(render_search_bar(tab_state.search_query.clone())
        )
    } else {
        None
    };

    // Filter species list if search mode is active
    let species_list_title = if tab_state.search_mode {
        format!("Search: {}", tab_state.search_query)
    } else {
        "Species List - Press'/' to search".to_string()
    };

    // let species_items: Vec<ListItem> = if tab_state.search_mode {
    //     // Populate filtered_species initially if empty
    //     if tab_state.filtered_species.is_empty() {
    //         tab_state.filtered_species = tab_state.species.clone();
    //     }
    //     tab_state.filtered_species.iter()
    // } else {
    //     tab_state.species.iter()
    // }
    // .map(|species_name| ListItem::new(Span::from(species_name.clone())))
    // .collect();
    let species_items: Vec<ListItem> = if tab_state.search_mode && !tab_state.search_query.is_empty() {
        tab_state.filtered_species = tab_state.species
            .iter()
            .filter(|name| name.to_lowercase().contains(&tab_state.search_query.to_lowercase()))
            .cloned()
            .collect();
        tab_state.filtered_species.iter()
    } else {
        tab_state.species.iter()
    }
    .map(|species_name| ListItem::new(Span::from(species_name.clone())))
    .collect();

    let species_list = List::new(species_items)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(species_list_title),
    )
    .style(Style::default().fg(Color::Black).bg(Color::White))
    .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    // // Create a List widget with species items
    // let species_list = List::new(species_items)
    //     .block(Block::default().borders(Borders::ALL).title(species_list_title))
    //     .style(Style::default().fg(Color::Black).bg(Color::White))
    //     .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow)); // Highlighted item style

    // Create Species Info block
    let selected_species = tab_state.species.get(tab_state.list_state.selected().unwrap_or(0)).unwrap();
    let species_info = tab_state.species_info.get(selected_species).cloned().unwrap_or(SpeciesData::default());

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

    // Simulation settings block
    let sim_settings_lines = vec![
        Line::from(""),
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

    let sim_settings_block = Paragraph::new(sim_settings_lines)
        .block(Block::default().borders(Borders::ALL).title("Simulation Settings [Use <Tab> to scroll]"))
        .style(Style::default().bg(Color::Green).fg(Color::White));

    let mut widgets = vec![
        // AppWidget::SearchBar(search_bar.unwrap_or_else(|| Paragraph::new(""))),
        AppWidget::SpeciesList(species_list),
        AppWidget::InfoBlock(info_block),
        AppWidget::SettingsBlock(sim_settings_block),
    ];

    // Add the search bar widget if it's active
    if let Some(bar) = search_bar {
        widgets.insert(0, AppWidget::SearchBar(bar));
    }

    widgets
}
