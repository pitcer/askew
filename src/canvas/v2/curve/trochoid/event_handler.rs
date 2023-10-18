use crate::canvas::v2::curve::trochoid::TrochoidCurve;
use crate::event::curve::formula::SetTrochoidProperties;
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::{EventHandler, EventHandlerMut, HandlerResult};

pub struct TrochoidCurveEventHandler<'a> {
    curve: &'a TrochoidCurve,
}

pub struct TrochoidCurveEventHandlerMut<'a> {
    curve: &'a mut TrochoidCurve,
}

impl<'a> TrochoidCurveEventHandler<'a> {
    #[must_use]
    pub fn new(curve: &'a TrochoidCurve) -> Self {
        Self { curve }
    }
}

impl<'a> TrochoidCurveEventHandlerMut<'a> {
    pub fn new(curve: &'a mut TrochoidCurve) -> Self {
        Self { curve }
    }
}

impl EventHandlerMut<SetSamples> for TrochoidCurveEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler_mut().handle_mut(event)
    }
}

impl EventHandler<GetSamples> for TrochoidCurveEventHandler<'_> {
    fn handle(&self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandlerMut<SetTrochoidProperties> for TrochoidCurveEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: SetTrochoidProperties) -> HandlerResult<SetTrochoidProperties> {
        self.curve.properties = event.0;
        Ok(())
    }
}
