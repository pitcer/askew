use crate::canvas::curve::curve_path::ToPath;
use crate::canvas::curve::formula::trochoid::Trochoid;
use tiny_skia::Path;

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
