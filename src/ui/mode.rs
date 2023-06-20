use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Curve,
    Point,
    PointAdd,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Curve => write!(f, "Curve"),
            Mode::Point => write!(f, "Point"),
            Mode::PointAdd => write!(f, "PointAdd"),
        }
    }
}

#[derive(Debug)]
pub enum ModeState {
    Curve(ModeCurve),
    Point(ModePoint),
    PointAdd(ModePointAdd),
}

impl ModeState {
    #[must_use]
    pub fn initial() -> Self {
        ModeState::Curve(ModeCurve)
    }

    pub fn enter_point(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            ModeState::Curve(mode) => ModeState::Point(mode.into_point()),
            other => other,
        });
    }

    pub fn enter_add(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            ModeState::Point(mode) => ModeState::PointAdd(mode.add()),
            other => other,
        });
    }

    pub fn exit(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            ModeState::Point(mode) => ModeState::Curve(mode.exit()),
            ModeState::PointAdd(mode) => ModeState::Point(mode.exit()),
            other => other,
        });
    }

    #[must_use]
    pub fn as_mode(&self) -> Mode {
        match self {
            ModeState::Curve(_) => Mode::Curve,
            ModeState::Point(_) => Mode::Point,
            ModeState::PointAdd(_) => Mode::PointAdd,
        }
    }
}

#[derive(Debug)]
pub struct ModeCurve;

impl ModeCurve {
    #[must_use]
    pub fn into_point(self) -> ModePoint {
        ModePoint
    }
}

#[derive(Debug)]
pub struct ModePoint;

impl ModePoint {
    #[must_use]
    pub fn add(self) -> ModePointAdd {
        ModePointAdd
    }

    #[must_use]
    pub fn exit(self) -> ModeCurve {
        ModeCurve
    }
}

#[derive(Debug)]
pub struct ModePointAdd;

impl ModePointAdd {
    #[must_use]
    pub fn exit(self) -> ModePoint {
        ModePoint
    }
}
