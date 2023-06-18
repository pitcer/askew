use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};
use crate::canvas::curve::formula::trochoid::event_handler::TrochoidEventHandler;
use crate::canvas::curve::samples::Samples;
use crate::canvas::math::point::Point;

pub mod event_handler;

#[derive(Debug)]
pub struct Trochoid {
    samples: Samples,
    range: (f32, f32),
    r_1: f32,
    r_2: f32,
    w_1: f32,
    w_2: f32,
}

impl Trochoid {
    #[must_use]
    pub fn new(
        samples: Samples,
        range: (f32, f32),
        r_1: f32,
        r_2: f32,
        w_1: f32,
        w_2: f32,
    ) -> Self {
        Self {
            samples,
            range,
            r_1,
            r_2,
            w_1,
            w_2,
        }
    }

    pub fn event_handler(&mut self) -> TrochoidEventHandler<'_> {
        TrochoidEventHandler::new(self)
    }
}

impl ToPath for Trochoid {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        let x = move |t| self.r_1 * f32::cos(self.w_1 * t) + self.r_2 * f32::cos(self.w_2 * t);
        let y = move |t| self.r_1 * f32::sin(self.w_1 * t) + self.r_2 * f32::sin(self.w_2 * t);
        let path = self
            .samples
            .equally_spaced(self.range.0..=self.range.1)
            .map(move |t| Point::new(x(t) * 200.0 + 250.0, y(t) * 200.0 + 250.0));
        let path = CurvePath::new_open(path);
        converter.to_path(path)
    }
}
