use std::mem;

use tiny_skia::{PixmapMut, Point as SkiaPoint};

use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::math::convex_hull::GrahamScan;
use crate::canvas::v2::visual_path::line::{VisualLine, VisualLineProperties};
use crate::canvas::v2::visual_path::point::{VisualPoint, VisualPointProperties};
use crate::canvas::v2::DrawOn;
use crate::config::rgb::Rgb;
use crate::config::CanvasConfig;

pub type ControlLine = VisualLine<false>;
pub type ConvexHullLine = VisualLine<true>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ControlPointsCurve<P> {
    pub points: ControlPoints<P>,

    pub control_points: VisualPoint,
    pub control_line: ControlLine,

    pub convex_hull: ConvexHullLine,
    #[serde(skip)]
    convex_hull_buffer: Vec<CurvePoint>,

    pub center_of_mass: VisualPoint,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ControlPointsCurveProperties {
    pub control_points: VisualPoint,
    pub control_line: ControlLine,
    pub convex_hull: ConvexHullLine,
    pub center_of_mass: VisualPoint,
}

impl<P> ControlPointsCurve<P> {
    #[must_use]
    pub fn new(
        points: ControlPoints<P>,
        control_points: VisualPoint,
        control_line: ControlLine,
        convex_hull: ConvexHullLine,
        center_of_mass: VisualPoint,
    ) -> Self {
        let convex_hull_buffer = Vec::new();
        Self {
            points,
            control_points,
            control_line,
            convex_hull,
            convex_hull_buffer,
            center_of_mass,
        }
    }

    #[must_use]
    pub fn from_config(points: ControlPoints<P>, config: &CanvasConfig) -> Self {
        Self {
            points,
            control_points: VisualPoint::new(VisualPointProperties::new(
                true,
                config.default_point_radius,
                config.control_points_color,
            )),
            control_line: VisualLine::new(VisualLineProperties::new(
                config.show_control_line,
                config.default_line_width,
                config.convex_hull_color,
            )),
            convex_hull: VisualLine::new(VisualLineProperties::new(
                config.show_convex_hull,
                config.default_line_width,
                config.convex_hull_color,
            )),
            convex_hull_buffer: Vec::new(),
            center_of_mass: VisualPoint::new(VisualPointProperties::new(
                config.show_center_of_mass,
                config.default_point_radius * 2.0,
                Rgb::new(0, 255, 0),
            )),
        }
    }
}

impl<P> ControlPointsCurve<P>
where
    P: Into<SkiaPoint> + Copy,
    CurvePoint: From<P>,
{
    pub fn rebuild_paths(&mut self) {
        self.control_points.rebuild_path(self.points.copied_iterator());
        self.control_line.rebuild_path(self.points.copied_iterator());

        self.rebuild_convex_hull_path();

        let center_of_mass = self.points.center_of_mass().map(SkiaPoint::from).into_iter();
        self.center_of_mass.rebuild_path(center_of_mass);
    }

    fn rebuild_convex_hull_path(&mut self) {
        self.convex_hull_buffer.clear();
        let points = self.points.copied_iterator().map(|point| CurvePoint::from(point));
        self.convex_hull_buffer.extend(points);
        let graham_scan = GrahamScan::new(mem::take(&mut self.convex_hull_buffer));
        self.convex_hull_buffer = graham_scan.convex_hull();
        let convex_hull = self.convex_hull_buffer.iter().copied().map(SkiaPoint::from);
        self.convex_hull.rebuild_path(convex_hull);
    }
}

impl<P> DrawOn for ControlPointsCurve<P> {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) {
        self.control_line.draw_on(pixmap);
        self.convex_hull.draw_on(pixmap);
        self.control_points.draw_on(pixmap);
        self.center_of_mass.draw_on(pixmap);
    }
}

impl<P> Default for ControlPointsCurve<P>
where
    P: Default,
{
    fn default() -> Self {
        Self {
            points: ControlPoints::default(),
            control_points: VisualPoint::new(VisualPointProperties::new(true, 4.0, Rgb::WHITE)),
            control_line: VisualLine::new(VisualLineProperties::new(false, 4.0, Rgb::WHITE)),
            convex_hull: VisualLine::new(VisualLineProperties::new(false, 4.0, Rgb::WHITE)),
            convex_hull_buffer: Vec::new(),
            center_of_mass: VisualPoint::new(VisualPointProperties::new(true, 4.0, Rgb::WHITE)),
        }
    }
}
