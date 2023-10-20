use std::f32::consts::PI;

use tiny_skia::PixmapMut;

use crate::canvas::base_line::ClosedBaseLine;
use crate::canvas::control_points::point::CurveControlPoints;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::control_points_curve::VisualControlPoints;
use crate::canvas::math::point::Point;
use crate::canvas::polygon::Polygon;
use crate::canvas::shape::{DrawOn, Update};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RegularPolygon {
    polygon: Polygon,
    center: Point<f32>,
    radius: f32,
    vertices: u32,
}

impl RegularPolygon {
    #[must_use]
    pub fn new(center: Point<f32>, radius: f32, vertices: u32) -> Self {
        let points = Self::create_regular_polygon_points(center, radius, vertices);
        let polygon =
            Polygon::new(points, VisualControlPoints::default(), ClosedBaseLine::default());
        Self { polygon, center, radius, vertices }
    }

    fn create_regular_polygon_points(
        center: Point<f32>,
        radius: f32,
        vertices: u32,
    ) -> CurveControlPoints {
        let angle_delta = (2.0 * PI) / vertices as f32;
        let points = (0..vertices)
            .map(|index| {
                let x = radius * f32::sin(index as f32 * angle_delta);
                let y = radius * f32::cos(index as f32 * angle_delta);
                Point::new(x, y) + center.into_vector(Point::zero())
            })
            .collect();
        ControlPoints::new(points)
    }
}

impl Update for RegularPolygon {
    fn update(&mut self) {
        self.polygon.update();
    }
}

impl DrawOn for RegularPolygon {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.polygon.draw_on(pixmap);
    }
}

impl Default for RegularPolygon {
    fn default() -> Self {
        Self::new(Point::zero(), 10.0, 3)
    }
}
