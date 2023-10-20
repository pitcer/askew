use anyhow::Result;
use winit::keyboard::SmolStr;

use crate::command::program_view::ProgramView;
use crate::request::RequestHandlerMut;
use crate::ui::command_state::CommandState;
use crate::ui::frame::request::declare::{
    Add, ChangeIndex, ChangeWeight, Delete, MouseClick, MousePress, MovePoint, ToggleConvexHull,
};
use crate::ui::mode::Mode;

pub struct InputHandler<'a> {
    command: &'a mut CommandState,
    state: ProgramView<'a>,
}

impl<'a> InputHandler<'a> {
    pub fn new(command: &'a mut CommandState, state: ProgramView<'a>) -> Self {
        Self { command, state }
    }

    pub fn handle_input(self, input: Input) -> Result<()> {
        log::debug!("<cyan><b>Event received from input:</>\n<bright_black>{input:?}</>");

        let frame = &mut *self.state.frame;

        match self.command {
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
                        InputEvent::ChangeMode(mode) => self.change_mode(mode),
                        InputEvent::EnterCommand => {
                            self.command.open();
                            if let CommandState::Open(command) = self.command {
                                if let Some(text) = input.text {
                                    command.receive_text(&text);
                                }
                            }
                        }
                        InputEvent::ExitMode => self.exit_mode(),
                        event => {
                            log::debug!(
                                "<cyan><b>Cannot handle event in CommandClosed state:</> {event:?}"
                            );
                        }
                    }
                }
            }
            CommandState::Open(command) => match input.event {
                Some(InputEvent::ExecuteCommand) => self.command.execute(self.state),
                Some(InputEvent::ExitMode) => self.exit_mode(),
                _ => {
                    if let Some(text) = input.text {
                        command.receive_text(&text);
                    }
                }
            },
        }

        Ok(())
    }

    fn exit_mode(self) {
        if let CommandState::Closed(command) = self.command {
            command.clear_message();
            self.state.frame.mode_mut().exit();
        } else {
            self.command.close();
        }
    }

    fn change_mode(self, mode: Mode) {
        let mode_state = self.state.frame.mode_mut();
        match mode {
            Mode::Curve => mode_state.exit(),
            Mode::Point => mode_state.enter_point(),
            Mode::PointAdd => mode_state.enter_add(),
            Mode::PointSelect => mode_state.select(),
        }
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
