use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Curve,
    Point,
    PointSelect,
    PointAdd,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Curve => write!(f, "Curve"),
            Mode::Point => write!(f, "Point"),
            Mode::PointAdd => write!(f, "PointAdd"),
            Mode::PointSelect => write!(f, "PointSelect"),
        }
    }
}

#[derive(Debug)]
pub enum ModeState {
    Curve(ModeCurve),
    Point(ModePoint),
    PointAdd(ModePointAdd),
    PointSelect(ModePointSelect),
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

    pub fn select(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            ModeState::Point(mode) => ModeState::PointSelect(mode.select()),
            other => other,
        });
    }

    pub fn exit(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            ModeState::Point(mode) => ModeState::Curve(mode.exit()),
            ModeState::PointAdd(mode) => ModeState::Point(mode.exit()),
            ModeState::PointSelect(mode) => ModeState::Point(mode.exit()),
            other => other,
        });
    }

    #[must_use]
    pub fn as_mode(&self) -> Mode {
        match self {
            ModeState::Curve(_) => Mode::Curve,
            ModeState::Point(_) => Mode::Point,
            ModeState::PointAdd(_) => Mode::PointAdd,
            ModeState::PointSelect(_) => Mode::PointSelect,
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
    pub fn select(self) -> ModePointSelect {
        ModePointSelect
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

#[derive(Debug)]
pub struct ModePointSelect;

impl ModePointSelect {
    #[must_use]
    pub fn exit(self) -> ModePoint {
        ModePoint
    }
}
