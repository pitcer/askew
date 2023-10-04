use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use rgb::Rgb;

use crate::config::property::{
    ControlLine, ConvexHull, DefaultBezierAlgorithm, DefaultCurveType,
    DefaultRationalBezierAlgorithm, DefaultTrochoidProperties, DefaultWeight,
    InterpolationNodesProperty, LineWidth, Property, Samples, UiBackgroundColor, UiCommandBarColor,
    UiStatusBarColor, UiTextColor, UiTextErrorColor,
};

pub mod property;
pub mod rgb;
pub mod trochoid_properties;

#[derive(Debug, clap::Args)]
pub struct Config {
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,

    #[arg(
        short, long = DefaultCurveType.name(),
        value_enum, default_value_t = DefaultCurveType.value()
    )]
    pub curve_type: <DefaultCurveType as Property>::Type,

    #[arg(
        long = DefaultBezierAlgorithm.name(),
        value_enum, default_value_t = DefaultBezierAlgorithm.value()
    )]
    pub bezier_algorithm: <DefaultBezierAlgorithm as Property>::Type,

    #[arg(
        long = DefaultRationalBezierAlgorithm.name(),
        value_enum, default_value_t = DefaultRationalBezierAlgorithm.value()
    )]
    pub rational_bezier_algorithm: <DefaultRationalBezierAlgorithm as Property>::Type,

    #[arg(
        long = DefaultTrochoidProperties.name(),
        default_value_t = DefaultTrochoidProperties.value(),
        value_parser = trochoid_properties::parse,
    )]
    pub trochoid_properties: <DefaultTrochoidProperties as Property>::Type,

    #[arg(
        short = 'e', long = InterpolationNodesProperty.name(),
        value_enum, default_value_t = InterpolationNodesProperty.value()
    )]
    pub interpolation_nodes: <InterpolationNodesProperty as Property>::Type,

    #[arg(short = 'H', long = ConvexHull.name(), default_value_t = ConvexHull.value())]
    pub show_convex_hull: <ConvexHull as Property>::Type,

    #[arg(short = 'L', long = ControlLine.name(), default_value_t = ControlLine.value())]
    pub show_control_line: <ControlLine as Property>::Type,

    #[arg(short, long = Samples.name(), default_value_t = Samples.value())]
    pub samples: <Samples as Property>::Type,

    #[arg(long = DefaultWeight.name(), default_value_t = DefaultWeight.value())]
    pub default_weight: <DefaultWeight as Property>::Type,

    #[arg(short, long = LineWidth.name(), default_value_t = LineWidth.value())]
    pub line_width: <LineWidth as Property>::Type,

    #[arg(short, long, default_value_t = 4.0)]
    pub point_radius: f32,

    #[arg(short = 'f', long, value_enum)]
    pub save_format: Option<SaveFormat>,

    #[arg(short, long)]
    pub background_path: Option<PathBuf>,

    #[arg(long)]
    pub open_path: Option<PathBuf>,

    /// Command to execute on start, can be specified multiple times
    #[arg(long)]
    pub command: Vec<String>,

    #[arg(short = 'n', long, default_value_t = 0)]
    pub random_points: u32,

    #[arg(long, default_value_t = 16)]
    pub font_size: u32,

    #[arg(long, default_value = "JetBrainsMonoNL-Regular.ttf")]
    pub font_path: PathBuf,

    #[arg(long, default_value = "#ffff00", value_parser = Rgb::parse)]
    pub line_color: Rgb,

    #[arg(long, default_value = "#00ffff", value_parser = Rgb::parse)]
    pub convex_hull_color: Rgb,

    #[arg(long, default_value = "#ff00ff", value_parser = Rgb::parse)]
    pub control_points_color: Rgb,

    #[arg(long, default_value = "#ffffff", value_parser = Rgb::parse)]
    pub current_control_point_color: Rgb,

    #[arg(long, default_value = "/tmp/askew.socket")]
    pub ipc_path: PathBuf,

    #[arg(
        long = UiBackgroundColor.name(), default_value_t = UiBackgroundColor.value(),
        value_parser = Rgb::parse
    )]
    pub ui_background_color: <UiBackgroundColor as Property>::Type,

    #[arg(
        long = UiStatusBarColor.name(), default_value_t = UiStatusBarColor.value(),
        value_parser = Rgb::parse
    )]
    pub ui_status_bar_color: <UiStatusBarColor as Property>::Type,

    #[arg(
        long = UiCommandBarColor.name(), default_value_t = UiCommandBarColor.value(),
        value_parser = Rgb::parse
    )]
    pub ui_command_bar_color: <UiCommandBarColor as Property>::Type,

    #[arg(
        long = UiTextColor.name(), default_value_t = UiTextColor.value(),
        value_parser = Rgb::parse
    )]
    pub ui_text_color: <UiTextColor as Property>::Type,

    #[arg(
        long = UiTextErrorColor.name(), default_value_t = UiTextErrorColor.value(),
        value_parser = Rgb::parse
    )]
    pub ui_text_error_color: <UiTextErrorColor as Property>::Type,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum CurveType {
    Polyline,
    ConvexHull,
    Interpolation,
    Bezier,
    RationalBezier,
    Trochoid,
}

impl Display for CurveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CurveType::Polyline => write!(f, "Polyline"),
            CurveType::ConvexHull => write!(f, "ConvexHull"),
            CurveType::Interpolation => write!(f, "Interpolation"),
            CurveType::Bezier => write!(f, "Bezier"),
            CurveType::RationalBezier => write!(f, "RationalBezier"),
            CurveType::Trochoid => write!(f, "Trochoid"),
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum SaveFormat {
    Png,
}
