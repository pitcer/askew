use crate::canvas::shape::bezier::BezierCurve;
use crate::canvas::shape::interpolation::InterpolationCurve;
use crate::canvas::shape::polyline::PolylineCurve;
use crate::canvas::shape::rational_bezier::RationalBezierCurve;
use crate::canvas::shape::regular_polygon::RegularPolygon;
use crate::canvas::shape::request::sieve::{
    ExcludeAllRequests, ExcludeControlPointsRequests, ExcludeInterpolationRequests,
};
use crate::canvas::shape::trochoid::TrochoidCurve;
use crate::canvas::shape::{Shape, Update};
use crate::request::{
    Request, RequestHandler, RequestHandlerMut, RequestMut, Response, ResponseMut,
};

pub mod declare;
pub mod sieve;

impl<T> RequestHandler<T> for Shape
where
    T: Request,
    PolylineCurve: RequestHandler<T>,
    InterpolationCurve: RequestHandler<T>,
    BezierCurve: RequestHandler<T>,
    RationalBezierCurve: RequestHandler<T>,
    for<'a> ExcludeInterpolationRequests<ExcludeControlPointsRequests<&'a TrochoidCurve>>:
        RequestHandler<T>,
    for<'a> ExcludeAllRequests<&'a RegularPolygon>: RequestHandler<T>,
{
    fn handle(&self, request: T) -> Response<T> {
        match self {
            Shape::Polyline(curve) => curve.handle(request),
            Shape::Interpolation(curve) => curve.handle(request),
            Shape::Bezier(curve) => curve.handle(request),
            Shape::RationalBezier(curve) => curve.handle(request),
            Shape::Trochoid(curve) => {
                ExcludeInterpolationRequests::new(ExcludeControlPointsRequests::new(curve.as_ref()))
                    .handle(request)
            }
            Shape::RegularPolygon(shape) => ExcludeAllRequests::new(shape.as_ref()).handle(request),
        }
    }
}

impl<T> RequestHandlerMut<T> for Shape
where
    T: RequestMut,
    PolylineCurve: RequestHandlerMut<T>,
    InterpolationCurve: RequestHandlerMut<T>,
    BezierCurve: RequestHandlerMut<T>,
    RationalBezierCurve: RequestHandlerMut<T>,
    for<'a> ExcludeInterpolationRequests<ExcludeControlPointsRequests<&'a mut TrochoidCurve>>:
        RequestHandlerMut<T>,
    for<'a> ExcludeAllRequests<&'a mut RegularPolygon>: RequestHandlerMut<T>,
{
    fn handle_mut(&mut self, request: T) -> ResponseMut<T> {
        let result = match self {
            Shape::Polyline(curve) => curve.handle_mut(request),
            Shape::Interpolation(curve) => curve.handle_mut(request),
            Shape::Bezier(curve) => curve.handle_mut(request),
            Shape::RationalBezier(curve) => curve.handle_mut(request),
            Shape::Trochoid(curve) => {
                ExcludeInterpolationRequests::new(ExcludeControlPointsRequests::new(curve.as_mut()))
                    .handle_mut(request)
            }
            Shape::RegularPolygon(shape) => {
                ExcludeAllRequests::new(shape.as_mut()).handle_mut(request)
            }
        };

        if result.is_ok() {
            self.update();
        }

        result
    }
}
