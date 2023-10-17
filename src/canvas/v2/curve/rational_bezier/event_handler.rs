use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::canvas::v2::curve::rational_bezier::{RationalBezierCurve, RationalBezierPoint};
use crate::event::curve::control_points::weighted::{
    AddWeightedControlPoint, ChangeWeight, GetWeight,
};
use crate::event::curve::{control_points, GetSamples, SetSamples};
use crate::event::macros::{
    delegate_handlers, delegate_handlers_mut, unimplemented_handlers, unimplemented_handlers_mut,
};
use crate::event::{
    canvas, curve, DelegateEventHandler, DelegateEventHandlerMut, Error, Event, EventHandler,
    EventHandlerMut, EventMut, HandlerResult,
};

pub struct RationalBezierCurveEventHandler<'a> {
    curve: &'a mut RationalBezierCurve,
}

impl<'a> RationalBezierCurveEventHandler<'a> {
    pub fn new(curve: &'a mut RationalBezierCurve) -> Self {
        Self { curve }
    }
}

impl<E> DelegateEventHandler<E> for RationalBezierCurveEventHandler<'_>
where
    E: Event,
    for<'b> ControlPointsEventHandler<'b, RationalBezierPoint>: EventHandler<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b, RationalBezierPoint> where Self: 'b;

    fn delegate_handler(&self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

impl<E> DelegateEventHandlerMut<E> for RationalBezierCurveEventHandler<'_>
where
    E: EventMut,
    for<'b> ControlPointsEventHandler<'b, RationalBezierPoint>: EventHandlerMut<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b, RationalBezierPoint> where Self: 'b;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

impl EventHandlerMut<ChangeWeight> for RationalBezierCurveEventHandler<'_> {
    fn handle_mut(&mut self, event: ChangeWeight) -> HandlerResult<ChangeWeight> {
        if let Some(point) = self.curve.control_points.points.get_mut(event.id) {
            *point.weight_mut() = event.weight;
            Ok(())
        } else {
            Err(Error::NoSuchPoint(event.id))
        }
    }
}

impl EventHandlerMut<AddWeightedControlPoint> for RationalBezierCurveEventHandler<'_> {
    fn handle_mut(
        &mut self,
        event: AddWeightedControlPoint,
    ) -> HandlerResult<AddWeightedControlPoint> {
        self.curve.control_points.points.add(event.point);
        Ok(())
    }
}

impl EventHandler<GetWeight> for RationalBezierCurveEventHandler<'_> {
    fn handle(&self, event: GetWeight) -> HandlerResult<GetWeight> {
        if let Some(point) = self.curve.control_points.points.get(event.id) {
            Ok(point.weight())
        } else {
            Err(Error::NoSuchPoint(event.id))
        }
    }
}

// NOTE: Those could be delegated, but unfortunately for some reason trait solver does not allow
// to implement the following, because of a conflict with ControlPointsEventHandler delegate:
// impl<E> DelegateEventHandler<E> for RationalBezierCurveEventHandler<'_>
// where
//     E: Event,
//     for<'b> SamplesEventHandler<'b>: EventHandler<E>
impl EventHandlerMut<SetSamples> for RationalBezierCurveEventHandler<'_> {
    fn handle_mut(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle_mut(event)
    }
}

impl EventHandler<GetSamples> for RationalBezierCurveEventHandler<'_> {
    fn handle(&self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

delegate_handlers! {
    RationalBezierCurveEventHandler<'_> {
        control_points::GetControlPointsLength,

        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint,
    }
}

delegate_handlers_mut! {
    RationalBezierCurveEventHandler<'_> {
        control_points::MovePoint,
        control_points::DeletePoint,

        canvas::RotateCurve,
        canvas::MoveCurve,
    }
}

unimplemented_handlers_mut! {
    RationalBezierCurveEventHandler<'_> {
        control_points::AddControlPoint,
    }
}
