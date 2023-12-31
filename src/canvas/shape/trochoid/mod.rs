use tiny_skia::PixmapMut;

use crate::canvas::base_line::OpenBaseLine;
use crate::canvas::math::point::Point;
use crate::canvas::samples::Samples;
use crate::canvas::shape::shape_changer::ShapeCommonValues;
use crate::canvas::shape::{DrawOn, Update};
use crate::config::CanvasConfig;

pub mod request;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrochoidCurve {
    base_line: OpenBaseLine,
    properties: TrochoidCurveProperties,
    samples: Samples,
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

impl From<TrochoidCurve> for ShapeCommonValues {
    fn from(value: TrochoidCurve) -> Self {
        Self {
            open_base_line: Some(value.base_line),
            samples: Some(value.samples),
            trochoid_properties: Some(value.properties),
            ..Default::default()
        }
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

impl From<&CanvasConfig> for TrochoidCurveProperties {
    fn from(value: &CanvasConfig) -> Self {
        value.default_trochoid_properties
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
