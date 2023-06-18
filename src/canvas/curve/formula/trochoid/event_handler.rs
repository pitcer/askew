use crate::canvas::curve::formula::trochoid::Trochoid;
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::{EventHandler, HandlerResult};

pub struct TrochoidEventHandler<'a> {
    curve: &'a mut Trochoid,
}

impl<'a> TrochoidEventHandler<'a> {
    pub fn new(curve: &'a mut Trochoid) -> Self {
        Self { curve }
    }
}

impl EventHandler<SetSamples> for TrochoidEventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<GetSamples> for TrochoidEventHandler<'_> {
    fn handle(&mut self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}
