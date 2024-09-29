#[derive(Debug, Clone)]
pub struct Build {}

impl Build {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Build {
    fn default() -> Self {
        Self::new()
    }
}

impl Build {
    pub fn build(&self) {}
}
