use std::fmt::{Display, Formatter};

use crate::canvas::curve::converter::{PathConverter, ToPath};
use crate::canvas::curve::formula::trochoid::Trochoid;

pub mod trochoid;

#[derive(Debug)]
pub enum FormulaCurve {
    Trochoid(Trochoid),
}

impl ToPath for FormulaCurve {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        match self {
            FormulaCurve::Trochoid(trochoid) => trochoid.to_path(converter),
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
