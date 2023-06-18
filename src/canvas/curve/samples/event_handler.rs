use crate::canvas::curve::samples::Samples;
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::{EventHandler, HandlerResult};

pub struct SamplesEventHandler<'a> {
    samples: &'a mut Samples,
}

impl<'a> SamplesEventHandler<'a> {
    pub fn new(samples: &'a mut Samples) -> Self {
        Self { samples }
    }
}

impl EventHandler<SetSamples> for SamplesEventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.samples.samples = event.0 as usize;
        Ok(())
    }
}

impl EventHandler<GetSamples> for SamplesEventHandler<'_> {
    fn handle(&mut self, _event: GetSamples) -> HandlerResult<GetSamples> {
        Ok(self.samples.samples as u32)
    }
}
