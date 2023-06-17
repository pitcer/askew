use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::mode::Mode;
use crate::config::Config;
use crate::ui::color::Rgb;
use crate::ui::paint::PaintColor;

#[derive(Debug)]
pub struct CanvasProperties {
    pub area: Rectangle<f32>,
    pub mode: Mode,
    pub command_mode: bool,
    pub line_width: f32,
    pub point_radius: f32,
    pub show_convex_hull: bool,
    pub current_point_index: usize,
    pub current_curve: usize,
    pub default_weight: f32,
    pub line_color: PaintColor,
    pub convex_hull_color: PaintColor,
    pub control_points_color: PaintColor,
    pub current_control_point_color: PaintColor,
}

impl CanvasProperties {
    pub fn new(area: Rectangle<f32>) -> Self {
        Self {
            area,
            mode: Mode::Normal,
            command_mode: false,
            line_width: 0.0,
            point_radius: 0.0,
            show_convex_hull: false,
            current_point_index: 0,
            current_curve: 0,
            default_weight: 1.0,
            line_color: PaintColor::from_rgb(Rgb::new(0, 0, 0)),
            convex_hull_color: PaintColor::from_rgb(Rgb::new(0, 0, 0)),
            control_points_color: PaintColor::from_rgb(Rgb::new(0, 0, 0)),
            current_control_point_color: PaintColor::from_rgb(Rgb::new(0, 0, 0)),
        }
    }

    pub fn include_config(&mut self, config: &Config) {
        self.line_width = config.line_width;
        self.point_radius = config.point_radius;
        self.show_convex_hull = config.show_convex_hull;
        self.line_color = PaintColor::from_rgb(config.line_color);
        self.convex_hull_color = PaintColor::from_rgb(config.convex_hull_color);
        self.control_points_color = PaintColor::from_rgb(config.control_points_color);
        self.current_control_point_color = PaintColor::from_rgb(config.current_control_point_color);
    }
}
