use crate::canvas::curve::samples::Samples;
use crate::canvas::v2::request::{GetSamples, SetSamples};
use crate::request::{RequestHandler, RequestHandlerMut, Response, ResponseMut};

impl RequestHandlerMut<SetSamples> for Samples {
    fn handle_mut(&mut self, request: SetSamples) -> ResponseMut<SetSamples> {
        self.samples = request.0 as usize;
        Ok(())
    }
}

impl RequestHandler<GetSamples> for Samples {
    fn handle(&self, _request: GetSamples) -> Response<GetSamples> {
        Ok(self.samples as u32)
    }
}
