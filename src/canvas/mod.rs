use anyhow::Result;
use tiny_skia::FillRule;
use tiny_skia_path::{Path, PathBuilder, Stroke};

use crate::canvas::curve::{Curve, CurvePoint};
use crate::canvas::geometry::convex_hull::GrahamScan;
use crate::canvas::geometry::point::Point;
use crate::canvas::geometry::rectangle::Rectangle;
use crate::canvas::layout::Panel;
use crate::canvas::paint::{BgraColor, PaintBuilder};
use crate::command::Command;
use crate::curve_apply;

pub mod curve;
pub mod geometry;
pub mod layout;
pub mod math;
pub mod paint;

pub struct Canvas {
    area: Rectangle<f32>,
    content: Curve,
    line_width: f32,
    point_radius: f32,
    show_convex_hull: bool,
}

impl Canvas {
    pub fn new(area: Rectangle<f32>, content: Curve, command: &Command) -> Self {
        Self {
            area,
            content,
            line_width: command.line_width,
            point_radius: command.point_radius,
            show_convex_hull: command.show_convex_hull,
        }
    }

    pub fn add_point(&mut self, point: Point<f32>) {
        self.content.add_point(point)
    }

    pub fn resize(&mut self, area: Rectangle<f32>) {
        self.area = area
    }

    pub fn rasterize(&self, mut panel: Panel<'_>) -> Result<()> {
        if self.show_convex_hull
            && !matches!(self.content, Curve::ConvexHull(_))
            && self.content.points().len() >= 3
        {
            if let Some(path) = self.create_convex_hull_path() {
                let paint = PaintBuilder::new()
                    .bgra_color(BgraColor::from_rgba(0, 255, 255, 255))
                    .build();
                let stroke = Stroke {
                    width: self.line_width,
                    ..Stroke::default()
                };
                panel.draw_stroke_path(&path, &paint, &stroke);
            }
        }

        if let Some(path) = curve_apply!(&self.content => |curve| {
            curve.line_approx_points()
                .and_then(Self::build_path)
                .map(|mut path| {
                    if matches!(self.content, Curve::ConvexHull(_)) {
                        path.close()
                    }
                    path
                })
                .and_then(PathBuilder::finish)
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

    fn create_convex_hull_path(&self) -> Option<Path> {
        let points = self.content.points();
        let points_clone = points.to_owned();
        let mut graham_scan = GrahamScan::new(points_clone);
        let hull = graham_scan.convex_hull();
        let mut path = Self::build_path(hull.into_iter())?;
        path.close();
        path.finish()
    }

    fn build_path(mut points: impl Iterator<Item = CurvePoint>) -> Option<PathBuilder> {
        let mut path = PathBuilder::new();

        let point = points.next()?;
        path.move_to(point.horizontal(), point.vertical());

        for point in points {
            path.line_to(point.horizontal(), point.vertical());
        }

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
