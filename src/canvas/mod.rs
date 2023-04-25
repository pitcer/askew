use anyhow::Result;
use tiny_skia::FillRule;
use tiny_skia_path::{Path, PathBuilder, Stroke};

use crate::canvas::curve::{Curve, CurvePoint};
use crate::canvas::geometry::point::Point;
use crate::canvas::geometry::rectangle::Rectangle;
use crate::canvas::layout::Panel;
use crate::canvas::paint::{BgraColor, PaintBuilder};
use crate::command::Command;
use crate::curve_apply;

pub mod curve;
pub mod geometry;
pub mod layout;
pub mod paint;

pub struct Canvas {
    area: Rectangle<f32>,
    content: Curve,
    line_width: f32,
    point_radius: f32,
}

impl Canvas {
    pub fn new(area: Rectangle<f32>, content: Curve, command: &Command) -> Self {
        Self {
            area,
            content,
            line_width: command.line_width,
            point_radius: command.point_radius,
        }
    }

    pub fn add_point(&mut self, point: Point<f32>) {
        self.content.add_point(point)
    }

    pub fn resize(&mut self, area: Rectangle<f32>) {
        self.area = area
    }

    pub fn rasterize(&self, mut panel: Panel<'_>) -> Result<()> {
        if let Some(path) = curve_apply!(&self.content => |curve| {
            Self::create_path(curve.line_approx_points())
        }) {
            let paint = PaintBuilder::new()
                .bgra_color(BgraColor::from_rgba(255, 255, 0, 255))
                .build();
            let stroke = Stroke {
                width: self.line_width,
                ..Stroke::default()
            };
            panel.draw_stroke_path(&path, &paint, &stroke);
        }

        if let Some(points_path) = self.create_points_path() {
            let points_paint = PaintBuilder::new()
                .bgra_color(BgraColor::from_rgba(255, 0, 255, 255))
                .build();
            panel.draw_fill_path(&points_path, &points_paint, FillRule::Winding);
        }
        Ok(())
    }

    fn create_path(points: Option<impl Iterator<Item = CurvePoint>>) -> Option<Path> {
        let mut points = points?;
        let mut path = PathBuilder::new();

        let point = points.next()?;
        path.move_to(point.horizontal(), point.vertical());

        for point in points {
            path.line_to(point.horizontal(), point.vertical());
        }

        let path = path.finish()?;
        Some(path)
    }

    fn create_points_path(&self) -> Option<Path> {
        let mut path = PathBuilder::new();

        for point in self.content.points() {
            path.push_circle(point.horizontal(), point.vertical(), self.point_radius);
        }

        let path = path.finish()?;
        Some(path)
    }
}
