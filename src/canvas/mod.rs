use anyhow::Result;

use crate::canvas::curve::CurveKind;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::rasterizer::Rasterizer;
use crate::config::Config;
use crate::ui::frame::panel::Panel;

pub mod curve;
pub mod event_handler;
pub mod math;
pub mod paint;
pub mod properties;
mod rasterizer;

#[derive(Debug)]
pub struct Canvas {
    curves: Vec<CurveKind>,
    properties: CanvasProperties,
}

impl Canvas {
    #[must_use]
    pub fn new(area: Rectangle<f32>, curves: Vec<CurveKind>, config: &Config) -> Self {
        let mut properties = CanvasProperties::new(area);
        properties.include_config(config);
        Self { curves, properties }
    }

    pub fn event_handler(&mut self) -> CanvasEventHandler<'_> {
        CanvasEventHandler::new(self)
    }

    pub fn rasterize(&self, mut panel: Panel<'_>) -> Result<()> {
        for curve in &self.curves {
            Rasterizer.rasterize(curve, &self.properties, &mut panel)?;
        }
        Ok(())
    }

    pub fn current_curve_mut(&mut self) -> &mut CurveKind {
        &mut self.curves[self.properties.current_curve]
    }

    #[must_use]
    pub fn curves(&self) -> &Vec<CurveKind> {
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
