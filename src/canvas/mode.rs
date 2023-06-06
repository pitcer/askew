use std::fmt::{Display, Formatter};

pub enum Mode {
    PointSelect,
    TypeChange,
    CurveSelect,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::PointSelect => write!(f, "PointSelect"),
            Mode::TypeChange => write!(f, "TypeSelect"),
            Mode::CurveSelect => write!(f, "CurveSelect"),
        }
    }
}
