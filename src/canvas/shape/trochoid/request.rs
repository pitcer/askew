use crate::canvas::samples::Samples;
use crate::canvas::shape::request::declare::{GetSamples, SetSamples, SetTrochoidProperties};
use crate::canvas::shape::trochoid::TrochoidCurve;
use crate::request::macros::delegate_requests;
use crate::request::{RequestHandlerMut, RequestSubHandler, RequestSubHandlerMut, ResponseMut};

delegate_requests! {
    TrochoidCurve {
        { GetSamples => Samples },
        { mut SetSamples => Samples },
    }
}

impl RequestHandlerMut<SetTrochoidProperties> for TrochoidCurve {
    fn handle_mut(&mut self, event: SetTrochoidProperties) -> ResponseMut<SetTrochoidProperties> {
        self.properties = event.0;
        Ok(())
    }
}

impl RequestSubHandler<Samples> for TrochoidCurve {
    fn sub_handler(&self) -> &Samples {
        &self.samples
    }
}

impl RequestSubHandlerMut<Samples> for TrochoidCurve {
    fn sub_handler_mut(&mut self) -> &mut Samples {
        &mut self.samples
    }
}
