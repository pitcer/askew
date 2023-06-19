use anyhow::Result;

use crate::event::{canvas, input, EventHandler};
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

    pub fn handle_input(mut self, event: InputEvent) -> Result<()> {
        log::debug!("Event received from input: {event:?}");

        let command_closed = self.command.is_closed();
        let mode = self.state.mode.as_mode();
        let mut handler = self.state.frame.event_handler(mode);
        match event {
            InputEvent::ToggleConvexHull(event) if command_closed => handler.handle(event)?,
            InputEvent::ChangeWeight(event) if command_closed => handler.handle(event)?,
            InputEvent::MovePoint(event) if command_closed => handler.handle(event)?,
            InputEvent::MouseClick(event) if command_closed => handler.handle(event)?,
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

    fn receive_character(&mut self, character: char) {
        if let CommandState::Open(command) = &mut self.command {
            command.receive_character(character);
        }
    }

    fn exit_mode(&mut self) {
        if let CommandState::Closed(command) = &mut self.command {
            command.clear_message();
            self.state.mode.exit();
        } else {
            self.command.close();
        }
    }

    fn change_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Curve => self.state.mode.exit(),
            Mode::Point => self.state.mode.enter_point(),
        }
    }

    pub fn handle_mouse_press(self) {}

    pub fn handle_key_press(self) {}
}

#[derive(Debug)]
pub enum InputEvent {
    ToggleConvexHull(input::ToggleConvexHull),
    ChangeWeight(input::ChangeWeight),
    MovePoint(input::MovePoint),
    MouseClick(input::MouseClick),
    AddCurve(canvas::AddCurve),
    Delete(input::Delete),
    ChangeIndex(input::ChangeIndex),
    EnterCommand,
    ReceiveCharacter(char),
    ExecuteCommand,
    ExitMode,
    ChangeMode(Mode),
}
