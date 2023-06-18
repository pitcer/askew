use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::event::canvas::{
    AddCurve, AddPoint, ChangeCurrentCurveIndex, ChangeCurrentPointIndex, ChangeCurrentPointWeight,
    DeleteCurrentPoint, DeleteCurve, GetConvexHull, MoveCurrentPoint, SetConvexHull,
};
use crate::event::input::{
    ChangeIndex, ChangeMode, ChangeWeight, Delete, EnterCommand, ExecuteCommand, ExitMode,
    MouseClick, MovePoint, ReceiveCharacter, ToggleConvexHull,
};
use crate::event::macros::delegate_handlers;
use crate::event::{Change, DelegateEventHandler, Direction, Error, Event};
use crate::event::{EventHandler, HandlerResult};
use crate::ui::command::CommandState;
use crate::ui::frame::mode::{Mode, ModeState};
use crate::ui::frame::Frame;
use winit::dpi::PhysicalPosition;

pub struct InputEventHandler<'a> {
    frame: &'a mut Frame,
}

impl<'a> InputEventHandler<'a> {
    pub fn new(frame: &'a mut Frame) -> Self {
        Self { frame }
    }
}

impl<'a, E> DelegateEventHandler<E> for InputEventHandler<'a>
where
    E: Event,
    for<'b> CanvasEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = CanvasEventHandler<'b> where Self: 'b;

    fn delegate_handler(&mut self) -> Self::Delegate<'_> {
        self.frame.canvas.event_handler()
    }
}

impl EventHandler<ToggleConvexHull> for InputEventHandler<'_> {
    fn handle(&mut self, _event: ToggleConvexHull) -> HandlerResult<ToggleConvexHull> {
        let convex_hull = self.delegate(GetConvexHull)?;
        self.delegate(SetConvexHull(!convex_hull))?;
        Ok(())
    }
}

impl EventHandler<ChangeWeight> for InputEventHandler<'_> {
    fn handle(&mut self, event: ChangeWeight) -> HandlerResult<ChangeWeight> {
        let factor = match event.0 {
            Change::Decrease => 1.5,
            Change::Increase => -1.5,
        };
        self.delegate(ChangeCurrentPointWeight::new(factor))?;
        Ok(())
    }
}

impl EventHandler<MouseClick> for InputEventHandler<'_> {
    fn handle(&mut self, event: MouseClick) -> HandlerResult<MouseClick> {
        fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
            Point::new(position.x as f32, position.y as f32)
        }
        self.delegate(AddPoint::new(scale_position(event.0)))
    }
}

impl EventHandler<MovePoint> for InputEventHandler<'_> {
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

impl EventHandler<Delete> for InputEventHandler<'_> {
    fn handle(&mut self, _event: Delete) -> HandlerResult<Delete> {
        match self.frame.mode {
            ModeState::Curve(_) => self.delegate(DeleteCurve)?,
            ModeState::Point(_) => self.delegate(DeleteCurrentPoint)?,
        }
        Ok(())
    }
}

impl EventHandler<ChangeIndex> for InputEventHandler<'_> {
    fn handle(&mut self, event: ChangeIndex) -> HandlerResult<ChangeIndex> {
        let change = match event.0 {
            Change::Decrease => -1,
            Change::Increase => 1,
        };
        match self.frame.mode {
            ModeState::Curve(_) => self.delegate(ChangeCurrentCurveIndex::new(change))?,
            ModeState::Point(_) => self.delegate(ChangeCurrentPointIndex::new(change))?,
        }
        Ok(())
    }
}

impl EventHandler<ChangeMode> for InputEventHandler<'_> {
    fn handle(&mut self, event: ChangeMode) -> HandlerResult<ChangeMode> {
        match event.0 {
            Mode::Curve => self.frame.mode.exit(),
            Mode::Point => self.frame.mode.enter_point(),
        }
        Ok(())
    }
}

impl EventHandler<EnterCommand> for InputEventHandler<'_> {
    fn handle(&mut self, _event: EnterCommand) -> HandlerResult<EnterCommand> {
        self.frame.command.open();
        Ok(())
    }
}

impl EventHandler<ReceiveCharacter> for InputEventHandler<'_> {
    fn handle(&mut self, event: ReceiveCharacter) -> HandlerResult<ReceiveCharacter> {
        if let CommandState::Open(command) = &mut self.frame.command {
            command.receive_character(event.0);
        }
        Ok(())
    }
}

impl EventHandler<ExecuteCommand> for InputEventHandler<'_> {
    fn handle(&mut self, _event: ExecuteCommand) -> HandlerResult<ExecuteCommand> {
        let result = self.frame.command.execute(&mut self.frame.canvas);
        if let Some(event) = result {
            self.frame.receive_event(event).map_err(Error::Other)?;
            self.frame.window.request_redraw();
        }
        Ok(())
    }
}

impl EventHandler<ExitMode> for InputEventHandler<'_> {
    fn handle(&mut self, _event: ExitMode) -> HandlerResult<ExitMode> {
        if let CommandState::Closed(command) = &mut self.frame.command {
            command.clear_message();
            self.frame.mode.exit();
            self.frame.window.request_redraw();
        } else {
            self.frame.command.close();
        }
        Ok(())
    }
}

delegate_handlers! {
    InputEventHandler<'_> {
        AddCurve,
    }
}
