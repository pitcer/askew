use tiny_skia::PixmapMut;

use crate::canvas::base_line::ClosedBaseLine;
use crate::canvas::control_points::point::CurvePoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::shape::{DrawOn, Update};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Polygon {
    pub points: ControlPoints<CurvePoint>,
    pub control_points: VisualControlPoints,
    pub base_line: ClosedBaseLine,
}

impl Polygon {
    #[must_use]
    pub fn new(
        points: ControlPoints<CurvePoint>,
        control_points: VisualControlPoints,
        base_line: ClosedBaseLine,
    ) -> Self {
        Self { points, control_points, base_line }
    }
}

impl Update for Polygon {
    fn update(&mut self) {
        let points = self.points.copied_iterator();
        self.base_line.rebuild_paths(points);

        self.control_points.rebuild_paths(&self.points);
    }
}

impl DrawOn for Polygon {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.base_line.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
    }
}
