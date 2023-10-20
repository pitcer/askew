use crate::canvas::curve::Curve;
use crate::canvas::v2::curve::bezier::BezierCurve;
use crate::canvas::v2::curve::interpolation::InterpolationCurve;
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::canvas::v2::curve::rational_bezier::RationalBezierCurve;
use crate::canvas::v2::curve::trochoid::TrochoidCurve;
use crate::canvas::v2::request::{ExcludeControlPointsRequests, ExcludeInterpolationRequests};
use crate::request::macros::delegate_requests;
use crate::request::{
    Request, RequestHandler, RequestHandlerMut, RequestMut, Response, ResponseMut,
};

impl<T> RequestHandler<T> for Curve
where
    T: Request,
    PolylineCurve: RequestHandler<T>,
    InterpolationCurve: RequestHandler<T>,
    BezierCurve: RequestHandler<T>,
    RationalBezierCurve: RequestHandler<T>,
    for<'a> ExcludeInterpolationRequests<ExcludeControlPointsRequests<&'a TrochoidCurve>>:
        RequestHandler<T>,
{
    fn handle(&self, request: T) -> Response<T> {
        match self {
            Curve::Polyline(curve) => curve.handle(request),
            Curve::Interpolation(curve) => curve.handle(request),
            Curve::Bezier(curve) => curve.handle(request),
            Curve::RationalBezier(curve) => curve.handle(request),
            Curve::Trochoid(curve) => {
                ExcludeInterpolationRequests::new(ExcludeControlPointsRequests::new(curve.as_ref()))
                    .handle(request)
            }
        }
    }
}

impl<T> RequestHandlerMut<T> for Curve
where
    T: RequestMut,
    PolylineCurve: RequestHandlerMut<T>,
    InterpolationCurve: RequestHandlerMut<T>,
    BezierCurve: RequestHandlerMut<T>,
    RationalBezierCurve: RequestHandlerMut<T>,
    for<'a> ExcludeInterpolationRequests<ExcludeControlPointsRequests<&'a mut TrochoidCurve>>:
        RequestHandlerMut<T>,
{
    fn handle_mut(&mut self, request: T) -> ResponseMut<T> {
        match self {
            Curve::Polyline(curve) => curve.handle_mut(request),
            Curve::Interpolation(curve) => curve.handle_mut(request),
            Curve::Bezier(curve) => curve.handle_mut(request),
            Curve::RationalBezier(curve) => curve.handle_mut(request),
            Curve::Trochoid(curve) => {
                ExcludeInterpolationRequests::new(ExcludeControlPointsRequests::new(curve.as_mut()))
                    .handle_mut(request)
            }
        }
    }
}
