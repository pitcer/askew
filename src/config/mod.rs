use std::path::PathBuf;

use rgb::Rgb;

use crate::config::property::{ChebyshevNodes, ConvexHull, LineWidth, Property, Samples};

pub mod property;
pub mod rgb;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Config {
    #[arg(short, long, value_enum, default_value_t = CurveType::Polyline)]
    pub curve_type: CurveType,

    #[arg(
        short = 'e', long = ChebyshevNodes.name(),
        default_value_t = ChebyshevNodes.default_value()
    )]
    pub chebyshev_nodes: <ChebyshevNodes as Property>::Type,

    #[arg(short = 'H', long = ConvexHull.name(), default_value_t = ConvexHull.default_value())]
    pub show_convex_hull: <ConvexHull as Property>::Type,

    #[arg(short, long = Samples.name(), default_value_t = Samples.default_value())]
    pub samples: <Samples as Property>::Type,

    #[arg(short, long = LineWidth.name(), default_value_t = LineWidth.default_value())]
    pub line_width: <LineWidth as Property>::Type,

    #[arg(short, long, default_value_t = 4.0)]
    pub point_radius: f32,

    #[arg(short = 'f', long, value_enum)]
    pub save_format: Option<SaveFormat>,

    #[arg(short, long)]
    pub background_path: Option<String>,

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
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CurveType {
    Polyline,
    Interpolation,
    Bezier,
    RationalBezier,
    Trochoid,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum SaveFormat {
    Png,
}
