use anyhow::Result;

use crate::canvas::curve::Curve;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::rasterizer::Rasterizer;
use crate::config::Config;
use crate::ui::frame::mode::Mode;
use crate::ui::frame::panel::Panel;

pub mod curve;
pub mod event_handler;
pub mod math;
pub mod paint;
pub mod properties;
mod rasterizer;

#[derive(Debug)]
pub struct Canvas {
    rasterizer: Rasterizer,
    curves: Vec<Curve>,
    properties: CanvasProperties,
}

impl Canvas {
    #[must_use]
    pub fn new(area: Rectangle<f32>, curves: Vec<Curve>, config: &Config) -> Self {
        let mut properties = CanvasProperties::new(area);
        properties.include_config(config);
        Self {
            rasterizer: Rasterizer {},
            curves,
            properties,
        }
    }

    pub fn event_handler(&mut self, mode: Mode) -> CanvasEventHandler<'_> {
        CanvasEventHandler::new(self, mode)
    }

    pub fn rasterize(&self, mut panel: Panel<'_>) -> Result<()> {
        for curve in &self.curves {
            self.rasterizer
                .rasterize(curve, &self.properties, &mut panel)?;
        }
        Ok(())
    }

    pub fn current_curve_mut(&mut self) -> &mut Curve {
        &mut self.curves[self.properties.current_curve]
    }

    #[must_use]
    pub fn curves(&self) -> &Vec<Curve> {
        &self.curves
    }

    #[must_use]
    pub fn properties_mut(&mut self) -> &mut CanvasProperties {
        &mut self.properties
    }

    #[must_use]
    pub fn properties(&self) -> &CanvasProperties {
        &self.properties
    }
}
