#[derive(clap::Parser)]
#[command(version)]
pub struct Command {
    #[arg(value_enum, short, long, default_value_t = CurveType::Polyline)]
    pub curve_type: CurveType,

    #[arg(short, long, default_value_t = 5000)]
    pub interpolation_samples: u32,

    #[arg(short, long, default_value_t = 2.0)]
    pub line_width: f32,

    #[arg(short, long, default_value_t = 4.0)]
    pub point_radius: f32,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum CurveType {
    Polyline,
    Interpolation,
    Trochoid,
}
