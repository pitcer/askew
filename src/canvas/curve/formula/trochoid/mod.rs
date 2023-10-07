use crate::canvas::curve::converter::{CurvePath, PathConverter, ToPath};
use crate::canvas::curve::formula::trochoid::event_handler::TrochoidEventHandler;
use crate::canvas::curve::samples::Samples;
use crate::canvas::math::point::Point;

pub mod event_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Trochoid {
    samples: Samples,
    properties: TrochoidProperties,
}

impl Trochoid {
    #[must_use]
    pub fn new(samples: Samples, properties: TrochoidProperties) -> Self {
        Self { samples, properties }
    }

    pub fn event_handler(&mut self) -> TrochoidEventHandler<'_> {
        TrochoidEventHandler::new(self)
    }
}

impl ToPath for Trochoid {
    fn to_path<P>(&self, converter: impl PathConverter<Path = P>) -> Option<P> {
        let TrochoidProperties { range_start, range_end, r_1, r_2, w_1, w_2 } = self.properties;
        let x = move |t| r_1 * f32::cos(w_1 * t) + r_2 * f32::cos(w_2 * t);
        let y = move |t| r_1 * f32::sin(w_1 * t) + r_2 * f32::sin(w_2 * t);
        let path = self
            .samples
            .equally_spaced(range_start..=range_end)
            .map(move |t| Point::new(x(t) * 200.0 + 250.0, y(t) * 200.0 + 250.0));
        let path = CurvePath::new_open(path);
        converter.to_path(path)
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, clap::Args)]
pub struct TrochoidProperties {
    #[arg()]
    pub range_start: f32,
    #[arg()]
    pub range_end: f32,
    #[arg()]
    pub r_1: f32,
    #[arg()]
    pub r_2: f32,
    #[arg()]
    pub w_1: f32,
    #[arg()]
    pub w_2: f32,
}

impl TrochoidProperties {
    #[must_use]
    pub fn new(range_start: f32, range_end: f32, r_1: f32, r_2: f32, w_1: f32, w_2: f32) -> Self {
        Self { range_start, range_end, r_1, r_2, w_1, w_2 }
    }
}

impl Default for TrochoidProperties {
    fn default() -> Self {
        Self {
            range_start: 10.0 * -std::f32::consts::PI,
            range_end: 10.0 * std::f32::consts::PI,
            r_1: 0.3,
            r_2: 0.8,
            w_1: 0.3,
            w_2: 0.7,
        }
    }
}
