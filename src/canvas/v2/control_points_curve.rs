use anyhow::Result;
use tiny_skia::PixmapMut;

use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::v2::visual_path::line::{VisualLine, VisualLineProperties};
use crate::canvas::v2::visual_path::point::{VisualPoint, VisualPointProperties};
use crate::canvas::v2::DrawOn;
use crate::config::rgb::Rgb;
use crate::config::CanvasConfig;

pub type ControlLine = VisualLine<false>;
pub type ConvexHullLine = VisualLine<true>;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ControlPointsCurve<P> {
    pub points: ControlPoints<P>,
    pub properties: ControlPointsCurveProperties,
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
    pub fn new(points: ControlPoints<P>, properties: ControlPointsCurveProperties) -> Self {
        Self { points, properties }
    }
}

impl<P> ControlPointsCurve<P>
where
    P: Into<CurvePoint> + Copy,
{
    pub fn rebuild_paths(&mut self) -> Result<()> {
        let _ = self.properties.control_points.rebuild_path(self.points.copied_iterator())?;
        let _ = self.properties.control_line.rebuild_path(self.points.copied_iterator())?;
        let _ = self.properties.convex_hull.rebuild_path(self.points.copied_iterator())?;
        let _ = self
            .properties
            .center_of_mass
            .rebuild_path(self.points.center_of_mass().iter().copied())?;
        Ok(())
    }
}

impl<P> DrawOn for ControlPointsCurve<P> {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) -> Result<()> {
        self.properties.control_line.draw_on(pixmap)?;
        self.properties.convex_hull.draw_on(pixmap)?;
        self.properties.control_points.draw_on(pixmap)?;
        self.properties.center_of_mass.draw_on(pixmap)?;
        Ok(())
    }
}

impl ControlPointsCurveProperties {
    #[must_use]
    pub fn new(
        control_points: VisualPoint,
        control_line: ControlLine,
        convex_hull: ConvexHullLine,
        center_of_mass: VisualPoint,
    ) -> Self {
        Self { control_points, control_line, convex_hull, center_of_mass }
    }

    #[must_use]
    pub fn from_config(config: &CanvasConfig) -> Self {
        Self {
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
            center_of_mass: VisualPoint::new(VisualPointProperties::new(
                config.show_center_of_mass,
                config.default_point_radius * 2.0,
                Rgb::new(0, 255, 0),
            )),
        }
    }
}

impl Default for ControlPointsCurveProperties {
    fn default() -> Self {
        Self {
            control_points: VisualPoint::new(VisualPointProperties::new(true, 4.0, Rgb::WHITE)),
            control_line: VisualLine::new(VisualLineProperties::new(false, 4.0, Rgb::WHITE)),
            convex_hull: VisualLine::new(VisualLineProperties::new(false, 4.0, Rgb::WHITE)),
            center_of_mass: VisualPoint::new(VisualPointProperties::new(true, 4.0, Rgb::WHITE)),
        }
    }
}
