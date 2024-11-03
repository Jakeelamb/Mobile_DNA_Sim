// src/widgets/app_widgets.rs
use ratatui::widgets::{List, Paragraph, Widget};

#[derive(Clone)]
pub enum AppWidget {
    SpeciesList(List<'static>), 
    LogoBlock(Paragraph<'static>),
    InfoBlock(Paragraph<'static>),
    // Chart(Chart<'static>),
    RunTimeBlock(Paragraph<'static>),
    CurrentRoundBlock(Paragraph<'static>),
    SettingsBlock(Paragraph<'static>),
    FooterBlock(Paragraph<'static>),
    SearchBar(Paragraph<'static>),
}

impl AppWidget {
    pub fn as_widget(&self) -> &dyn Widget {
        match self {
            AppWidget::SpeciesList(list) => list,
            AppWidget::LogoBlock(logo) => logo,
            AppWidget::InfoBlock(info) => info,
            AppWidget::RunTimeBlock(run_time) => run_time,
            AppWidget::CurrentRoundBlock(current_round) => current_round,
            AppWidget::SettingsBlock(settings) => settings,
            // AppWidget::Chart(chart) => chart,
            AppWidget::FooterBlock(footer) => footer,
            AppWidget::SearchBar(search_bar) => search_bar,
        }
    }
}
