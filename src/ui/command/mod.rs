use std::fmt::{Debug, Write};

use anyhow::Result;

use message::{Message, MessageType};

use crate::ui::command::interpreter::CommandInterpreter;
use crate::ui::command::parser::CommandParser;
use crate::ui::state::ProgramState;

pub mod interpreter;
pub mod message;
pub mod parser;

#[derive(Debug)]
pub enum CommandState {
    Closed(CommandClosed),
    Open(CommandOpen),
}

impl CommandState {
    #[must_use]
    pub fn initial() -> Self {
        CommandState::Closed(CommandClosed::new(None))
    }

    pub fn open(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            CommandState::Closed(command) => CommandState::Open(command.open()),
            other => other,
        });
    }

    pub fn close(&mut self) {
        replace_with::replace_with_or_abort(self, |state| match state {
            CommandState::Open(command) => CommandState::Closed(command.close()),
            other => other,
        });
    }

    pub fn execute(&mut self, program_state: ProgramState<'_>) {
        replace_with::replace_with_or_abort(self, |state| match state {
            CommandState::Open(command) => CommandState::Closed(command.execute(program_state)),
            other => other,
        });
    }

    #[must_use]
    pub fn is_closed(&self) -> bool {
        matches!(self, CommandState::Closed(_))
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        matches!(self, CommandState::Open(_))
    }
}

#[derive(Debug)]
pub struct CommandClosed {
    message: Option<Message>,
}

impl CommandClosed {
    #[must_use]
    pub fn new(message: Option<Message>) -> Self {
        Self { message }
    }

    #[must_use]
    pub fn open(self) -> CommandOpen {
        let mut message = self.message.map(Message::into_text).unwrap_or_default();
        message.clear();
        CommandOpen::new(message)
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    #[must_use]
    pub fn message(&self) -> &Option<Message> {
        &self.message
    }
}

#[derive(Debug)]
pub struct CommandOpen {
    buffer: String,
}

impl CommandOpen {
    #[must_use]
    pub fn new(buffer: String) -> Self {
        Self { buffer }
    }

    pub fn receive_character(&mut self, character: char) {
        if character == '\u{8}' {
            self.buffer.pop();
        } else if !character.is_control() {
            self.buffer.push(character);
        }
    }

    #[must_use]
    pub fn execute(mut self, state: ProgramState<'_>) -> CommandClosed {
        let result = self.execute_command(state);
        let message = result.unwrap_or_else(|error| {
            self.buffer.clear();
            self.buffer
                .write_fmt(format_args!("{error}"))
                .expect("formatting should not fail");
            let message = Message::new(self.buffer, MessageType::Error);
            Some(message)
        });
        CommandClosed::new(message)
    }

    fn execute_command(&self, state: ProgramState<'_>) -> Result<Option<Message>> {
        let input = &self.buffer[1..];
        let mut parser = CommandParser::new(input);
        let result = parser.parse()?;
        let mut interpreter = CommandInterpreter::new(state);
        let message = interpreter.interpret(result)?;
        Ok(message)
    }

    #[must_use]
    pub fn close(self) -> CommandClosed {
        CommandClosed::new(None)
    }

    #[must_use]
    pub fn input(&self) -> &str {
        &self.buffer
    }
}
