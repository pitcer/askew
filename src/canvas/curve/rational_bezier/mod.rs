use tiny_skia::PixmapMut;

use crate::canvas::base_line::VisualBaseLine;
use crate::canvas::control_points::point::WeightedPoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::curve::{DrawOn, Update};
use crate::{canvas::math, canvas::samples::Samples};

pub mod request;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RationalBezierCurve {
    pub points: ControlPoints<RationalBezierPoint>,
    pub control_points: VisualControlPoints,
    pub base_line: VisualBaseLine<false>,
    pub properties: RationalBezierCurveProperties,
    pub samples: Samples,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

impl RationalBezierCurveProperties {
    #[must_use]
    pub fn new(algorithm: RationalBezierCurveAlgorithm) -> Self {
        Self { algorithm }
    }
}
