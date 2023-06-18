use anyhow::Result;
use tiny_skia::FillRule;
use tiny_skia::{Path, PathBuilder, Stroke};

use crate::canvas::curve::control_points::kind::convex_hull::ConvexHull;
use crate::canvas::curve::control_points::{ControlPointsCurveKind, CurvePoints, GetControlPoints};
use crate::canvas::curve::converter::{TinySkiaPathConverter, ToPath};
use crate::canvas::curve::CurveKind;
use crate::canvas::paint::PaintBuilder;
use crate::canvas::properties::CanvasProperties;
use crate::ui::frame::panel::Panel;

#[derive(Debug)]
pub struct Rasterizer;

impl Rasterizer {
    pub fn rasterize<'a>(
        &self,
        curve: &'a CurveKind,
        properties: &'a CanvasProperties,
        panel: &'a mut Panel<'_>,
    ) -> Result<()> {
        match curve {
            CurveKind::ControlPoints(curve) => match curve {
                ControlPointsCurveKind::Polyline(curve) => {
                    self.draw_control_points_curve(curve, properties, panel);
                }
                ControlPointsCurveKind::Interpolation(curve) => {
                    self.draw_control_points_curve(curve, properties, panel);
                }
                ControlPointsCurveKind::Bezier(curve) => {
                    self.draw_control_points_curve(curve, properties, panel);
                }
                ControlPointsCurveKind::RationalBezier(curve) => {
                    self.draw_control_points_curve(curve, properties, panel);
                }
                ControlPointsCurveKind::ConvexHull(curve) => {
                    self.draw_control_points_curve(curve, properties, panel);
                }
            },
            CurveKind::Formula(curve) => {
                let mut rasterizer = CurveRasterizer::new(curve, properties, panel);
                rasterizer.draw_curve();
            }
        }

        Ok(())
    }

    fn draw_control_points_curve<'a, C>(
        &self,
        curve: &C,
        properties: &'a CanvasProperties,
        panel: &'a mut Panel<'_>,
    ) where
        C: ToPath + GetControlPoints,
    {
        let mut rasterizer = CurveRasterizer::new(curve, properties, panel);
        rasterizer.draw_convex_hull();
        rasterizer.draw_curve();
        rasterizer.draw_control_points();
        rasterizer.draw_current_control_point();
    }
}

struct CurveRasterizer<'a, 'b, T> {
    curve: &'a T,
    properties: &'a CanvasProperties,
    panel: &'a mut Panel<'b>,
}

impl<'a, 'b, T> CurveRasterizer<'a, 'b, T> {
    pub fn new(curve: &'a T, properties: &'a CanvasProperties, panel: &'a mut Panel<'b>) -> Self {
        Self {
            curve,
            properties,
            panel,
        }
    }
}

impl<'a, 'b, T> CurveRasterizer<'a, 'b, T>
where
    T: ToPath,
{
    fn draw_curve(&mut self) {
        if let Some(path) = self.curve.to_path(TinySkiaPathConverter) {
            let paint = PaintBuilder::new()
                .color(self.properties.line_color)
                .build();
            let stroke = Stroke {
                width: self.properties.line_width,
                ..Stroke::default()
            };
            self.panel.draw_stroke_path(&path, &paint, &stroke);
        }
    }
}

impl<'a, 'b, T> CurveRasterizer<'a, 'b, T>
where
    T: ToPath + GetControlPoints,
{
    fn draw_convex_hull(&mut self) {
        if self.properties.show_convex_hull && self.curve.control_points().length() >= 3 {
            if let Some(path) = self.create_convex_hull_path() {
                let paint = PaintBuilder::new()
                    .color(self.properties.convex_hull_color)
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
                .color(self.properties.control_points_color)
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
                .color(self.properties.current_control_point_color)
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
            .map(AsRef::as_ref)
            .copied()
            .collect();
        ConvexHull::points_to_convex_hull_path(points, TinySkiaPathConverter)
    }

    fn create_points_path(&self, properties: &CanvasProperties) -> Option<Path> {
        let mut path = PathBuilder::new();

        for point in self.curve.control_points().iterator().map(AsRef::as_ref) {
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

impl<'a, 'b> CurveRasterizer<'a, 'b, CurvePoints> {}
