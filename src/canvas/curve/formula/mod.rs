use tiny_skia::PixmapMut;

use crate::canvas::curve::formula::event_handler::FormulaCurveEventHandler;
use crate::canvas::v2::curve::trochoid::TrochoidCurve;
use crate::canvas::v2::{DrawOn, Update};

pub mod event_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FormulaCurveKind {
    Trochoid(Box<TrochoidCurve>),
}

impl FormulaCurveKind {
    pub fn event_handler(&mut self) -> FormulaCurveEventHandler<'_> {
        FormulaCurveEventHandler::new(self)
    }
}

impl Update for FormulaCurveKind {
    fn update(&mut self) {
        match self {
            FormulaCurveKind::Trochoid(curve) => curve.update(),
        }
    }
}

impl DrawOn for FormulaCurveKind {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        match self {
            FormulaCurveKind::Trochoid(curve) => curve.draw_on(pixmap),
        }
    }
}
