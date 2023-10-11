use anyhow::Result;
use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::kind::bezier::BezierCurveAlgorithm;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::v2::base_polyline::BasePolyline;
use crate::canvas::v2::control_points_curve::ControlPointsCurve;
use crate::canvas::v2::DrawOn;

#[derive(Debug, Clone)]
pub struct BezierCurve {
    control_points: ControlPointsCurve<CurvePoint>,
    polyline: BasePolyline,
    properties: BezierCurveProperties,
}

#[derive(Debug, Clone)]
pub struct BezierCurveProperties {
    algorithm: BezierCurveAlgorithm,
}

impl BezierCurve {
    pub fn new(
        control_points: ControlPointsCurve<CurvePoint>,
        polyline: BasePolyline,
        properties: BezierCurveProperties,
    ) -> Self {
        Self { control_points, polyline, properties }
    }
}

impl DrawOn for BezierCurve {
    fn draw_on(&self, pixmap: PixmapMut<'_>) -> Result<()> {
        todo!()
    }
}

impl BezierCurveProperties {
    pub fn new(algorithm: BezierCurveAlgorithm) -> Self {
        Self { algorithm }
    }
}
