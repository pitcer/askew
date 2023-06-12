use std::path::PathBuf;

use crate::ui::color::Rgb;

pub mod rgb_parser;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Config {
    #[arg(short, long, value_enum, default_value_t = CurveType::Polyline)]
    pub curve_type: CurveType,

    #[arg(short = 'e', long, default_value_t = false)]
    pub chebyshev_nodes: bool,

    #[arg(short = 'H', long, default_value_t = false)]
    pub show_convex_hull: bool,

    #[arg(short, long, default_value_t = 5000)]
    pub samples: u32,

    #[arg(short, long, default_value_t = 2.0)]
    pub line_width: f32,

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

    #[arg(long, default_value = "#ffff00", value_parser = rgb_parser::parse_rgb)]
    pub line_color: Rgb,

    #[arg(long, default_value = "#00ffff", value_parser = rgb_parser::parse_rgb)]
    pub convex_hull_color: Rgb,

    #[arg(long, default_value = "#ff00ff", value_parser = rgb_parser::parse_rgb)]
    pub control_points_color: Rgb,

    #[arg(long, default_value = "#ffffff", value_parser = rgb_parser::parse_rgb)]
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
