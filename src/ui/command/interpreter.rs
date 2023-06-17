use crate::canvas::Canvas;
use crate::event::Event;
use crate::ui::command::parser::{Command, Get, Set, Toggle};

#[derive(Debug)]
pub struct CommandInterpreter<'a> {
    properties: &'a mut Canvas,
}

impl<'a> CommandInterpreter<'a> {
    pub fn new(properties: &'a mut Canvas) -> Self {
        Self { properties }
    }

    pub fn interpret(&mut self, command: Command) -> Result<Option<Event>, Error> {
        match command {
            Command::Get(get) => self.interpret_get(get),
            Command::Set(set) => self.interpret_set(set),
            Command::Toggle(toggle) => self.interpret_toggle(toggle),
        }
        Err(Error::UnknownCommand)
    }

    fn interpret_get(&mut self, get: Get) {
        match get {
            Get::ConvexHull => {}
            Get::ChebyshevNodes => {}
            Get::Samples => {}
        }
    }

    fn interpret_set(&mut self, set: Set) {
        match set {
            Set::ConvexHull(value) => self.properties.properties_mut().show_convex_hull = value,
            Set::ChebyshevNodes(value) => {}
            Set::Samples(value) => {
                if let Some(samples) = self.properties.current_curve_mut().samples_mut() {
                    *samples = value;
                }
            }
        }
    }

    fn interpret_toggle(&mut self, toggle: Toggle) {
        match toggle {
            Toggle::ConvexHull => {
                self.properties.properties_mut().show_convex_hull =
                    !self.properties.properties_mut().show_convex_hull;
            }
            Toggle::ChebyshevNodes => {}
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown command")]
    UnknownCommand,
}
