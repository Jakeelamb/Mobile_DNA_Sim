use ratatui::widgets::{List, Paragraph};

#[derive(Clone)]
pub enum AppWidget {
    SpeciesList(List<'static>),
    InfoBlock(Paragraph<'static>),
    SettingsBlock(Paragraph<'static>),
}
