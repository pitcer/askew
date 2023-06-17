use crate::event::input::command::{EnterCommand, ExecuteCommand, ExitMode, ReceiveCharacter};
use crate::event::{Error, EventHandler, HandlerResult};
use crate::ui::command::CommandState;
use crate::ui::frame::event_handler::FrameEventHandler;
use crate::ui::frame::mode::Mode;

impl EventHandler<EnterCommand> for FrameEventHandler<'_> {
    fn handle(&mut self, _event: EnterCommand) -> HandlerResult<EnterCommand> {
        self.frame.command.open();
        Ok(())
    }
}

impl EventHandler<ReceiveCharacter> for FrameEventHandler<'_> {
    fn handle(&mut self, event: ReceiveCharacter) -> HandlerResult<ReceiveCharacter> {
        if let CommandState::Open(command) = &mut self.frame.command {
            command.receive_character(event.0);
        }
        Ok(())
    }
}

impl EventHandler<ExecuteCommand> for FrameEventHandler<'_> {
    fn handle(&mut self, _event: ExecuteCommand) -> HandlerResult<ExecuteCommand> {
        let result = self.frame.command.execute(&mut self.frame.canvas);
        if let Some(event) = result {
            self.frame.receive_event(event).map_err(Error::Other)?;
            self.frame.window.request_redraw();
        }
        Ok(())
    }
}

impl EventHandler<ExitMode> for FrameEventHandler<'_> {
    fn handle(&mut self, _event: ExitMode) -> HandlerResult<ExitMode> {
        if let CommandState::Closed(command) = &mut self.frame.command {
            command.clear_message();
            self.frame.mode = Mode::Normal;
            self.frame.window.request_redraw();
        } else {
            self.frame.command.close();
        }
        Ok(())
    }
}
