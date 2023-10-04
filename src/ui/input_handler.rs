use anyhow::Result;
use winit::keyboard::SmolStr;

use crate::command::program_view::ProgramView;
use crate::command::CommandState;
use crate::event::{input, EventHandler};
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

        let mut handler = self.state.frame.event_handler(self.state.mode);

        match self.command {
            CommandState::Closed(_) => {
                if let Some(event) = input.event {
                    match event {
                        InputEvent::ToggleConvexHull(event) => handler.handle(event)?,
                        InputEvent::ChangeWeight(event) => handler.handle(event)?,
                        InputEvent::MovePoint(event) => handler.handle(event)?,
                        InputEvent::MouseClick(event) => handler.handle(event)?,
                        InputEvent::MousePress(event) => handler.handle(event)?,
                        InputEvent::AddCurve(event) => handler.handle(event)?,
                        InputEvent::Delete(event) => handler.handle(event)?,
                        InputEvent::ChangeIndex(event) => handler.handle(event)?,
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
    ToggleConvexHull(input::ToggleConvexHull),
    ChangeWeight(input::ChangeWeight),
    MovePoint(input::MovePoint),
    MouseClick(input::MouseClick),
    MousePress(input::MousePress),
    AddCurve(input::Add),
    Delete(input::Delete),
    ChangeIndex(input::ChangeIndex),
    EnterCommand,
    ExecuteCommand,
    ExitMode,
    ChangeMode(Mode),
}
