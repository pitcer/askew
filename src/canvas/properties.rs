use crate::canvas::math::point::Point;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::math::size::Size;
use crate::command::Command;

pub struct CanvasProperties {
    pub area: Rectangle<f32>,
    pub line_width: f32,
    pub point_radius: f32,
    pub show_convex_hull: bool,
    pub current_point_index: usize,
    pub current_curve: usize,
    pub default_weight: f32,
}

impl CanvasProperties {}

pub struct CanvasPropertiesBuilder {
    area: Rectangle<f32>,
    line_width: f32,
    point_radius: f32,
    show_convex_hull: bool,
    current_point_index: usize,
    current_curve: usize,
    default_weight: f32,
}

impl CanvasPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            area: Rectangle::new(Point::new(0.0, 0.0), Size::new(0.0, 0.0)),
            line_width: 0.0,
            point_radius: 0.0,
            show_convex_hull: false,
            current_point_index: 0,
            current_curve: 0,
            default_weight: 1.0,
        }
    }

    pub fn include_command(mut self, command: &Command) -> Self {
        self.line_width = command.line_width;
        self.point_radius = command.point_radius;
        self.show_convex_hull = command.show_convex_hull;
        self
    }

    pub fn build(self) -> CanvasProperties {
        CanvasProperties {
            area: self.area,
            line_width: self.line_width,
            point_radius: self.point_radius,
            show_convex_hull: self.show_convex_hull,
            current_point_index: self.current_point_index,
            current_curve: self.current_curve,
            default_weight: self.default_weight,
        }
    }
}
