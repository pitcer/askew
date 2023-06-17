use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Normal,
    Curve,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Curve => write!(f, "Curve"),
        }
    }
}
