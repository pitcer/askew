use crate::canvas::curve::control_points::kind::bezier::BezierCurveAlgorithm;
use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;
use crate::canvas::curve::control_points::kind::rational_bezier::RationalBezierAlgorithm;
use crate::canvas::curve::formula::trochoid::TrochoidProperties;
use crate::config::rgb::Rgb;
use crate::config::{CanvasConfig, CurveType};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CanvasProperties {
    pub current_point_index: usize,
    pub current_curve: usize,
    #[deprecated]
    pub default_curve_type: CurveType,
    #[deprecated]
    pub bezier_algorithm: BezierCurveAlgorithm,
    #[deprecated]
    pub rational_bezier_algorithm: RationalBezierAlgorithm,
    #[deprecated]
    pub trochoid_properties: TrochoidProperties,
    #[deprecated]
    pub line_width: f32,
    #[deprecated]
    pub point_radius: f32,
    #[deprecated]
    pub control_line: bool,
    #[deprecated]
    pub show_convex_hull: bool,
    #[deprecated]
    pub show_center_of_mass: bool,
    #[deprecated]
    pub interpolation_nodes: InterpolationNodes,
    #[deprecated]
    pub default_weight: f32,
    #[deprecated]
    pub samples: u32,
    #[deprecated]
    pub line_color: Rgb,
    #[deprecated]
    pub convex_hull_color: Rgb,
    #[deprecated]
    pub control_points_color: Rgb,
    #[deprecated]
    pub current_control_point_color: Rgb,
}

impl CanvasProperties {
    #[must_use]
    pub fn new(config: &CanvasConfig) -> Self {
        Self {
            current_point_index: 0,
            current_curve: 0,
            // TODO: separate runtime properties from default config properties (e.g. simply use
            // CanvasConfig to store them and here store only runtime ones).
            default_curve_type: config.default_curve_type,
            bezier_algorithm: config.default_bezier_algorithm,
            rational_bezier_algorithm: config.default_rational_bezier_algorithm,
            trochoid_properties: config.default_trochoid_properties,
            line_width: config.default_line_width,
            point_radius: config.default_point_radius,
            control_line: config.show_control_line,
            show_convex_hull: config.show_convex_hull,
            show_center_of_mass: config.show_center_of_mass,
            interpolation_nodes: config.default_interpolation_nodes,
            default_weight: config.default_rational_bezier_weight,
            samples: config.curve_samples,
            line_color: config.line_color,
            convex_hull_color: config.convex_hull_color,
            control_points_color: config.control_points_color,
            current_control_point_color: config.current_control_point_color,
        }
    }
}
