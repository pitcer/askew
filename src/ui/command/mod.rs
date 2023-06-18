use std::fmt::{Debug, Write};

use anyhow::Result;

use crate::canvas::Canvas;
use crate::event::InputEvent;
use crate::ui::command::interpreter::CommandInterpreter;
use crate::ui::command::parser::CommandParser;

mod interpreter;
mod parser;

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

    pub fn execute(&mut self, properties: &mut Canvas) -> Option<InputEvent> {
        replace_with::replace_with_or_abort_and_return(self, |state| match state {
            CommandState::Open(command) => {
                let (event, command) = command.execute(properties);
                (event, CommandState::Closed(command))
            }
            other => (None, other),
        })
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

#[derive(Debug)]
pub struct Message {
    message: String,
    message_type: MessageType,
}

impl Message {
    pub fn new(message: String, message_type: MessageType) -> Self {
        Self {
            message,
            message_type,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
}

#[derive(Debug)]
pub enum MessageType {
    Info,
    Error,
}

impl CommandClosed {
    pub fn new(message: Option<Message>) -> Self {
        Self { message }
    }

    #[must_use]
    pub fn open(self) -> CommandOpen {
        let mut message = self
            .message
            .map(|message| message.message)
            .unwrap_or_default();
        message.clear();
        CommandOpen::new(message)
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    pub fn message(&self) -> &Option<Message> {
        &self.message
    }
}

#[derive(Debug)]
pub struct CommandOpen {
    buffer: String,
}

impl CommandOpen {
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
    pub fn execute(mut self, properties: &mut Canvas) -> (Option<InputEvent>, CommandClosed) {
        let result: Result<(Option<InputEvent>, Option<Message>)> = (|| {
            let mut parser = CommandParser::new(&self.buffer);
            let result = parser.parse()?;
            let mut interpreter = CommandInterpreter::new(properties);
            let result = interpreter.interpret(result)?;
            let message = None;
            Ok((result, message))
        })();
        let (event, message) = result.unwrap_or_else(|error| {
            self.buffer.clear();
            self.buffer
                .write_fmt(format_args!("{error}"))
                .expect("formatting should not fail");
            let message = Message::new(self.buffer, MessageType::Error);
            (None, Some(message))
        });
        let closed = CommandClosed::new(message);
        (event, closed)
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
