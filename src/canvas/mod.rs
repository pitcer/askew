use anyhow::Result;
use nalgebra::Point2;
use tiny_skia_path::{PathBuilder, Stroke};

use crate::canvas::curve::Curve;
use crate::canvas::geometry::point::Point;
use crate::canvas::geometry::rectangle::Rectangle;
use crate::canvas::layout::Panel;
use crate::canvas::paint::{BgraColor, PaintBuilder};

pub mod curve;
pub mod geometry;
pub mod layout;
pub mod paint;

pub struct Canvas {
    area: Rectangle<f32>,
    content: Curve,
}

impl Canvas {
    pub fn new(area: Rectangle<f32>, content: Curve) -> Self {
        Self { area, content }
    }

    pub fn add_point(&mut self, point: Point<f32>) {
        let point = Point2::from([point.horizontal(), point.vertical()]);
        self.content.add_point(point)
    }

    pub fn rasterize(&self, mut panel: Panel<'_>) -> Result<()> {
        let mut path = PathBuilder::new();
        let mut points = self.content.line_approx_points();
        let Some(first_point) = points.next() else {
            return Ok(());
        };
        path.move_to(first_point.x, first_point.y);
        for point in points {
            path.line_to(point.x, point.y);
        }
        let Some(path) = path.finish() else {
            return Ok(());
        };

        let paint = PaintBuilder::new()
            .bgra_color(BgraColor::from_rgba(255, 255, 0, 255))
            .build();

        let stroke = Stroke {
            width: 4.0,
            ..Stroke::default()
        };

        panel.draw_path(&path, &paint, &stroke)?;

        Ok(())
    }
}
