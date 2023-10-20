use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::canvas::shape::bezier::BezierCurveAlgorithm;
use crate::canvas::shape::interpolation::InterpolationNodes;
use crate::canvas::shape::rational_bezier::RationalBezierCurveAlgorithm;
use crate::canvas::shape::trochoid::TrochoidCurveProperties;
use crate::cli::RunArguments;
use crate::config::property::{
    Property, UiBackgroundColor, UiCommandBarColor, UiStatusBarColor, UiTextColor, UiTextErrorColor,
};
use crate::config::rgb::Rgb;

pub mod property;
pub mod rgb;
pub mod trochoid_properties;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub startup_commands: Vec<String>,
    pub ipc_socket_path: PathBuf,

    #[serde(flatten)]
    pub frame: FrameConfig,

    pub canvas: CanvasConfig,
    pub ui: UiConfig,
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let config = if path.exists() {
            let config_toml = std::fs::read_to_string(path)?;
            toml::from_str(&config_toml)?
        } else {
            let config = Config::default();
            let config_toml = toml::to_string_pretty(&config)?;
            std::fs::write(path, config_toml)?;
            config
        };
        Ok(config)
    }

    pub fn overwrite_with_run(&mut self, arguments: RunArguments) {
        if !arguments.startup_commands.is_empty() {
            self.startup_commands = arguments.startup_commands;
        }
        if let Some(random_points) = arguments.random_points {
            self.frame.generate_random_points = random_points;
        }
        if let Some(project_path) = arguments.project_path {
            self.frame.project_to_open_path = Some(project_path);
        }
        if let Some(canvas_curve_samples) = arguments.canvas_curve_samples {
            self.canvas.curve_samples = canvas_curve_samples;
        }
        if let Some(background_image_path) = arguments.background_image_path {
            self.frame.background_to_load_path = Some(background_image_path);
        }
        if let Some(font_size) = arguments.font_size {
            self.ui.font_size = font_size;
        }
        if let Some(font_path) = arguments.font_path {
            self.ui.font_path = font_path;
        }
        if let Some(ipc_socket_path) = arguments.ipc_socket_path {
            self.ipc_socket_path = ipc_socket_path;
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            startup_commands: vec![],
            ipc_socket_path: PathBuf::from("/tmp/askew.socket"),
            frame: FrameConfig::default(),
            canvas: CanvasConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct FrameConfig {
    pub generate_random_points: u32,

    pub background_to_load_path: Option<PathBuf>,

    pub project_to_open_path: Option<PathBuf>,

    pub default_project_save_path: PathBuf,

    pub default_image_save_path: PathBuf,
}

impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            generate_random_points: 0,
            background_to_load_path: None,
            project_to_open_path: None,
            default_project_save_path: PathBuf::from("askew_project.json"),
            default_image_save_path: PathBuf::from("askew_canvas.png"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct CanvasConfig {
    pub show_convex_hull: bool,

    pub show_control_line: bool,

    pub show_center_of_mass: bool,

    pub curve_samples: u32,

    pub default_curve_type: ShapeType,

    pub default_bezier_algorithm: BezierCurveAlgorithm,

    pub default_rational_bezier_algorithm: RationalBezierCurveAlgorithm,

    pub default_interpolation_nodes: InterpolationNodes,

    pub default_trochoid_properties: TrochoidCurveProperties,

    pub default_rational_bezier_weight: f32,

    pub default_line_width: f32,

    pub default_point_radius: f32,

    #[serde(with = "rgb::serde_pretty")]
    pub line_color: Rgb,

    #[serde(with = "rgb::serde_pretty")]
    pub convex_hull_color: Rgb,

    #[serde(with = "rgb::serde_pretty")]
    pub control_points_color: Rgb,

    #[serde(with = "rgb::serde_pretty")]
    pub current_control_point_color: Rgb,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            show_convex_hull: false,
            show_control_line: false,
            show_center_of_mass: true,
            curve_samples: 1000,
            default_curve_type: ShapeType::Polyline,
            default_bezier_algorithm: BezierCurveAlgorithm::ChudyWozny,
            default_rational_bezier_algorithm: RationalBezierCurveAlgorithm::ChudyWozny,
            default_interpolation_nodes: InterpolationNodes::Chebyshev,
            default_trochoid_properties: TrochoidCurveProperties::default(),
            default_rational_bezier_weight: 1.0,
            default_line_width: 2.0,
            default_point_radius: 4.0,
            line_color: Rgb::new(255, 255, 0),
            convex_hull_color: Rgb::new(0, 255, 255),
            control_points_color: Rgb::new(255, 0, 255),
            current_control_point_color: Rgb::new(255, 255, 255),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct UiConfig {
    pub font_path: PathBuf,

    pub font_size: u32,

    #[serde(with = "rgb::serde_pretty")]
    pub background_color: Rgb,

    #[serde(with = "rgb::serde_pretty")]
    pub status_bar_color: Rgb,

    #[serde(with = "rgb::serde_pretty")]
    pub command_bar_color: Rgb,

    #[serde(with = "rgb::serde_pretty")]
    pub text_color: Rgb,

    #[serde(with = "rgb::serde_pretty")]
    pub text_error_color: Rgb,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_path: PathBuf::from("JetBrainsMonoNL-Regular.ttf"),
            background_color: UiBackgroundColor.value(),
            status_bar_color: UiStatusBarColor.value(),
            command_bar_color: UiCommandBarColor.value(),
            text_color: UiTextColor.value(),
            text_error_color: UiTextErrorColor.value(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
pub enum ShapeType {
    #[default]
    Polyline,
    Interpolation,
    Bezier,
    RationalBezier,
    Trochoid,
    RegularPolygon,
}

impl Display for ShapeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ShapeType::Polyline => write!(f, "Polyline"),
            ShapeType::Interpolation => write!(f, "Interpolation"),
            ShapeType::Bezier => write!(f, "Bezier"),
            ShapeType::RationalBezier => write!(f, "RationalBezier"),
            ShapeType::Trochoid => write!(f, "Trochoid"),
            ShapeType::RegularPolygon => write!(f, "RegularPolygon"),
        }
    }
}
