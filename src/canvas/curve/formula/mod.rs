use crate::canvas::curve::converter::{PathConverter, ToPath};
use crate::canvas::curve::formula::event_handler::FormulaCurveEventHandler;
use crate::canvas::curve::formula::trochoid::Trochoid;

pub mod event_handler;
pub mod trochoid;

#[derive(Debug)]
pub enum FormulaCurveKind {
    Trochoid(Trochoid),
}

impl FormulaCurveKind {
    pub fn event_handler(&mut self) -> FormulaCurveEventHandler<'_> {
        FormulaCurveEventHandler::new(self)
    }
}

impl ToPath for FormulaCurveKind {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        match self {
            FormulaCurveKind::Trochoid(trochoid) => trochoid.to_path(converter),
        }
    }
}
