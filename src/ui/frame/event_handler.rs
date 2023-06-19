use winit::dpi::PhysicalPosition;

use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::event::canvas::{
    AddCurve, AddPoint, ChangeCurrentCurveIndex, ChangeCurrentPointIndex, ChangeCurrentPointWeight,
    DeleteCurrentPoint, DeleteCurve, GetConvexHull, MoveCurrentPoint, SetConvexHull,
};
use crate::event::input::{
    ChangeIndex, ChangeWeight, Delete, MouseClick, MovePoint, ToggleConvexHull,
};
use crate::event::macros::delegate_handlers;
use crate::event::{Change, DelegateEventHandler, Direction, Event};
use crate::event::{EventHandler, HandlerResult};
use crate::ui::frame::Frame;
use crate::ui::mode::Mode;

pub struct CommandEventHandler<'a> {
    frame: &'a mut Frame,
    mode: Mode,
}

impl<'a> CommandEventHandler<'a> {
    pub fn new(frame: &'a mut Frame, mode: Mode) -> Self {
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
        fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
            Point::new(position.x as f32, position.y as f32)
        }
        self.delegate(AddPoint::new(scale_position(event.0)))
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
        self.delegate(MoveCurrentPoint::new(direction))?;
        Ok(())
    }
}

impl EventHandler<Delete> for CommandEventHandler<'_> {
    fn handle(&mut self, _event: Delete) -> HandlerResult<Delete> {
        match self.mode {
            Mode::Curve => self.delegate(DeleteCurve)?,
            Mode::Point => self.delegate(DeleteCurrentPoint)?,
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
        match self.mode {
            Mode::Curve => self.delegate(ChangeCurrentCurveIndex::new(change))?,
            Mode::Point => self.delegate(ChangeCurrentPointIndex::new(change))?,
        }
        Ok(())
    }
}

delegate_handlers! {
    CommandEventHandler<'_> {
        AddCurve,
    }
}
