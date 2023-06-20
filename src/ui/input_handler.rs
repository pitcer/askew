use anyhow::Result;

use crate::event::{input, EventHandler};
use crate::ui::command::CommandState;
use crate::ui::mode::Mode;
use crate::ui::state::ProgramState;

pub struct InputHandler<'a> {
    command: &'a mut CommandState,
    state: ProgramState<'a>,
}

impl<'a> InputHandler<'a> {
    pub fn new(command: &'a mut CommandState, state: ProgramState<'a>) -> Self {
        Self { command, state }
    }

    pub fn handle_input(self, event: InputEvent) -> Result<()> {
        log::debug!("Event received from input: {event:?}");

        let command_closed = self.command.is_closed();
        let mut handler = self.state.frame.event_handler(self.state.mode);
        match event {
            InputEvent::ToggleConvexHull(event) if command_closed => handler.handle(event)?,
            InputEvent::ChangeWeight(event) if command_closed => handler.handle(event)?,
            InputEvent::MovePoint(event) if command_closed => handler.handle(event)?,
            InputEvent::MouseClick(event) if command_closed => handler.handle(event)?,
            InputEvent::MousePress(event) if command_closed => handler.handle(event)?,
            InputEvent::AddCurve(event) if command_closed => handler.handle(event)?,
            InputEvent::Delete(event) if command_closed => handler.handle(event)?,
            InputEvent::ChangeIndex(event) if command_closed => handler.handle(event)?,

            InputEvent::ChangeMode(mode) if command_closed => self.change_mode(mode),
            InputEvent::EnterCommand => self.command.open(),
            InputEvent::ReceiveCharacter(character) => self.receive_character(character),
            InputEvent::ExecuteCommand => self.command.execute(self.state),
            InputEvent::ExitMode => self.exit_mode(),

            _ => {}
        }

        Ok(())
    }

    fn receive_character(self, character: char) {
        if let CommandState::Open(command) = self.command {
            command.receive_character(character);
        }
    }

    fn exit_mode(self) {
        if let CommandState::Closed(command) = self.command {
            command.clear_message();
            self.state.mode.exit();
        } else {
            self.command.close();
        }
    }

    fn change_mode(self, mode: Mode) {
        match mode {
            Mode::Curve => self.state.mode.exit(),
            Mode::Point => self.state.mode.enter_point(),
            Mode::PointAdd => self.state.mode.enter_add(),
            Mode::PointSelect => self.state.mode.select(),
        }
    }
}

#[derive(Debug)]
pub enum InputEvent {
    ToggleConvexHull(input::ToggleConvexHull),
    ChangeWeight(input::ChangeWeight),
    MovePoint(input::MovePoint),
    MouseClick(input::MouseClick),
    MousePress(input::MousePress),
    AddCurve(input::Add),
    Delete(input::Delete),
    ChangeIndex(input::ChangeIndex),
    EnterCommand,
    ReceiveCharacter(char),
    ExecuteCommand,
    ExitMode,
    ChangeMode(Mode),
}
