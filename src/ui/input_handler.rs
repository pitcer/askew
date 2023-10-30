use anyhow::Result;
use winit::keyboard::SmolStr;

use crate::command::program_view::ProgramView;
use crate::request::RequestHandlerMut;
use crate::ui::command_state::CommandState;
use crate::ui::frame::request::declare::{
    Add, ChangeIndex, ChangeWeight, Delete, MouseClick, MousePress, MovePoint, ToggleConvexHull,
};
use crate::ui::mode::Mode;

#[derive(Debug, Default)]
pub struct InputHandler {
    command: CommandState,
}

impl InputHandler {
    #[must_use]
    pub fn new() -> Self {
        let command = CommandState::new();
        Self { command }
    }

    pub fn handle_input(&mut self, input: Input, state: ProgramView<'_>) -> Result<()> {
        log::debug!("<cyan><b>Event received from input:</>\n<bright_black>{input:?}</>");

        let frame = &mut state.state.frame;

        match &mut self.command {
            CommandState::Closed(_) => {
                if let Some(event) = input.event {
                    match event {
                        InputEvent::ToggleConvexHull(event) => frame.handle_mut(event)?,
                        InputEvent::ChangeWeight(event) => frame.handle_mut(event)?,
                        InputEvent::MovePoint(event) => frame.handle_mut(event)?,
                        InputEvent::MouseClick(event) => frame.handle_mut(event)?,
                        InputEvent::MousePress(event) => frame.handle_mut(event)?,
                        InputEvent::AddCurve(event) => frame.handle_mut(event)?,
                        InputEvent::Delete(event) => frame.handle_mut(event)?,
                        InputEvent::ChangeIndex(event) => frame.handle_mut(event)?,
                        InputEvent::ChangeMode(mode) => self.change_mode(mode, state),
                        InputEvent::EnterCommand => {
                            self.command.open();
                            if let CommandState::Open(command) = &mut self.command {
                                if let Some(text) = input.text {
                                    command.receive_text(&text);
                                }
                            }
                        }
                        InputEvent::ExitMode => self.exit_mode(state),
                        event => {
                            log::debug!(
                                "<cyan><b>Cannot handle event in CommandClosed state:</> {event:?}"
                            );
                        }
                    }
                }
            }
            CommandState::Open(command) => match input.event {
                Some(InputEvent::ExecuteCommand) => self.command.execute(state),
                Some(InputEvent::ExitMode) => self.exit_mode(state),
                _ => {
                    if let Some(text) = input.text {
                        command.receive_text(&text);
                    }
                }
            },
        }

        Ok(())
    }

    fn exit_mode(&mut self, state: ProgramView<'_>) {
        if let CommandState::Closed(command) = &mut self.command {
            command.clear_message();
            state.state.frame.mode_mut().exit();
        } else {
            self.command.close();
        }
    }

    fn change_mode(&mut self, mode: Mode, state: ProgramView<'_>) {
        let mode_state = state.state.frame.mode_mut();
        match mode {
            Mode::Curve => mode_state.exit(),
            Mode::Point => mode_state.enter_point(),
            Mode::PointAdd => mode_state.enter_add(),
            Mode::PointSelect => mode_state.select(),
        }
    }

    #[must_use]
    pub fn command(&self) -> &CommandState {
        &self.command
    }
}

#[derive(Debug)]
pub struct Input {
    event: Option<InputEvent>,
    text: Option<SmolStr>,
}

impl Input {
    #[must_use]
    pub fn new(event: Option<InputEvent>, text: Option<SmolStr>) -> Self {
        Self { event, text }
    }
}

#[derive(Debug)]
pub enum InputEvent {
    ToggleConvexHull(ToggleConvexHull),
    ChangeWeight(ChangeWeight),
    MovePoint(MovePoint),
    MouseClick(MouseClick),
    MousePress(MousePress),
    AddCurve(Add),
    Delete(Delete),
    ChangeIndex(ChangeIndex),
    EnterCommand,
    ExecuteCommand,
    ExitMode,
    ChangeMode(Mode),
}
