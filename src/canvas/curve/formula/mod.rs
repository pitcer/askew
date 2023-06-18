use std::fmt::{Display, Formatter};

use crate::canvas::curve::converter::{PathConverter, ToPath};
use crate::canvas::curve::formula::trochoid::Trochoid;

pub mod trochoid;

#[derive(Debug)]
pub enum FormulaCurveKind {
    Trochoid(Trochoid),
}

impl FormulaCurveKind {
    pub fn samples_mut(&mut self) -> Option<&mut u32> {
        match self {
            FormulaCurveKind::Trochoid(curve) => Some(curve.samples_mut()),
        }
    }
}

impl ToPath for FormulaCurveKind {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        match self {
            FormulaCurveKind::Trochoid(trochoid) => trochoid.to_path(converter),
        }
    }
}

impl Display for FormulaCurveKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaCurveKind::Trochoid(_) => write!(f, "trochoid"),
        }
    }
}
