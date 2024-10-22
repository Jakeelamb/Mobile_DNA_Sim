// src/widgets/app_widgets.rs
use ratatui::widgets::{List, Paragraph, Widget, Chart};

#[derive(Clone)]
pub enum AppWidget {
    SpeciesList(List<'static>), 
    LogoBlock(Paragraph<'static>),
    InfoBlock(Paragraph<'static>),
    // Chart(Chart<'static>),
    SettingsBlock(Paragraph<'static>),
    FooterBlock(Paragraph<'static>),
}

impl AppWidget {
    pub fn as_widget(&self) -> &dyn Widget {
        match self {
            AppWidget::SpeciesList(list) => list,
            AppWidget::LogoBlock(logo) => logo,
            AppWidget::InfoBlock(info) => info,
            AppWidget::SettingsBlock(settings) => settings,
            // AppWidget::Chart(chart) => chart,
            AppWidget::FooterBlock(footer) => footer,
        }
    }
}
