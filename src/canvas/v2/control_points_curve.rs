use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::v2::visual_path::point::VisualPoint;
use crate::canvas::v2::visual_path::line::VisualLine;

#[derive(Debug, Clone)]
pub struct ControlPointsCurve<P> {
    points: ControlPoints<P>,
    properties: ControlPointsCurveProperties,
}

#[derive(Debug, Clone)]
pub struct ControlPointsCurveProperties {
    pub control_points: VisualPoint,
    pub control_line: VisualLine,
    pub convex_hull: VisualLine,
    pub center_of_mass: VisualPoint,
}

impl<P> ControlPointsCurve<P> {
    pub fn new(points: ControlPoints<P>, properties: ControlPointsCurveProperties) -> Self {
        Self { points, properties }
    }
}

impl ControlPointsCurveProperties {
    pub fn new(
        control_points: VisualPoint,
        control_line: VisualLine,
        convex_hull: VisualLine,
        center_of_mass: VisualPoint,
    ) -> Self {
        Self { control_points, control_line, convex_hull, center_of_mass }
    }
}
