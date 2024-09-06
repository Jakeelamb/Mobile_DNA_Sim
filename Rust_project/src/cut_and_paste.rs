pub struct CutAndPaste {
    pub sequence: String,
}

impl CutAndPaste {
    pub fn print(&self) -> &str {
        &self.sequence
    }
}