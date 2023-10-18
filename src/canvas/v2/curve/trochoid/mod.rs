use tiny_skia::PixmapMut;

use crate::canvas::curve::samples::Samples;
use crate::canvas::math::point::Point;
use crate::canvas::v2::base_polyline::OpenBaseLine;
use crate::canvas::v2::curve::trochoid::event_handler::{
    TrochoidCurveEventHandler, TrochoidCurveEventHandlerMut,
};
use crate::canvas::v2::{DrawOn, Update};

pub mod event_handler;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrochoidCurve {
    pub base_line: OpenBaseLine,
    pub properties: TrochoidCurveProperties,
    pub samples: Samples,
}

impl TrochoidCurve {
    #[must_use]
    pub fn new(
        base_line: OpenBaseLine,
        properties: TrochoidCurveProperties,
        samples: Samples,
    ) -> Self {
        Self { base_line, properties, samples }
    }

    pub fn event_handler(&self) -> TrochoidCurveEventHandler<'_> {
        TrochoidCurveEventHandler::new(self)
    }

    pub fn event_handler_mut(&mut self) -> TrochoidCurveEventHandlerMut<'_> {
        TrochoidCurveEventHandlerMut::new(self)
    }
}

impl Update for TrochoidCurve {
    fn update(&mut self) {
        let TrochoidCurveProperties { range_start, range_end, r_1, r_2, w_1, w_2 } =
            self.properties;
        let x = move |t| r_1 * f32::cos(w_1 * t) + r_2 * f32::cos(w_2 * t);
        let y = move |t| r_1 * f32::sin(w_1 * t) + r_2 * f32::sin(w_2 * t);
        let path = self
            .samples
            .equally_spaced(range_start..=range_end)
            .map(move |t| Point::new(x(t) * 200.0 + 250.0, y(t) * 200.0 + 250.0));
        self.base_line.rebuild_paths(path);
    }
}

impl DrawOn for TrochoidCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.base_line.draw_on(pixmap);
    }
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, clap::Args)]
pub struct TrochoidCurveProperties {
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

impl TrochoidCurveProperties {
    #[must_use]
    pub fn new(range_start: f32, range_end: f32, r_1: f32, r_2: f32, w_1: f32, w_2: f32) -> Self {
        Self { range_start, range_end, r_1, r_2, w_1, w_2 }
    }
}

impl Default for TrochoidCurveProperties {
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
