use std::cell::RefCell;
use std::rc::Rc;
use crate::canvas::curve::control_points::polyline::Polyline;
use crate::canvas::curve::control_points::{ControlPointsCurve, CurvePoints};
use anyhow::Result;

use crate::canvas::curve::Curve;
use crate::canvas::event_handler::EventHandler;
use crate::canvas::math::rectangle::Rectangle;
use crate::canvas::mode::Mode;
use crate::canvas::properties::CanvasProperties;
use crate::canvas::rasterizer::Rasterizer;
use crate::command::Command;
use crate::event::{CanvasEvent, CurveEvent};
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
        match event {
            CanvasEvent::ChangeMode(mode) => {
                self.properties.mode = mode;
                Ok(())
            }
            CanvasEvent::Add => {
                self.curves
                    .push(Curve::ControlPoints(ControlPointsCurve::Polyline(
                        Polyline::new(CurvePoints::new(vec![])),
                    )));
                self.properties.current_curve += 1;
                Ok(())
            }
            CanvasEvent::Delete => {
                if self.properties.mode == Mode::Curve {
                    self.event_handler.handle_event(
                        &mut self.curves[self.properties.current_curve],
                        &mut self.properties,
                        CurveEvent::DeleteCurrentPoint,
                    )?;
                }
                Ok(())
            }
            CanvasEvent::Curve(event) => self.event_handler.handle_event(
                &mut self.curves[self.properties.current_curve],
                &mut self.properties,
                event,
            ),
            CanvasEvent::ChangeIndex(change) => {
                if self.properties.mode == Mode::Curve {
                    self.event_handler.handle_event(
                        &mut self.curves[self.properties.current_curve],
                        &mut self.properties,
                        CurveEvent::ChangeCurrentIndex(change),
                    )?;
                } else {
                    self.change_current_index(change as isize)?;
                }
                Ok(())
            }
        }
    }

    fn change_current_index(&mut self, change: isize) -> Result<()> {
        let i = self.properties.current_curve as isize + change;
        let v = self.curves.len() as isize;
        let r = i % v;
        self.properties.current_curve = if r < 0 {
            if v < 0 {
                r - v
            } else {
                r + v
            }
        } else {
            r
        } as usize;
        Ok(())
    }

    pub fn rasterize(&self, panel: Panel<'_>) -> Result<()> {
        let rc = Rc::new(RefCell::new(panel));
        for curve in &self.curves {
            self.rasterizer.rasterize(curve, &self.properties, rc.clone())?;
        }
        Ok(())
    }

    pub fn curves(&self) -> &Vec<Curve> {
        &self.curves
    }

    pub fn properties(&self) -> &CanvasProperties {
        &self.properties
    }
}
