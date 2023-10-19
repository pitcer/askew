use std::fmt::Debug;

use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::CurvePoint;
use crate::canvas::math::point::Point;
use crate::event::canvas::{GetCurveCenter, MoveCurve, RotateCurve, SelectPoint};
use crate::event::curve::control_points::{
    AddControlPoint, DeletePoint, GetControlPointsLength, MovePoint,
};
use crate::event::curve::GetPoint;
use crate::event::{Error, EventHandler, EventHandlerMut, HandlerResult};

#[deprecated]
pub struct ControlPointsEventHandler<'a, P = CurvePoint> {
    points: &'a ControlPoints<P>,
}

#[deprecated]
pub struct ControlPointsEventHandlerMut<'a, P = CurvePoint> {
    points: &'a mut ControlPoints<P>,
}

impl<'a, P> ControlPointsEventHandler<'a, P> {
    #[must_use]
    pub fn new(points: &'a ControlPoints<P>) -> Self {
        Self { points }
    }
}

impl<'a, P> ControlPointsEventHandlerMut<'a, P> {
    pub fn new(points: &'a mut ControlPoints<P>) -> Self {
        Self { points }
    }
}

impl<P> EventHandler<GetControlPointsLength> for ControlPointsEventHandler<'_, P> {
    fn handle(&self, _event: GetControlPointsLength) -> HandlerResult<GetControlPointsLength> {
        Ok(self.points.length())
    }
}

impl EventHandlerMut<AddControlPoint> for ControlPointsEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: AddControlPoint) -> HandlerResult<AddControlPoint> {
        self.points.add(event.point);
        Ok(())
    }
}

impl<P> EventHandlerMut<MovePoint> for ControlPointsEventHandlerMut<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>>,
{
    fn handle_mut(&mut self, event: MovePoint) -> HandlerResult<MovePoint> {
        self.points.shift(event.id, event.shift).ok_or_else(|| Error::NoSuchPoint(event.id))?;
        Ok(())
    }
}

impl<P> EventHandlerMut<DeletePoint> for ControlPointsEventHandlerMut<'_, P> {
    fn handle_mut(&mut self, event: DeletePoint) -> HandlerResult<DeletePoint> {
        self.points.remove(event.id).ok_or_else(|| Error::NoSuchPoint(event.id))?;
        Ok(())
    }
}

impl<P> EventHandlerMut<MoveCurve> for ControlPointsEventHandlerMut<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>>,
{
    fn handle_mut(&mut self, event: MoveCurve) -> HandlerResult<MoveCurve> {
        self.points.shift_all(event.shift);
        Ok(())
    }
}

impl<P> EventHandlerMut<RotateCurve> for ControlPointsEventHandlerMut<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug + Into<Point<f32>> + Copy,
{
    fn handle_mut(&mut self, event: RotateCurve) -> HandlerResult<RotateCurve> {
        self.points.rotate_all(event.angle);
        Ok(())
    }
}

impl<P> EventHandler<GetCurveCenter> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug + Copy + Into<Point<f32>>,
{
    fn handle(&self, _event: GetCurveCenter) -> HandlerResult<GetCurveCenter> {
        Ok(self.points.center_of_mass())
    }
}

impl<P> EventHandler<GetPoint> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug,
{
    fn handle(&self, event: GetPoint) -> HandlerResult<GetPoint> {
        let point = *self.points.get(event.0).ok_or_else(|| Error::NoSuchPoint(event.0))?.as_ref();
        Ok(point)
    }
}

impl<P> EventHandler<SelectPoint> for ControlPointsEventHandler<'_, P>
where
    P: AsRef<Point<f32>> + AsMut<Point<f32>> + Debug + Copy,
{
    fn handle(&self, event: SelectPoint) -> HandlerResult<SelectPoint> {
        Ok(self.points.select_point(event.guess, event.radius))
    }
}
