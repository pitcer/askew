use std::fmt::Debug;

use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::math::point::Point;
use crate::event::canvas::{GetCurveCenter, MoveCurve, RotateCurve, SelectPoint};
use crate::event::curve::control_points::{
    AddControlPoint, DeletePoint, GetControlPointsLength, MovePoint,
};
use crate::event::curve::GetPoint;
use crate::event::{Error, EventHandler, HandlerResult};

pub struct ControlPointsEventHandler<'a, P = CurvePoint> {
    points: &'a mut ControlPoints<P>,
}

impl<'a, P> ControlPointsEventHandler<'a, P> {
    pub fn new(points: &'a mut ControlPoints<P>) -> Self {
        Self { points }
    }
}

impl<P> EventHandler<GetControlPointsLength> for ControlPointsEventHandler<'_, P> {
    fn handle(&mut self, _event: GetControlPointsLength) -> HandlerResult<GetControlPointsLength> {
        Ok(self.points.length())
    }
}

impl EventHandler<AddControlPoint> for ControlPointsEventHandler<'_> {
    fn handle(&mut self, event: AddControlPoint) -> HandlerResult<AddControlPoint> {
        self.points.add(event.point);
        Ok(())
    }
}

impl<P> EventHandler<MovePoint> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>>,
{
    fn handle(&mut self, event: MovePoint) -> HandlerResult<MovePoint> {
        self.points
            .shift(event.id, event.shift)
            .ok_or_else(|| Error::NoSuchPoint(event.id))?;
        Ok(())
    }
}

impl<P> EventHandler<DeletePoint> for ControlPointsEventHandler<'_, P> {
    fn handle(&mut self, event: DeletePoint) -> HandlerResult<DeletePoint> {
        self.points
            .remove(event.id)
            .ok_or_else(|| Error::NoSuchPoint(event.id))?;
        Ok(())
    }
}

impl<P> EventHandler<MoveCurve> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>>,
{
    fn handle(&mut self, event: MoveCurve) -> HandlerResult<MoveCurve> {
        self.points.shift_all(event.shift);
        Ok(())
    }
}

impl<P> EventHandler<RotateCurve> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug,
{
    fn handle(&mut self, event: RotateCurve) -> HandlerResult<RotateCurve> {
        self.points.rotate_all(event.angle);
        Ok(())
    }
}

impl<P> EventHandler<GetCurveCenter> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug,
{
    fn handle(&mut self, _event: GetCurveCenter) -> HandlerResult<GetCurveCenter> {
        Ok(self.points.center_of_mass())
    }
}

impl<P> EventHandler<GetPoint> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug,
{
    fn handle(&mut self, event: GetPoint) -> HandlerResult<GetPoint> {
        let point = *self
            .points
            .get(event.0)
            .ok_or_else(|| Error::NoSuchPoint(event.0))?
            .as_ref();
        Ok(point)
    }
}

impl<P> EventHandler<SelectPoint> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug + Copy,
{
    fn handle(&mut self, event: SelectPoint) -> HandlerResult<SelectPoint> {
        Ok(self.points.select_point(event.guess, event.radius))
    }
}
