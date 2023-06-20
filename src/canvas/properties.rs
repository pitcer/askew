use crate::canvas::curve::control_points::kind::bezier::BezierAlgorithm;
use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;
use crate::canvas::curve::control_points::kind::rational_bezier::RationalBezierAlgorithm;
use crate::canvas::curve::formula::trochoid::TrochoidProperties;
use crate::config::rgb::Rgb;
use crate::config::{Config, CurveType};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CanvasProperties {
    pub current_point_index: usize,
    pub current_curve: usize,
    pub default_curve_type: CurveType,
    pub bezier_algorithm: BezierAlgorithm,
    pub rational_bezier_algorithm: RationalBezierAlgorithm,
    pub trochoid_properties: TrochoidProperties,
    pub line_width: f32,
    pub point_radius: f32,
    pub control_line: bool,
    pub show_convex_hull: bool,
    pub show_center_of_mass: bool,
    pub interpolation_nodes: InterpolationNodes,
    pub default_weight: f32,
    pub samples: u32,
    pub line_color: Rgb,
    pub convex_hull_color: Rgb,
    pub control_points_color: Rgb,
    pub current_control_point_color: Rgb,
}

impl CanvasProperties {
    #[must_use]
    pub fn new(config: &Config) -> Self {
        Self {
            current_point_index: 0,
            current_curve: 0,
            default_curve_type: config.curve_type,
            bezier_algorithm: config.bezier_algorithm,
            rational_bezier_algorithm: config.rational_bezier_algorithm,
            trochoid_properties: config.trochoid_properties,
            line_width: config.line_width,
            point_radius: config.point_radius,
            control_line: config.show_control_line,
            show_convex_hull: config.show_convex_hull,
            // TODO: use config variable
            show_center_of_mass: true,
            interpolation_nodes: config.interpolation_nodes,
            default_weight: config.default_weight,
            samples: config.samples,
            line_color: (config.line_color),
            convex_hull_color: (config.convex_hull_color),
            control_points_color: (config.control_points_color),
            current_control_point_color: (config.current_control_point_color),
        }
    }
}
