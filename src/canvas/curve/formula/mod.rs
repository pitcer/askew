use std::fmt::{Display, Formatter};

use tiny_skia::Path;

use crate::canvas::curve::curve_path::ToPath;
use crate::canvas::curve::formula::trochoid::Trochoid;

pub mod trochoid;

#[derive(Debug)]
pub enum FormulaCurve {
    Trochoid(Trochoid),
}

impl ToPath for FormulaCurve {
    fn to_path(&self) -> Option<Path> {
        match self {
            FormulaCurve::Trochoid(trochoid) => trochoid.to_path(),
        }
    }
}

impl Display for FormulaCurve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaCurve::Trochoid(_) => write!(f, "trochoid"),
        }
    }
}
