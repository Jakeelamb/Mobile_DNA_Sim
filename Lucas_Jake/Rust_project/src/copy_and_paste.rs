pub struct CopyAndPaste {
    pub sequence: String,
}

impl CopyAndPaste {
    pub fn print(&self) -> &str {
        &self.sequence
    }
}