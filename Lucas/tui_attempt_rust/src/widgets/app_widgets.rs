// src/widgets/app_widgets.rs
use ratatui::widgets::{List, Paragraph, Widget};

#[derive(Clone)]
pub enum AppWidget {
    SpeciesList(List<'static>), // Change to List instead of Paragraph
    LogoBlock(Paragraph<'static>),
    InfoBlock(Paragraph<'static>),
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
            AppWidget::FooterBlock(footer) => footer,
        }
    }
}

// Logo functions for different tabs
// pub fn get_ascii_logo() -> String {
//     fs::read_to_string("img/logo2.txt")
//         .expect("Failed to read logo.txt")
//         .trim()
//         .to_string()
// }

// pub fn get_ascii_sim() -> String {
//     fs::read_to_string("img/simulation.txt")
//         .expect("Failed to read simulation.txt")
//         .trim()
//         .to_string()
// }