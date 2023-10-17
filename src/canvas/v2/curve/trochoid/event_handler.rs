use crate::canvas::v2::curve::trochoid::TrochoidCurve;
use crate::event::curve::formula::SetTrochoidProperties;
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::{EventHandler, HandlerResult};

pub struct TrochoidCurveEventHandler<'a> {
    curve: &'a mut TrochoidCurve,
}

impl<'a> TrochoidCurveEventHandler<'a> {
    pub fn new(curve: &'a mut TrochoidCurve) -> Self {
        Self { curve }
    }
}

impl EventHandler<SetSamples> for TrochoidCurveEventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<GetSamples> for TrochoidCurveEventHandler<'_> {
    fn handle(&mut self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<SetTrochoidProperties> for TrochoidCurveEventHandler<'_> {
    fn handle(&mut self, event: SetTrochoidProperties) -> HandlerResult<SetTrochoidProperties> {
        self.curve.properties = event.0;
        Ok(())
    }
}
