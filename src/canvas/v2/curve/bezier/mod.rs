use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::curve::samples::Samples;
use crate::canvas::math;
use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::curve::bezier::event_handler::BezierCurveEventHandler;
use crate::canvas::v2::{DrawOn, Update};

pub mod event_handler;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BezierCurve {
    pub control_points: ControlPointsCurve<CurvePoint>,
    pub polyline: BasePolyline<false>,
    pub properties: BezierCurveProperties,
    pub samples: Samples,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BezierCurveProperties {
    algorithm: BezierCurveAlgorithm,
}

impl BezierCurve {
    #[must_use]
    pub fn new(
        control_points: ControlPointsCurve<CurvePoint>,
        polyline: BasePolyline<false>,
        properties: BezierCurveProperties,
        samples: Samples,
    ) -> Self {
        Self { control_points, polyline, properties, samples }
    }

    pub fn event_handler(&mut self) -> BezierCurveEventHandler<'_> {
        BezierCurveEventHandler::new(self)
    }
}

impl Update for BezierCurve {
    fn update(&mut self) {
        if self.control_points.points.length() > 1 {
            let points = self.control_points.points.as_slice();

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

        self.control_points.rebuild_paths();
    }
}

impl DrawOn for BezierCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.polyline.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}

impl BezierCurveProperties {
    #[must_use]
    pub fn new(algorithm: BezierCurveAlgorithm) -> Self {
        Self { algorithm }
    }
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum BezierCurveAlgorithm {
    #[default]
    DeCasteljau,
    ChudyWozny,
}
