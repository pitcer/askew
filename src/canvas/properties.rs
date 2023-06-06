use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::mode::Mode;
use crate::command::Command;

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
        }
    }

    pub fn include_command(&mut self, command: &Command) {
        self.line_width = command.line_width;
        self.point_radius = command.point_radius;
        self.show_convex_hull = command.show_convex_hull;
    }
}
