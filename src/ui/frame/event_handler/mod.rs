use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::vector::Vector;
use crate::event::canvas::{
    AddCurve, AddPoint, ChangeCurrentCurveIndex, ChangeCurrentPointIndex, ChangeCurrentPointWeight,
    DeleteCurrentPoint, DeleteCurve, GetConvexHull, MoveCurrentPoint, SetConvexHull,
};
use crate::event::input::{
    ChangeIndex, ChangeMode, ChangeWeight, Delete, MovePoint, ToggleConvexHull,
};
use crate::event::{Change, Direction, Event};
use crate::event::{EventHandler, HandlerResult};
use crate::ui::frame::mode::Mode;
use crate::ui::frame::Frame;

pub mod command;

pub struct FrameEventHandler<'a> {
    frame: &'a mut Frame,
}

impl<'a> FrameEventHandler<'a> {
    pub fn new(frame: &'a mut Frame) -> Self {
        Self { frame }
    }

    fn canvas_handle<'b, E>(&'b mut self, event: E) -> HandlerResult<E>
    where
        CanvasEventHandler<'b>: EventHandler<E>,
        E: Event,
    {
        let mut handler = self.frame.canvas.event_handler(self.frame.mode);
        handler.handle(event)
    }
}

impl EventHandler<ToggleConvexHull> for FrameEventHandler<'_> {
    fn handle(&mut self, _event: ToggleConvexHull) -> HandlerResult<ToggleConvexHull> {
        let convex_hull = self.canvas_handle(GetConvexHull)?;
        self.canvas_handle(SetConvexHull(!convex_hull))?;
        Ok(())
    }
}

impl EventHandler<ChangeWeight> for FrameEventHandler<'_> {
    fn handle(&mut self, event: ChangeWeight) -> HandlerResult<ChangeWeight> {
        let factor = match event.0 {
            Change::Decrease => 1.5,
            Change::Increase => -1.5,
        };
        self.canvas_handle(ChangeCurrentPointWeight::new(factor))?;
        Ok(())
    }
}

impl EventHandler<MovePoint> for FrameEventHandler<'_> {
    fn handle(&mut self, event: MovePoint) -> HandlerResult<MovePoint> {
        let direction = match event.0 {
            Direction::Up => Vector::new(0.0, -4.0),
            Direction::Down => Vector::new(0.0, 4.0),
            Direction::Left => Vector::new(-4.0, 0.0),
            Direction::Right => Vector::new(4.0, 0.0),
        };
        self.canvas_handle(MoveCurrentPoint::new(direction))?;
        Ok(())
    }
}

impl EventHandler<AddPoint> for FrameEventHandler<'_> {
    fn handle(&mut self, event: AddPoint) -> HandlerResult<AddPoint> {
        self.canvas_handle(event)
    }
}

impl EventHandler<AddCurve> for FrameEventHandler<'_> {
    fn handle(&mut self, event: AddCurve) -> HandlerResult<AddCurve> {
        self.canvas_handle(event)
    }
}

impl EventHandler<Delete> for FrameEventHandler<'_> {
    fn handle(&mut self, _event: Delete) -> HandlerResult<Delete> {
        match self.frame.mode {
            Mode::Normal => self.canvas_handle(DeleteCurve)?,
            Mode::Curve => self.canvas_handle(DeleteCurrentPoint)?,
        }
        Ok(())
    }
}

impl EventHandler<ChangeIndex> for FrameEventHandler<'_> {
    fn handle(&mut self, event: ChangeIndex) -> HandlerResult<ChangeIndex> {
        let change = match event.0 {
            Change::Decrease => -1,
            Change::Increase => 1,
        };
        match self.frame.mode {
            Mode::Normal => self.canvas_handle(ChangeCurrentCurveIndex::new(change))?,
            Mode::Curve => self.canvas_handle(ChangeCurrentPointIndex::new(change))?,
        }
        Ok(())
    }
}

impl EventHandler<ChangeMode> for FrameEventHandler<'_> {
    fn handle(&mut self, event: ChangeMode) -> HandlerResult<ChangeMode> {
        self.frame.mode = event.0;
        Ok(())
    }
}
