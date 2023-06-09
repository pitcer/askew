use crate::event::Event;
use crate::ui::command::parser::Command;

#[derive(Debug)]
pub struct CommandInterpreter {
    command: Command,
}

impl CommandInterpreter {
    pub fn new(command: Command) -> Self {
        Self { command }
    }

    pub fn interpret(&mut self) -> Result<Option<Event>, Error> {
        Err(Error::UnknownCommand)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown command")]
    UnknownCommand,
}
