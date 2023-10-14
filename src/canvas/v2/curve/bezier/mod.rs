use anyhow::Result;
use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::kind::bezier::BezierCurveAlgorithm;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::curve::samples::Samples;
use crate::canvas::math;
use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::curve::bezier::event_handler::BezierV2EventHandler;
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

    pub fn event_handler(&mut self) -> BezierV2EventHandler<'_> {
        BezierV2EventHandler::new(self)
    }
}

impl DrawOn for BezierCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) -> Result<()> {
        self.polyline.draw_on(pixmap)?;
        self.control_points.draw_on(pixmap)?;
        Ok(())
    }
}

impl Update for BezierCurve {
    fn update(&mut self) -> Result<()> {
        if self.control_points.points.length() < 2 {
            return Ok(());
        }

        let path = self.samples.equally_spaced(0.0..=1.0);
        match self.properties.algorithm {
            BezierCurveAlgorithm::DeCasteljau => {
                let path =
                    path.map(|t| math::de_casteljau(self.control_points.points.as_slice(), t));
                self.polyline.rebuild_paths(path)?;
            }
            BezierCurveAlgorithm::ChudyWozny => {
                let path =
                    path.map(|t| math::chudy_wozny(self.control_points.points.as_slice(), t));
                self.polyline.rebuild_paths(path)?;
            }
        }
        self.control_points.rebuild_paths()?;
        Ok(())
    }
}

impl BezierCurveProperties {
    #[must_use]
    pub fn new(algorithm: BezierCurveAlgorithm) -> Self {
        Self { algorithm }
    }
}
