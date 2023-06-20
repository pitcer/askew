use crate::canvas::curve::control_points::kind::rational_bezier::{
    RationalBezier, RationalBezierPoint,
};
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::control_points::weighted::{
    AddWeightedControlPoint, ChangeWeight, GetWeight,
};
use crate::event::curve::{control_points, GetSamples, SetSamples};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{
    canvas, curve, DelegateEventHandler, Error, Event, EventHandler, HandlerResult,
};

pub struct RationalBezierEventHandler<'a> {
    curve: &'a mut RationalBezier,
}

impl<'a> RationalBezierEventHandler<'a> {
    pub fn new(curve: &'a mut RationalBezier) -> Self {
        Self { curve }
    }
}

impl<E> DelegateEventHandler<E> for RationalBezierEventHandler<'_>
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

// NOTE: Those could be delegated, but unfortunately for some reason trait solver does not allow
// to implement the following, because of a conflict with ControlPointsEventHandler delegate:
// impl<E> DelegateEventHandler<E> for RationalBezierEventHandler<'_>
// where
//     E: Event,
//     for<'b> SamplesEventHandler<'b>: EventHandler<E>
impl EventHandler<SetSamples> for RationalBezierEventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<GetSamples> for RationalBezierEventHandler<'_> {
    fn handle(&mut self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

delegate_handlers! {
    RationalBezierEventHandler<'_> {
        control_points::GetControlPointsLength,
        control_points::MovePoint,
        control_points::DeletePoint,

        canvas::RotateCurve,
        canvas::MoveCurve,
        canvas::GetCurveCenter,

        curve::GetPoint,
    }
}

unimplemented_handlers! {
    RationalBezierEventHandler<'_> {
        control_points::AddControlPoint,
    }
}
