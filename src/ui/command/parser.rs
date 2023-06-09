#[derive(Debug)]
pub struct CommandParser<'a> {
    input: &'a str,
}

impl<'a> CommandParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(&mut self) -> Result<Command, Error> {
        Ok(Command)
    }
}

#[derive(Debug)]
pub struct Command;

#[derive(Debug, thiserror::Error)]
pub enum Error {}
