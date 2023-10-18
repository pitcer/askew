use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::WeightedPoint;
use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::curve::rational_bezier::event_handler::{
    RationalBezierCurveEventHandler, RationalBezierCurveEventHandlerMut,
};
use crate::canvas::v2::{DrawOn, Update};
use crate::{
    canvas::curve::control_points::points::ControlPoints, canvas::curve::samples::Samples,
    canvas::math,
};

pub mod event_handler;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RationalBezierCurve {
    pub control_points: ControlPointsCurve<RationalBezierPoint>,
    pub polyline: BasePolyline<false>,
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
        control_points: ControlPointsCurve<RationalBezierPoint>,
        polyline: BasePolyline<false>,
        properties: RationalBezierCurveProperties,
        samples: Samples,
    ) -> Self {
        Self { control_points, polyline, properties, samples }
    }

    pub fn event_handler(&self) -> RationalBezierCurveEventHandler<'_> {
        RationalBezierCurveEventHandler::new(self)
    }

    pub fn event_handler_mut(&mut self) -> RationalBezierCurveEventHandlerMut<'_> {
        RationalBezierCurveEventHandlerMut::new(self)
    }
}
pub type RationalBezierPoints = ControlPoints<RationalBezierPoint>;
pub type RationalBezierWeight = f32;
pub type RationalBezierPoint = WeightedPoint<f32, RationalBezierWeight>;

impl Update for RationalBezierCurve {
    fn update(&mut self) {
        if self.control_points.points.length() > 1 {
            let points = self.control_points.points.as_slice();

            let path = self.samples.equally_spaced(0.0..=1.0);
            match self.properties.algorithm {
                RationalBezierCurveAlgorithm::DeCasteljau => {
                    self.polyline
                        .rebuild_paths(path.map(|t| math::rational_de_casteljau(points, t)));
                }
                RationalBezierCurveAlgorithm::ChudyWozny => {
                    self.polyline
                        .rebuild_paths(path.map(|t| math::rational_chudy_wozny(points, t)));
                }
            };
        }

        self.control_points.rebuild_paths();
    }
}

impl DrawOn for RationalBezierCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.polyline.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}

impl RationalBezierCurveProperties {
    #[must_use]
    pub fn new(algorithm: RationalBezierCurveAlgorithm) -> Self {
        Self { algorithm }
    }
}
