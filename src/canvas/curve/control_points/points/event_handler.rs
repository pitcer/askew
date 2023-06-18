use crate::canvas::curve::control_points::kind::rational_bezier::RationalBezierPoint;
use crate::canvas::curve::control_points::points::ControlPoints;
use crate::canvas::curve::control_points::CurvePoint;
use crate::event::curve::control_points::{
    AddControlPoint, DeletePoint, GetControlPointsLength, MovePoint,
};
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

impl EventHandler<MovePoint> for ControlPointsEventHandler<'_> {
    fn handle(&mut self, event: MovePoint) -> HandlerResult<MovePoint> {
        self.points
            .shift(event.id, event.shift)
            .ok_or_else(|| Error::NoSuchPoint(event.id))?;
        Ok(())
    }
}

impl EventHandler<MovePoint> for ControlPointsEventHandler<'_, RationalBezierPoint> {
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
