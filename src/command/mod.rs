use anyhow::Result;

use crate::command::interpreter::CommandInterpreter;
use crate::command::message::Message;
use crate::command::parser::CommandParser;
use crate::command::program_view::ProgramView;

pub mod interpreter;
pub mod message;
pub mod parser;
pub mod program_view;

pub fn execute(input: &str, state: ProgramView<'_>) -> Result<Option<Message>> {
    let mut parser = CommandParser::new(input);
    let result = parser.parse()?;
    let mut interpreter = CommandInterpreter::new(state);
    let message = interpreter.interpret(result)?;
    Ok(message)
}
