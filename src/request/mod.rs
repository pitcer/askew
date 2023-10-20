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

pub type Response<R> = anyhow::Result<<R as Request>::Response, Error>;

pub type ResponseMut<R> = anyhow::Result<<R as RequestMut>::Response, Error>;

pub trait RequestHandler<R>
where
    R: Request,
{
    fn handle(&self, request: R) -> Response<R>;
}

impl<R, H> RequestHandler<R> for &H
where
    H: RequestHandler<R>,
    R: Request,
{
    fn handle(&self, request: R) -> Response<R> {
        (*self).handle(request)
    }
}

pub trait RequestHandlerMut<R>
where
    R: RequestMut,
{
    fn handle_mut(&mut self, request: R) -> ResponseMut<R>;
}

impl<R, H> RequestHandlerMut<R> for &mut H
where
    H: RequestHandlerMut<R>,
    R: RequestMut,
{
    fn handle_mut(&mut self, request: R) -> ResponseMut<R> {
        (*self).handle_mut(request)
    }
}

pub trait RequestSubHandler<H> {
    fn sub_handler(&self) -> &H;

    fn sub_handle<R>(&self, request: R) -> Response<R>
    where
        H: RequestHandler<R>,
        R: Request,
    {
        self.sub_handler().handle(request)
    }
}

pub trait RequestSubHandlerMut<H> {
    fn sub_handler_mut(&mut self) -> &mut H;

    fn sub_handle_mut<R>(&mut self, request: R) -> ResponseMut<R>
    where
        H: RequestHandlerMut<R>,
        R: RequestMut,
    {
        self.sub_handler_mut().handle_mut(request)
    }
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

pub type PointId = usize;

#[derive(Debug)]
pub enum Change {
    Decrease,
    Increase,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
