use tiny_skia::PixmapMut;

use crate::canvas::base_line::VisualBaseLine;
use crate::canvas::control_points::point::CurvePoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::shape::shape_changer::ShapeCommonValues;
use crate::canvas::shape::{DrawOn, Update};

pub mod request;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PolylineCurve {
    points: ControlPoints<CurvePoint>,
    control_points: VisualControlPoints,
    base_line: VisualBaseLine<false>,
}

impl PolylineCurve {
    #[must_use]
    pub fn new(
        points: ControlPoints<CurvePoint>,
        control_points: VisualControlPoints,
        base_line: VisualBaseLine<false>,
    ) -> Self {
        Self { points, control_points, base_line }
    }
}

impl Update for PolylineCurve {
    fn update(&mut self) {
        let points = self.points.points_iterator();
        self.base_line.rebuild_paths(points);

        self.control_points.rebuild_paths(&self.points);
    }
}

impl DrawOn for PolylineCurve {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.base_line.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}

impl From<PolylineCurve> for ShapeCommonValues {
    fn from(value: PolylineCurve) -> Self {
        Self {
            points: Some(value.points),
            control_points: Some(value.control_points),
            open_base_line: Some(value.base_line),
            ..Default::default()
        }
    }
}
