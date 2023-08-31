

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELMenuMode {
    Horizontal,
    Vertical
}

impl Default for YELMenuMode {
    fn default() -> Self {
        YELMenuMode::Vertical
    }
}