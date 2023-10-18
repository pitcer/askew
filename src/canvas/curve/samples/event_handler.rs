use crate::canvas::curve::samples::Samples;
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::{EventHandler, EventHandlerMut, HandlerResult};

pub struct SamplesEventHandler<'a> {
    samples: &'a Samples,
}

pub struct SamplesEventHandlerMut<'a> {
    samples: &'a mut Samples,
}

impl<'a> SamplesEventHandler<'a> {
    pub fn new(samples: &'a Samples) -> Self {
        Self { samples }
    }
}

impl<'a> SamplesEventHandlerMut<'a> {
    pub fn new(samples: &'a mut Samples) -> Self {
        Self { samples }
    }
}

impl EventHandlerMut<SetSamples> for SamplesEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.samples.samples = event.0 as usize;
        Ok(())
    }
}

impl EventHandler<GetSamples> for SamplesEventHandler<'_> {
    fn handle(&self, _event: GetSamples) -> HandlerResult<GetSamples> {
        Ok(self.samples.samples as u32)
    }
}
