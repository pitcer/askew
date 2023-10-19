use crate::event::PointId;

pub mod macros;
pub mod sieve;
#[cfg(test)]
pub mod tests;

pub trait Request {
    type Response;
}

pub trait RequestMut {
    type Response;
}

pub type Response<T> = anyhow::Result<<T as Request>::Response, Error>;

pub type ResponseMut<T> = anyhow::Result<<T as RequestMut>::Response, Error>;

pub trait RequestHandler<T>
where
    T: Request,
{
    fn handle(&self, request: T) -> Response<T>;
}

pub trait RequestHandlerMut<T>
where
    T: RequestMut,
{
    fn handle_mut(&mut self, request: T) -> ResponseMut<T>;
}

pub trait RequestSubHandler<T> {
    fn sub_handler(&self) -> &T;
}

pub trait RequestSubHandlerMut<T> {
    fn sub_handler_mut(&mut self) -> &mut T;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request `{request}` is unimplemented for handler '{handler}'")]
    Unimplemented { request: String, handler: &'static str },
    #[error("no such point: {0}")]
    NoSuchPoint(PointId),
    #[error("no such curve: {0}")]
    NoSuchCurve(usize),
    #[error("other error: `{0}`")]
    Other(anyhow::Error),
}
