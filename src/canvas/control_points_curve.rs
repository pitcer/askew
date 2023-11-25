use std::mem;

use tiny_skia::{PixmapMut, Point as SkiaPoint};

use crate::canvas::control_points::point::CurvePoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::math::convex_hull::GrahamScan;
use crate::canvas::shape::DrawOn;
use crate::canvas::visual_path::line::{VisualLine, VisualLineProperties};
use crate::canvas::visual_path::point::{VisualPoint, VisualPointProperties};
use crate::config::rgb::{Alpha, Rgb};
use crate::config::CanvasConfig;

pub type ControlLine = VisualLine<false>;
pub type ConvexHullLine = VisualLine<true>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VisualControlPoints {
    control_points: VisualPoint,
    // TODO: add current control point
    control_line: ControlLine,

    convex_hull: ConvexHullLine,
    #[serde(skip)]
    convex_hull_buffer: Vec<CurvePoint>,

    center_of_mass: VisualPoint,
}

impl VisualControlPoints {
    #[must_use]
    pub fn new(
        control_points: VisualPoint,
        control_line: ControlLine,
        convex_hull: ConvexHullLine,
        center_of_mass: VisualPoint,
    ) -> Self {
        let convex_hull_buffer = Vec::new();
        Self { control_points, control_line, convex_hull, convex_hull_buffer, center_of_mass }
    }

    pub fn rebuild_paths<P>(&mut self, points: &ControlPoints<P>)
    where
        P: Into<SkiaPoint> + Into<CurvePoint> + Copy,
    {
        self.control_points.rebuild_path(points.copied_iterator());
        self.control_line.rebuild_path(points.copied_iterator());

        self.rebuild_convex_hull_path(points);

        let center_of_mass = points.center_of_mass().map(SkiaPoint::from).into_iter();
        self.center_of_mass.rebuild_path(center_of_mass);
    }

    fn rebuild_convex_hull_path<P>(&mut self, points: &ControlPoints<P>)
    where
        P: Into<SkiaPoint> + Into<CurvePoint> + Copy,
    {
        self.convex_hull_buffer.clear();
        let points = points.copied_iterator().map(Into::<CurvePoint>::into);
        self.convex_hull_buffer.extend(points);
        let graham_scan = GrahamScan::new(mem::take(&mut self.convex_hull_buffer));
        self.convex_hull_buffer = graham_scan.convex_hull();
        let convex_hull = self.convex_hull_buffer.iter().copied().map(SkiaPoint::from);
        self.convex_hull.rebuild_path(convex_hull);
    }
}

impl DrawOn for VisualControlPoints {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.control_line.draw_on(pixmap);
        self.convex_hull.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
        self.center_of_mass.draw_on(pixmap);
    }
}

impl Default for VisualControlPoints {
    fn default() -> Self {
        Self {
            control_points: VisualPoint::new(
                true,
                VisualPointProperties::new(4.0, Rgb::WHITE, Alpha::OPAQUE),
            ),
            control_line: VisualLine::new(
                false,
                VisualLineProperties::new(4.0, Rgb::WHITE, Alpha::OPAQUE),
            ),
            convex_hull: VisualLine::new(
                false,
                VisualLineProperties::new(4.0, Rgb::WHITE, Alpha::OPAQUE),
            ),
            convex_hull_buffer: Vec::new(),
            center_of_mass: VisualPoint::new(
                true,
                VisualPointProperties::new(4.0, Rgb::WHITE, Alpha::OPAQUE),
            ),
        }
    }
}

impl From<&CanvasConfig> for VisualControlPoints {
    fn from(value: &CanvasConfig) -> Self {
        Self {
            control_points: VisualPoint::new(
                true,
                VisualPointProperties::new(
                    value.default_point_radius,
                    value.control_points_color,
                    Alpha::OPAQUE,
                ),
            ),
            control_line: VisualLine::new(
                value.show_control_line,
                VisualLineProperties::new(
                    value.default_line_width,
                    value.convex_hull_color,
                    Alpha::OPAQUE,
                ),
            ),
            convex_hull: VisualLine::new(
                value.show_convex_hull,
                VisualLineProperties::new(
                    value.default_line_width,
                    value.convex_hull_color,
                    Alpha::OPAQUE,
                ),
            ),
            convex_hull_buffer: Vec::new(),
            center_of_mass: VisualPoint::new(
                value.show_center_of_mass,
                VisualPointProperties::new(
                    value.default_point_radius * 2.0,
                    Rgb::new(0, 255, 0),
                    Alpha::OPAQUE,
                ),
            ),
        }
    }
}
