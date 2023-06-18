use crate::canvas::curve::control_points::kind::bezier::BezierAlgorithm;
use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;
use crate::canvas::curve::control_points::kind::rational_bezier::RationalBezierAlgorithm;
use crate::canvas::curve::formula::trochoid::TrochoidProperties;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::paint::PaintColor;
use crate::config::{Config, CurveType};

#[derive(Debug)]
pub struct CanvasProperties {
    pub area: Rectangle<f32>,
    pub current_point_index: usize,
    pub current_curve: usize,
    pub default_curve_type: CurveType,
    pub bezier_algorithm: BezierAlgorithm,
    pub rational_bezier_algorithm: RationalBezierAlgorithm,
    pub trochoid_properties: TrochoidProperties,
    pub line_width: f32,
    pub point_radius: f32,
    pub show_convex_hull: bool,
    pub interpolation_nodes: InterpolationNodes,
    pub default_weight: f32,
    pub samples: u32,
    pub line_color: PaintColor,
    pub convex_hull_color: PaintColor,
    pub control_points_color: PaintColor,
    pub current_control_point_color: PaintColor,
}

impl CanvasProperties {
    #[must_use]
    pub fn new(area: Rectangle<f32>, config: &Config) -> Self {
        Self {
            area,
            current_point_index: 0,
            current_curve: 0,
            default_curve_type: config.curve_type,
            bezier_algorithm: config.bezier_algorithm,
            rational_bezier_algorithm: config.rational_bezier_algorithm,
            trochoid_properties: config.trochoid_properties,
            line_width: config.line_width,
            point_radius: config.point_radius,
            show_convex_hull: config.show_convex_hull,
            interpolation_nodes: config.interpolation_nodes,
            default_weight: config.default_weight,
            samples: config.samples,
            line_color: PaintColor::from_rgb(config.line_color),
            convex_hull_color: PaintColor::from_rgb(config.convex_hull_color),
            control_points_color: PaintColor::from_rgb(config.control_points_color),
            current_control_point_color: PaintColor::from_rgb(config.current_control_point_color),
        }
    }
}
