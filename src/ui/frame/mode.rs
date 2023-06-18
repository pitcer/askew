use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Curve,
    Point,
}

#[derive(Debug)]
pub enum ModeState {
    Curve(ModeCurve),
    Point(ModePoint),
}

impl ModeState {
    #[must_use]
    pub fn initial() -> Self {
        ModeState::Curve(ModeCurve::new())
    }

    pub fn enter_point(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            ModeState::Curve(mode) => ModeState::Point(mode.into_point()),
            other => other,
        });
    }

    pub fn exit(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            ModeState::Point(mode) => ModeState::Curve(mode.exit()),
            other => other,
        });
    }
}

impl Display for ModeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeState::Curve(_) => write!(f, "Curve"),
            ModeState::Point(_) => write!(f, "Point"),
        }
    }
}

#[derive(Debug)]
pub struct ModeCurve {}

impl ModeCurve {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn into_point(self) -> ModePoint {
        ModePoint::new()
    }
}

#[derive(Debug)]
pub struct ModePoint {}

impl ModePoint {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn exit(self) -> ModeCurve {
        ModeCurve::new()
    }
}
