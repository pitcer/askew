use tiny_skia::PixmapMut;

use crate::canvas::base_line::VisualBaseLine;
use crate::canvas::control_points::point::CurvePoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::math;
use crate::canvas::samples::Samples;
use crate::canvas::shape::shape_changer::ShapeCommonValues;
use crate::canvas::shape::{DrawOn, Update};
use crate::config::CanvasConfig;

pub mod request;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BezierCurve {
    points: ControlPoints<CurvePoint>,
    control_points: VisualControlPoints,
    polyline: VisualBaseLine<false>,
    properties: BezierCurveProperties,
    samples: Samples,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BezierCurveProperties {
    algorithm: BezierCurveAlgorithm,
}

impl BezierCurve {
    #[must_use]
    pub fn new(
        points: ControlPoints<CurvePoint>,
        control_points: VisualControlPoints,
        polyline: VisualBaseLine<false>,
        properties: BezierCurveProperties,
        samples: Samples,
    ) -> Self {
        Self { points, control_points, polyline, properties, samples }
    }
}

impl Update for BezierCurve {
    fn update(&mut self) {
        if self.points.length() > 1 {
            let points = self.points.as_slice();

            let path = self.samples.equally_spaced(0.0..=1.0);
            match self.properties.algorithm {
                BezierCurveAlgorithm::DeCasteljau => {
                    self.polyline.rebuild_paths(path.map(|t| math::de_casteljau(points, t)));
                }
                BezierCurveAlgorithm::ChudyWozny => {
                    self.polyline.rebuild_paths(path.map(|t| math::chudy_wozny(points, t)));
                }
            };
        }

        self.control_points.rebuild_paths(&self.points);
    }
}

impl DrawOn for BezierCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.polyline.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}

impl From<BezierCurve> for ShapeCommonValues {
    fn from(value: BezierCurve) -> Self {
        Self {
            points: Some(value.points),
            control_points: Some(value.control_points),
            open_base_line: Some(value.polyline),
            bezier_properties: Some(value.properties),
            samples: Some(value.samples),
            ..Default::default()
        }
    }
}

impl BezierCurveProperties {
    #[must_use]
    pub fn new(algorithm: BezierCurveAlgorithm) -> Self {
        Self { algorithm }
    }
}

impl From<&CanvasConfig> for BezierCurveProperties {
    fn from(value: &CanvasConfig) -> Self {
        Self { algorithm: value.default_bezier_algorithm }
    }
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum BezierCurveAlgorithm {
    #[default]
    DeCasteljau,
    ChudyWozny,
}
