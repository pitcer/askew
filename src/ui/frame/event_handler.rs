use winit::dpi::PhysicalPosition;

use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::event::canvas::{
    AddCurve, AddPoint, ChangeCurrentCurveIndex, ChangeCurrentPointIndex, ChangeCurrentPointWeight,
    DeleteCurrentPoint, DeleteCurve, GetConvexHull, GetCurrentPoint, GetCurveCenter,
    MoveCurrentPoint, MoveCurve, RotateCurve, SelectPoint, SetConvexHull,
};
use crate::event::input::{
    Add, ChangeIndex, ChangeWeight, Delete, MouseClick, MousePress, MovePoint, ToggleConvexHull,
};
use crate::event::{
    Change, DelegateEventHandler, DelegateEventHandlerMut, Direction, Event, EventHandlerMut,
    EventMut,
};
use crate::event::{EventHandler, HandlerResult};
use crate::ui::frame::Frame;
use crate::ui::mode::{Mode, ModeState};

pub struct CommandEventHandler<'a> {
    frame: &'a mut Frame,
    mode: &'a mut ModeState,
}

impl<'a> CommandEventHandler<'a> {
    pub fn new(frame: &'a mut Frame, mode: &'a mut ModeState) -> Self {
        Self { frame, mode }
    }
}

impl<'a, E> DelegateEventHandler<E> for CommandEventHandler<'a>
where
    E: Event,
    for<'b> CanvasEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = CanvasEventHandler<'b> where Self: 'b;

    fn delegate_handler(&self) -> Self::Delegate<'_> {
        self.frame.canvas.event_handler()
    }
}

impl<'a, E> DelegateEventHandlerMut<E> for CommandEventHandler<'a>
where
    E: EventMut,
    for<'b> CanvasEventHandler<'b>: EventHandlerMut<E>,
{
    type Delegate<'b> = CanvasEventHandler<'b> where Self: 'b;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_> {
        self.frame.canvas.event_handler()
    }
}

impl EventHandlerMut<ToggleConvexHull> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, _event: ToggleConvexHull) -> HandlerResult<ToggleConvexHull> {
        let convex_hull = self.delegate(GetConvexHull)?;
        self.delegate_mut(SetConvexHull(!convex_hull))?;
        Ok(())
    }
}

impl EventHandlerMut<ChangeWeight> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, event: ChangeWeight) -> HandlerResult<ChangeWeight> {
        let factor = match event.0 {
            Change::Decrease => 1.5,
            Change::Increase => -1.5,
        };
        match self.mode.as_mode() {
            Mode::Curve => {
                self.delegate_mut(RotateCurve::new(std::f32::consts::PI * factor * 4.0 / 180.0))?;
            }
            Mode::Point => {
                self.delegate_mut(ChangeCurrentPointWeight::new(factor))?;
            }
            Mode::PointSelect | Mode::PointAdd => {}
        }

        Ok(())
    }
}

impl EventHandlerMut<MouseClick> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, event: MouseClick) -> HandlerResult<MouseClick> {
        let click_point = scale_position(event.0);
        match self.mode.as_mode() {
            Mode::Curve => {
                let Some(center) = self.delegate(GetCurveCenter)? else {
                    return Ok(());
                };
                let shift = click_point - center;
                self.delegate_mut(MoveCurve::new(shift))?;
            }
            Mode::Point => {
                let point = self.delegate(GetCurrentPoint)?;
                let shift = click_point - point;
                self.delegate_mut(MoveCurrentPoint::new(shift))?;
            }
            Mode::PointAdd => {
                self.delegate_mut(AddPoint::new(click_point))?;
            }
            Mode::PointSelect => {
                let point = self.delegate(SelectPoint::new(
                    click_point,
                    self.frame.canvas.config.default_point_radius,
                ))?;
                if let Some(point) = point {
                    self.frame.canvas.properties_mut().current_point_index = point;
                }
            }
        }
        Ok(())
    }
}

impl EventHandlerMut<MousePress> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, event: MousePress) -> HandlerResult<MousePress> {
        let click_point = scale_position(event.0);
        match self.mode.as_mode() {
            Mode::Curve => {
                let Some(center) = self.delegate(GetCurveCenter)? else {
                    return Ok(());
                };
                let shift = click_point - center;
                self.delegate_mut(MoveCurve::new(shift))?;
            }
            Mode::Point => {
                let point = self.delegate(GetCurrentPoint)?;
                let shift = click_point - point;
                self.delegate_mut(MoveCurrentPoint::new(shift))?;
            }
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandlerMut<MovePoint> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, event: MovePoint) -> HandlerResult<MovePoint> {
        let direction = match event.0 {
            Direction::Up => Vector::new(0.0, -4.0),
            Direction::Down => Vector::new(0.0, 4.0),
            Direction::Left => Vector::new(-4.0, 0.0),
            Direction::Right => Vector::new(4.0, 0.0),
        };
        match self.mode.as_mode() {
            Mode::Curve => self.delegate_mut(MoveCurve::new(direction))?,
            Mode::Point => self.delegate_mut(MoveCurrentPoint::new(direction))?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandlerMut<Delete> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, _event: Delete) -> HandlerResult<Delete> {
        match self.mode.as_mode() {
            Mode::Curve => self.delegate_mut(DeleteCurve)?,
            Mode::Point => self.delegate_mut(DeleteCurrentPoint)?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandlerMut<ChangeIndex> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, event: ChangeIndex) -> HandlerResult<ChangeIndex> {
        let change = match event.0 {
            Change::Decrease => -1,
            Change::Increase => 1,
        };
        match self.mode.as_mode() {
            Mode::Curve => self.delegate_mut(ChangeCurrentCurveIndex::new(change))?,
            Mode::Point => self.delegate_mut(ChangeCurrentPointIndex::new(change))?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandlerMut<Add> for CommandEventHandler<'_> {
    fn handle_mut(&mut self, _event: Add) -> HandlerResult<Add> {
        match self.mode.as_mode() {
            Mode::Curve => self.delegate_mut(AddCurve)?,
            Mode::Point => self.mode.enter_add(),
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
    Point::new(position.x as f32, position.y as f32)
}
