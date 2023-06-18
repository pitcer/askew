use crate::canvas::curve::control_points::kind::rational_bezier::{
    RationalBezier, RationalBezierPoint,
};
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetWeight, MovePoint,
};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{DelegateEventHandler, Error, Event, EventHandler, HandlerResult};

pub struct RationalBezierEventHandler<'a> {
    curve: &'a mut RationalBezier,
}

impl<'a> RationalBezierEventHandler<'a> {
    pub fn new(curve: &'a mut RationalBezier) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for RationalBezierEventHandler<'a>
where
    E: Event,
    for<'b> ControlPointsEventHandler<'b, RationalBezierPoint>: EventHandler<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b, RationalBezierPoint> where Self: 'b;

    fn delegate_handler(&mut self) -> Self::Delegate<'_> {
        self.curve.points.event_handler()
    }
}

impl EventHandler<ChangeWeight> for RationalBezierEventHandler<'_> {
    fn handle(&mut self, event: ChangeWeight) -> HandlerResult<ChangeWeight> {
        if let Some(point) = self.curve.points.get_mut(event.id) {
            point.weight = event.weight;
            Ok(())
        } else {
            Err(Error::NoSuchPoint(event.id))
        }
    }
}

impl EventHandler<AddWeightedControlPoint> for RationalBezierEventHandler<'_> {
    fn handle(&mut self, event: AddWeightedControlPoint) -> HandlerResult<AddWeightedControlPoint> {
        self.curve.points.add(event.point);
        Ok(())
    }
}

impl EventHandler<GetWeight> for RationalBezierEventHandler<'_> {
    fn handle(&mut self, event: GetWeight) -> HandlerResult<GetWeight> {
        if let Some(point) = self.curve.points.get(event.id) {
            Ok(point.weight)
        } else {
            Err(Error::NoSuchPoint(event.id))
        }
    }
}

delegate_handlers! {
    RationalBezierEventHandler<'_> {
        GetControlPointsLength,
        MovePoint,
        DeletePoint,
    }
}

unimplemented_handlers! {
    RationalBezierEventHandler<'_> {
        AddControlPoint,
    }
}
