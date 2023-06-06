use anyhow::Result;

use crate::canvas::curve::Curve;
use crate::canvas::event_handler::EventHandler;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::rasterizer::Rasterizer;
use crate::command::Command;
use crate::event::CanvasEvent;
use crate::ui::panel::Panel;

pub mod curve;
mod event_handler;
pub mod math;
pub mod mode;
mod properties;
mod rasterizer;

pub struct Canvas {
    rasterizer: Rasterizer,
    event_handler: EventHandler,
    curves: Vec<Curve>,
    properties: CanvasProperties,
}

impl Canvas {
    pub fn new(area: Rectangle<f32>, curves: Vec<Curve>, command: &Command) -> Self {
        let mut properties = CanvasProperties::new(area);
        properties.include_command(command);
        Self {
            rasterizer: Rasterizer {},
            event_handler: EventHandler {},
            curves,
            properties,
        }
    }

    pub fn handle_event(&mut self, event: CanvasEvent) -> Result<()> {
        self.event_handler.handle_event(
            &mut self.curves[self.properties.current_curve],
            &mut self.properties,
            event,
        )
    }

    pub fn rasterize(&self, panel: Panel<'_>) -> Result<()> {
        self.rasterizer.rasterize(
            &self.curves[self.properties.current_curve],
            &self.properties,
            panel,
        )
    }

    pub fn curves(&self) -> &Vec<Curve> {
        &self.curves
    }

    pub fn properties(&self) -> &CanvasProperties {
        &self.properties
    }
}
