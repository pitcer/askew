use std::marker::PhantomData;

use crate::request::{RequestSubHandler, RequestSubHandlerMut};

pub struct RequestSieve<N, T> {
    handler: T,
    name: PhantomData<N>,
}

impl<N, T> RequestSieve<N, T> {
    pub fn new(handler: T) -> Self {
        Self { handler, name: PhantomData }
    }
}

impl<N, T> RequestSubHandler<T> for RequestSieve<N, T> {
    fn sub_handler(&self) -> &T {
        &self.handler
    }
}

impl<N, T> RequestSubHandlerMut<T> for RequestSieve<N, T> {
    fn sub_handler_mut(&mut self) -> &mut T {
        &mut self.handler
    }
}
