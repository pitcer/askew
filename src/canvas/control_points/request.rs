use std::fmt::Debug;

use crate::canvas::control_points::point::CurvePoint;
use crate::canvas::control_points::ControlPoints;
use crate::canvas::curve::rational_bezier::RationalBezierPoint;
use crate::canvas::curve::request::declare::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetPoint, GetWeight, MoveCurve, MovePoint, RotateCurve, SelectPoint,
};
use crate::canvas::math::point::Point;
use crate::request::{Error, RequestHandler, RequestHandlerMut, Response, ResponseMut};

impl RequestHandlerMut<AddControlPoint> for ControlPoints<CurvePoint> {
    fn handle_mut(&mut self, event: AddControlPoint) -> ResponseMut<AddControlPoint> {
        self.add(event.point);
        Ok(())
    }
}

impl RequestHandlerMut<ChangeWeight> for ControlPoints<RationalBezierPoint> {
    fn handle_mut(&mut self, event: ChangeWeight) -> ResponseMut<ChangeWeight> {
        if let Some(point) = self.get_mut(event.id) {
            *point.weight_mut() = event.weight;
            Ok(())
        } else {
            Err(Error::NoSuchPoint(event.id))
        }
    }
}

impl RequestHandlerMut<AddWeightedControlPoint> for ControlPoints<RationalBezierPoint> {
    fn handle_mut(
        &mut self,
        event: AddWeightedControlPoint,
    ) -> ResponseMut<AddWeightedControlPoint> {
        self.add(event.point);
        Ok(())
    }
}

impl RequestHandler<GetWeight> for ControlPoints<RationalBezierPoint> {
    fn handle(&self, event: GetWeight) -> Response<GetWeight> {
        if let Some(point) = self.get(event.id) {
            Ok(point.weight())
        } else {
            Err(Error::NoSuchPoint(event.id))
        }
    }
}

impl<P> RequestHandler<GetControlPointsLength> for ControlPoints<P> {
    fn handle(&self, _event: GetControlPointsLength) -> Response<GetControlPointsLength> {
        Ok(self.length())
    }
}

impl<P> RequestHandlerMut<MovePoint> for ControlPoints<P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>>,
{
    fn handle_mut(&mut self, event: MovePoint) -> ResponseMut<MovePoint> {
        self.shift(event.id, event.shift).ok_or_else(|| Error::NoSuchPoint(event.id))?;
        Ok(())
    }
}

impl<P> RequestHandlerMut<DeletePoint> for ControlPoints<P> {
    fn handle_mut(&mut self, event: DeletePoint) -> ResponseMut<DeletePoint> {
        self.remove(event.id).ok_or_else(|| Error::NoSuchPoint(event.id))?;
        Ok(())
    }
}

impl<P> RequestHandlerMut<MoveCurve> for ControlPoints<P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>>,
{
    fn handle_mut(&mut self, event: MoveCurve) -> ResponseMut<MoveCurve> {
        self.shift_all(event.shift);
        Ok(())
    }
}

impl<P> RequestHandlerMut<RotateCurve> for ControlPoints<P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug + Into<Point<f32>> + Copy,
{
    fn handle_mut(&mut self, event: RotateCurve) -> ResponseMut<RotateCurve> {
        self.rotate_all(event.angle);
        Ok(())
    }
}

impl<P> RequestHandler<GetCurveCenter> for ControlPoints<P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug + Copy + Into<Point<f32>>,
{
    fn handle(&self, _event: GetCurveCenter) -> Response<GetCurveCenter> {
        Ok(self.center_of_mass())
    }
}

impl<P> RequestHandler<GetPoint> for ControlPoints<P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug,
{
    fn handle(&self, event: GetPoint) -> Response<GetPoint> {
        let point = *self.get(event.0).ok_or_else(|| Error::NoSuchPoint(event.0))?.as_ref();
        Ok(point)
    }
}

impl<P> RequestHandler<SelectPoint> for ControlPoints<P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug + Copy,
{
    fn handle(&self, event: SelectPoint) -> Response<SelectPoint> {
        Ok(self.select_point(event.guess, event.radius))
    }
}
