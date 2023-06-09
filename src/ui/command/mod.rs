use std::fmt::{Debug, Write};
use std::marker::PhantomData;

use anyhow::Result;

use crate::event::Event;
use crate::ui::command::interpreter::CommandInterpreter;
use crate::ui::command::parser::CommandParser;

mod interpreter;
mod parser;

#[derive(Debug)]
pub enum CommandState {
    Closed(Command<Closed>),
    Open(Command<Open>),
}

impl CommandState {
    #[must_use]
    pub fn initial() -> Self {
        CommandState::Closed(Command::new(String::new()))
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

    pub fn execute(&mut self) -> Option<Event> {
        replace_with::replace_with_or_abort_and_return(self, |state| match state {
            CommandState::Open(command) => {
                let (event, command) = command.execute();
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
pub struct Closed;

#[derive(Debug)]
pub struct Open;

#[derive(Debug)]
pub struct Command<State> {
    buffer: String,
    state: PhantomData<State>,
}

impl<State> Command<State> {
    #[must_use]
    pub fn new(buffer: String) -> Self {
        Self {
            buffer,
            state: PhantomData,
        }
    }
}

impl Command<Closed> {
    #[must_use]
    pub fn open(mut self) -> Command<Open> {
        self.buffer.clear();
        Command::new(self.buffer)
    }

    pub fn clear_message(&mut self) {
        self.buffer.clear();
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.buffer
    }
}

impl Command<Open> {
    pub fn receive_character(&mut self, character: char) {
        self.buffer.push(character);
    }

    #[must_use]
    pub fn execute(mut self) -> (Option<Event>, Command<Closed>) {
        let result: Result<Option<Event>> = (|| {
            let mut parser = CommandParser::new(&self.buffer);
            let result = parser.parse()?;
            let mut interpreter = CommandInterpreter::new(result);
            let result = interpreter.interpret()?;
            Ok(result)
        })();
        self.buffer.clear();
        let event = result.unwrap_or_else(|error| {
            self.buffer
                .write_fmt(format_args!("{error}"))
                .expect("formatting should not fail");
            None
        });
        let closed = Command::new(self.buffer);
        (event, closed)
    }

    #[must_use]
    pub fn close(mut self) -> Command<Closed> {
        self.buffer.clear();
        Command::new(self.buffer)
    }

    #[must_use]
    pub fn input(&self) -> &str {
        &self.buffer
    }
}
