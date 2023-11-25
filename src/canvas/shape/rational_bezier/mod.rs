use tiny_skia::PixmapMut;

use crate::canvas::base_line::VisualBaseLine;
use crate::canvas::control_points::weighted::WeightedPoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::shape::shape_changer::ShapeCommonValues;
use crate::canvas::shape::{DrawOn, Update};
use crate::config::CanvasConfig;
use crate::{canvas::math, canvas::samples::Samples};

pub mod request;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RationalBezierCurve {
    points: ControlPoints<RationalBezierPoint>,
    control_points: VisualControlPoints,
    base_line: VisualBaseLine<false>,
    properties: RationalBezierCurveProperties,
    samples: Samples,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RationalBezierCurveProperties {
    algorithm: RationalBezierCurveAlgorithm,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum RationalBezierCurveAlgorithm {
    #[default]
    DeCasteljau,
    ChudyWozny,
}

impl RationalBezierCurve {
    #[must_use]
    pub fn new(
        points: ControlPoints<RationalBezierPoint>,
        control_points: VisualControlPoints,
        base_line: VisualBaseLine<false>,
        properties: RationalBezierCurveProperties,
        samples: Samples,
    ) -> Self {
        Self { points, control_points, base_line, properties, samples }
    }
}
pub type WeightedControlPoints = ControlPoints<RationalBezierPoint>;
pub type RationalBezierWeight = f32;
pub type RationalBezierPoint = WeightedPoint<f32, RationalBezierWeight>;

impl Update for RationalBezierCurve {
    fn update(&mut self) {
        if self.points.length() > 1 {
            let points = self.points.as_slice();

            let path = self.samples.equally_spaced(0.0..=1.0);
            match self.properties.algorithm {
                RationalBezierCurveAlgorithm::DeCasteljau => {
                    self.base_line
                        .rebuild_paths(path.map(|t| math::rational_de_casteljau(points, t)));
                }
                RationalBezierCurveAlgorithm::ChudyWozny => {
                    self.base_line
                        .rebuild_paths(path.map(|t| math::rational_chudy_wozny(points, t)));
                }
            };
        }

        self.control_points.rebuild_paths(&self.points);
    }
}

impl DrawOn for RationalBezierCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.base_line.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}

impl From<RationalBezierCurve> for ShapeCommonValues {
    fn from(value: RationalBezierCurve) -> Self {
        Self {
            weighted_points: Some(value.points),
            control_points: Some(value.control_points),
            open_base_line: Some(value.base_line),
            rational_bezier_properties: Some(value.properties),
            samples: Some(value.samples),
            ..Default::default()
        }
    }
}

impl RationalBezierCurveProperties {
    #[must_use]
    pub fn new(algorithm: RationalBezierCurveAlgorithm) -> Self {
        Self { algorithm }
    }
}

impl From<&CanvasConfig> for RationalBezierCurveProperties {
    fn from(value: &CanvasConfig) -> Self {
        Self { algorithm: value.default_rational_bezier_algorithm }
    }
}
