use crate::canvas::curve::control_points::kind::bezier::BezierAlgorithm;
use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;
use crate::canvas::curve::control_points::kind::rational_bezier::RationalBezierAlgorithm;
use crate::canvas::curve::formula::trochoid::TrochoidProperties;
use crate::config::rgb::Rgb;
use crate::config::CurveType;

pub trait Property {
    type Type;

    fn name(self) -> &'static str;

    fn value(self) -> Self::Type;
}

macro_rules! declare_properties {
    ($(
        $struct_name:ident($name:literal, $property_type:ty, $value:expr)
    ),+ $(,)?
    ) => {
        $(
        pub struct $struct_name;

        impl $crate::config::Property for $struct_name {
            type Type = $property_type;

            fn name(self) -> &'static str {
                $name
            }

            fn value(self) -> Self::Type {
                $value
            }
        }
        )*
    };
}

declare_properties! {
    ConvexHull("show_convex_hull", bool, false),
    InterpolationNodesProperty(
        "interpolation_nodes", InterpolationNodes, InterpolationNodes::Chebyshev
    ),
    LineWidth("line_width", f32, 2.0),
    Samples("samples", u32, 1000),
    DefaultWeight("default_weight", f32, 1.0),
    DefaultCurveType("curve_type", CurveType, CurveType::Polyline),
    DefaultBezierAlgorithm("bezier_algorithm", BezierAlgorithm, BezierAlgorithm::DeCasteljau),
    DefaultRationalBezierAlgorithm(
        "rational_bezier_algorithm",
        RationalBezierAlgorithm,
        RationalBezierAlgorithm::DeCasteljau
    ),
    DefaultTrochoidProperties("trochoid_properties", TrochoidProperties,
        TrochoidProperties::new((10.0 * -std::f32::consts::PI, 10.0 * std::f32::consts::PI),
            0.3, 0.8, 0.3, 0.7,)
    ),

    UiBackgroundColor("ui_background_color", Rgb, Rgb::new(32, 32, 32)),
    UiStatusBarColor("ui_status_bar_color", Rgb, Rgb::new(42, 42, 42)),
    UiCommandBarColor("ui_command_bar_color", Rgb, Rgb::new(42, 42, 42)),
    UiTextColor("ui_text_color", Rgb, Rgb::new(249, 250, 244)),
    UiTextErrorColor("ui_text_error_color", Rgb, Rgb::new(179, 26, 64)),
}
