use anyhow::Result;
use tiny_skia::FillRule;
use tiny_skia::{Path, PathBuilder, Stroke};

use crate::canvas::curve::control_points::{ControlPointsCurve, CurvePoints, GetControlPoints};
use crate::canvas::curve::curve_path::{CurvePath, ToPath};
use crate::canvas::curve::Curve;
use crate::canvas::math::convex_hull::GrahamScan;
use crate::canvas::properties::CanvasProperties;
use crate::enum_apply;
use crate::ui::color::Rgb;
use crate::ui::paint::{PaintBuilder, PaintColor};
use crate::ui::panel::Panel;

pub struct Rasterizer {}

impl Rasterizer {
    pub fn rasterize(
        &self,
        curve: &Curve,
        properties: &CanvasProperties,
        panel: Panel<'_>,
    ) -> Result<()> {
        match curve {
            Curve::ControlPoints(curve) => {
                enum_apply!(curve,
                ControlPointsCurve::Polyline | ControlPointsCurve::Interpolation |
                ControlPointsCurve::Bezier | ControlPointsCurve::RationalBezier => |curve| {
                    let mut rasterizer = CurveRasterizer::new(curve, properties, panel);
                    rasterizer.draw_convex_hull();
                    rasterizer.draw_curve();
                    rasterizer.draw_control_points();
                    rasterizer.draw_current_control_point();
                });
            }
            Curve::Formula(curve) => {
                let mut rasterizer = CurveRasterizer::new(curve, properties, panel);
                rasterizer.draw_curve();
            }
        }

        Ok(())
    }
}

struct CurveRasterizer<'a, T> {
    curve: &'a T,
    properties: &'a CanvasProperties,
    panel: Panel<'a>,
}

impl<'a, T> CurveRasterizer<'a, T> {
    pub fn new(curve: &'a T, properties: &'a CanvasProperties, panel: Panel<'a>) -> Self {
        Self {
            curve,
            properties,
            panel,
        }
    }
}

impl<'a, T> CurveRasterizer<'a, T>
where
    T: ToPath,
{
    fn draw_curve(&mut self) {
        if let Some(path) = self.curve.to_path() {
            let paint = PaintBuilder::new()
                .color(PaintColor::from_rgba(Rgb::new(255, 255, 0), 255))
                .build();
            let stroke = Stroke {
                width: self.properties.line_width,
                ..Stroke::default()
            };
            self.panel.draw_stroke_path(&path, &paint, &stroke);
        }
    }
}

impl<'a, T> CurveRasterizer<'a, T>
where
    T: ToPath + GetControlPoints,
{
    fn draw_convex_hull(&mut self) {
        if self.properties.show_convex_hull && self.curve.control_points().length() >= 3 {
            if let Some(path) = self.create_convex_hull_path() {
                let paint = PaintBuilder::new()
                    .color(PaintColor::from_rgba(Rgb::new(0, 255, 255), 255))
                    .build();
                let stroke = Stroke {
                    width: self.properties.line_width,
                    ..Stroke::default()
                };
                self.panel.draw_stroke_path(&path, &paint, &stroke);
            }
        }
    }

    fn draw_control_points(&mut self) {
        if let Some(points_path) = self.create_points_path(self.properties) {
            let points_paint = PaintBuilder::new()
                .color(PaintColor::from_rgba(Rgb::new(255, 0, 255), 255))
                .build();
            self.panel
                .draw_fill_path(&points_path, &points_paint, FillRule::Winding);
        }
    }

    fn draw_current_control_point(&mut self) {
        if let Some(point) = self
            .curve
            .control_points()
            .get(self.properties.current_point_index)
        {
            let point = point.as_ref();
            let points_paint = PaintBuilder::new()
                .color(PaintColor::from_rgba(Rgb::new(255, 255, 255), 255))
                .build();
            let mut path = PathBuilder::new();
            path.push_circle(
                point.horizontal(),
                point.vertical(),
                self.properties.point_radius * 2.0,
            );
            let path = path.finish();
            if let Some(path) = path {
                self.panel
                    .draw_fill_path(&path, &points_paint, FillRule::Winding);
            }
        }
    }

    fn create_convex_hull_path(&self) -> Option<Path> {
        let points = self
            .curve
            .control_points()
            .iterator()
            .map(|y| y.as_ref())
            .copied();
        let graham_scan = GrahamScan::new(points.collect());
        let hull = graham_scan.convex_hull();
        let mut path = CurvePath::new(hull.into_iter());
        path.close();
        path.into_skia_path()
    }

    fn create_points_path(&self, properties: &CanvasProperties) -> Option<Path> {
        let mut path = PathBuilder::new();

        for point in self.curve.control_points().iterator().map(|y| y.as_ref()) {
            path.push_circle(
                point.horizontal(),
                point.vertical(),
                properties.point_radius,
            );
        }

        let path = path.finish()?;
        Some(path)
    }
}

impl<'a> CurveRasterizer<'a, CurvePoints> {}
