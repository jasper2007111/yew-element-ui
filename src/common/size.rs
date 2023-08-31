
#[derive(Clone, PartialEq, Debug, Eq)]
pub enum YELSize {
    Medium,
    Small,
    Mini
}

impl std::fmt::Display for YELSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let result = match *self {
            YELSize::Medium => "medium",
            YELSize::Small => "small",
            YELSize::Mini => "mini",
        };
        write!(f, "{}", result)
    }
}