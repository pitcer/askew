use winit::dpi::PhysicalPosition;

use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::event::canvas::{
    AddCurve, AddPoint, ChangeCurrentCurveIndex, ChangeCurrentPointIndex, ChangeCurrentPointWeight,
    DeleteCurrentPoint, DeleteCurve, GetConvexHull, GetCurrentPoint, GetCurveCenter,
    MoveCurrentPoint, MoveCurve, SelectPoint, SetConvexHull,
};
use crate::event::input::{
    Add, ChangeIndex, ChangeWeight, Delete, MouseClick, MousePress, MovePoint, ToggleConvexHull,
};
use crate::event::{Change, DelegateEventHandler, Direction, Event};
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

    fn delegate_handler(&mut self) -> Self::Delegate<'_> {
        self.frame.canvas.event_handler()
    }
}

impl EventHandler<ToggleConvexHull> for CommandEventHandler<'_> {
    fn handle(&mut self, _event: ToggleConvexHull) -> HandlerResult<ToggleConvexHull> {
        let convex_hull = self.delegate(GetConvexHull)?;
        self.delegate(SetConvexHull(!convex_hull))?;
        Ok(())
    }
}

impl EventHandler<ChangeWeight> for CommandEventHandler<'_> {
    fn handle(&mut self, event: ChangeWeight) -> HandlerResult<ChangeWeight> {
        let factor = match event.0 {
            Change::Decrease => 1.5,
            Change::Increase => -1.5,
        };
        self.delegate(ChangeCurrentPointWeight::new(factor))?;
        Ok(())
    }
}

impl EventHandler<MouseClick> for CommandEventHandler<'_> {
    fn handle(&mut self, event: MouseClick) -> HandlerResult<MouseClick> {
        let click_point = scale_position(event.0);
        match self.mode.as_mode() {
            Mode::Curve => {
                let Some(center) = self.delegate(GetCurveCenter)? else { return Ok(()) };
                let shift = click_point - center;
                self.delegate(MoveCurve::new(shift))?;
            }
            Mode::Point => {
                let point = self.delegate(GetCurrentPoint)?;
                let shift = click_point - point;
                self.delegate(MoveCurrentPoint::new(shift))?;
            }
            Mode::PointAdd => {
                self.delegate(AddPoint::new(click_point))?;
            }
            Mode::PointSelect => {
                let point = self.delegate(SelectPoint::new(
                    click_point,
                    self.frame.canvas.properties().point_radius,
                ))?;
                if let Some(point) = point {
                    self.frame.canvas.properties_mut().current_point_index = point;
                }
            }
        }
        Ok(())
    }
}

impl EventHandler<MousePress> for CommandEventHandler<'_> {
    fn handle(&mut self, event: MousePress) -> HandlerResult<MousePress> {
        let click_point = scale_position(event.0);
        match self.mode.as_mode() {
            Mode::Curve => {
                let Some(center) = self.delegate(GetCurveCenter)? else { return Ok(()) };
                let shift = click_point - center;
                self.delegate(MoveCurve::new(shift))?;
            }
            Mode::Point => {
                let point = self.delegate(GetCurrentPoint)?;
                let shift = click_point - point;
                self.delegate(MoveCurrentPoint::new(shift))?;
            }
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandler<MovePoint> for CommandEventHandler<'_> {
    fn handle(&mut self, event: MovePoint) -> HandlerResult<MovePoint> {
        let direction = match event.0 {
            Direction::Up => Vector::new(0.0, -4.0),
            Direction::Down => Vector::new(0.0, 4.0),
            Direction::Left => Vector::new(-4.0, 0.0),
            Direction::Right => Vector::new(4.0, 0.0),
        };
        match self.mode.as_mode() {
            Mode::Curve => self.delegate(MoveCurve::new(direction))?,
            Mode::Point => self.delegate(MoveCurrentPoint::new(direction))?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandler<Delete> for CommandEventHandler<'_> {
    fn handle(&mut self, _event: Delete) -> HandlerResult<Delete> {
        match self.mode.as_mode() {
            Mode::Curve => self.delegate(DeleteCurve)?,
            Mode::Point => self.delegate(DeleteCurrentPoint)?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandler<ChangeIndex> for CommandEventHandler<'_> {
    fn handle(&mut self, event: ChangeIndex) -> HandlerResult<ChangeIndex> {
        let change = match event.0 {
            Change::Decrease => -1,
            Change::Increase => 1,
        };
        match self.mode.as_mode() {
            Mode::Curve => self.delegate(ChangeCurrentCurveIndex::new(change))?,
            Mode::Point => self.delegate(ChangeCurrentPointIndex::new(change))?,
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

impl EventHandler<Add> for CommandEventHandler<'_> {
    fn handle(&mut self, _event: Add) -> HandlerResult<Add> {
        match self.mode.as_mode() {
            Mode::Curve => self.delegate(AddCurve)?,
            Mode::Point => self.mode.enter_add(),
            Mode::PointAdd | Mode::PointSelect => {}
        }
        Ok(())
    }
}

fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
    Point::new(position.x as f32, position.y as f32)
}
