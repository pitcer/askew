use anyhow::Result;
use tiny_skia_path::{Path, PathBuilder, Stroke};

use crate::canvas::curve::{Curve, CurvePoint};
use crate::canvas::geometry::point::Point;
use crate::canvas::geometry::rectangle::Rectangle;
use crate::canvas::layout::Panel;
use crate::canvas::paint::{BgraColor, PaintBuilder};
use crate::curve_apply;

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
        self.content.add_point(point)
    }

    pub fn resize(&mut self, area: Rectangle<f32>) {
        self.area = area
    }

    pub fn rasterize(&self, mut panel: Panel<'_>) -> Result<()> {
        if self.content.points().len() < 2 {
            return Ok(());
        }

        let Some(path) = curve_apply!(&self.content => |curve| {
            Self::create_path(curve.line_approx_points())
        }) else {
            return Ok(());
        };

        let paint = PaintBuilder::new()
            .bgra_color(BgraColor::from_rgba(255, 255, 0, 255))
            .build();

        let stroke = Stroke {
            width: 4.0,
            ..Stroke::default()
        };

        panel.draw_path(&path, &paint, &stroke);

        Ok(())
    }

    fn create_path(mut points: impl Iterator<Item = CurvePoint>) -> Option<Path> {
        let mut path = PathBuilder::new();

        let point = points.next()?;
        path.move_to(point.horizontal(), point.vertical());

        for point in points {
            path.line_to(point.horizontal(), point.vertical());
        }

        let path = path.finish()?;
        Some(path)
    }
}
