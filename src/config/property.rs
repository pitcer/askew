use crate::canvas::shape::bezier::BezierCurveAlgorithm;
use crate::canvas::shape::interpolation::InterpolationNodes;
use crate::canvas::shape::rational_bezier::RationalBezierCurveAlgorithm;
use crate::canvas::shape::trochoid::TrochoidCurveProperties;
use crate::config::rgb::Rgb;
use crate::config::ShapeType;

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
    ControlLine("control_line", bool, false),
    InterpolationNodesProperty(
        "interpolation_nodes", InterpolationNodes, InterpolationNodes::Chebyshev
    ),
    LineWidth("line_width", f32, 2.0),
    Samples("samples", u32, 1000),
    DefaultWeight("default_weight", f32, 1.0),
    DefaultCurveType("curve_type", ShapeType, ShapeType::Polyline),
    DefaultBezierAlgorithm("bezier_algorithm", BezierCurveAlgorithm, BezierCurveAlgorithm::DeCasteljau),
    DefaultRationalBezierAlgorithm(
        "rational_bezier_algorithm",
        RationalBezierCurveAlgorithm,
        RationalBezierCurveAlgorithm::DeCasteljau
    ),
    DefaultTrochoidProperties("trochoid_properties", TrochoidCurveProperties,
        TrochoidCurveProperties::new(10.0 * -std::f32::consts::PI, 10.0 * std::f32::consts::PI,
            0.3, 0.8, 0.3, 0.7,)
    ),

    UiBackgroundColor("ui_background_color", Rgb, Rgb::new(32, 32, 32)),
    UiStatusBarColor("ui_status_bar_color", Rgb, Rgb::new(42, 42, 42)),
    UiCommandBarColor("ui_command_bar_color", Rgb, Rgb::new(42, 42, 42)),
    UiTextColor("ui_text_color", Rgb, Rgb::new(249, 250, 244)),
    UiTextErrorColor("ui_text_error_color", Rgb, Rgb::new(179, 26, 64)),
}
