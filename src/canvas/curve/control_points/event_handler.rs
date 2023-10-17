use crate::canvas::curve::Curve;
use crate::canvas::v2::curve::bezier::event_handler::BezierCurveEventHandler;
use crate::canvas::v2::curve::interpolation::event_handler::InterpolationCurveEventHandler;
use crate::canvas::v2::curve::polyline::event_handler::PolylineCurveEventHandler;
use crate::canvas::v2::curve::rational_bezier::event_handler::RationalBezierCurveEventHandler;
use crate::event::macros::delegate_events_mut;
use crate::event::{DelegateEventMut, EventHandlerMut, EventMut};
use crate::{
    event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes},
    event::macros::delegate_events,
    event::{canvas, curve, Error},
    event::{DelegateEvent, Event, EventHandler, HandlerResult},
};

pub struct ControlPointsCurveEventHandler<'a> {
    curve: &'a mut Curve,
}

impl<'a> ControlPointsCurveEventHandler<'a> {
    pub fn new(curve: &'a mut Curve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEvent<E> for ControlPointsCurveEventHandler<'a>
where
    E: Event,
    for<'b> PolylineCurveEventHandler<'b>: EventHandler<E>,
    for<'b> InterpolationCurveEventHandler<'b>: EventHandler<E>,
    for<'b> BezierCurveEventHandler<'b>: EventHandler<E>,
    for<'b> RationalBezierCurveEventHandler<'b>: EventHandler<E>,
{
    fn delegate(&self, event: E) -> HandlerResult<E> {
        match self.curve {
            Curve::Polyline(curve) => curve.event_handler().handle(event),
            Curve::Interpolation(curve) => curve.event_handler().handle(event),
            Curve::RationalBezier(curve) => curve.event_handler().handle(event),
            Curve::Bezier(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl<'a, E> DelegateEventMut<E> for ControlPointsCurveEventHandler<'a>
where
    E: EventMut,
    for<'b> PolylineCurveEventHandler<'b>: EventHandlerMut<E>,
    for<'b> InterpolationCurveEventHandler<'b>: EventHandlerMut<E>,
    for<'b> BezierCurveEventHandler<'b>: EventHandlerMut<E>,
    for<'b> RationalBezierCurveEventHandler<'b>: EventHandlerMut<E>,
{
    fn delegate_mut(&mut self, event: E) -> HandlerResult<E> {
        match self.curve {
            Curve::Polyline(curve) => curve.event_handler().handle_mut(event),
            Curve::Interpolation(curve) => curve.event_handler().handle_mut(event),
            Curve::RationalBezier(curve) => curve.event_handler().handle_mut(event),
            Curve::Bezier(curve) => curve.event_handler().handle_mut(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandlerMut<SetInterpolationNodes> for ControlPointsCurveEventHandler<'_> {
    fn handle_mut(&mut self, event: SetInterpolationNodes) -> HandlerResult<SetInterpolationNodes> {
        match self.curve {
            Curve::Interpolation(curve) => curve.event_handler().handle_mut(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetInterpolationNodes> for ControlPointsCurveEventHandler<'_> {
    fn handle(&self, event: GetInterpolationNodes) -> HandlerResult<GetInterpolationNodes> {
        match self.curve {
            Curve::Interpolation(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

delegate_events! {
    ControlPointsCurveEventHandler<'_> {
        curve::control_points::GetControlPointsLength,

        curve::control_points::weighted::GetWeight,

        curve::GetSamples,

        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint,
    }
}

delegate_events_mut! {
    ControlPointsCurveEventHandler<'_> {
        curve::control_points::AddControlPoint,
        curve::control_points::MovePoint,
        curve::control_points::DeletePoint,

        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,

        curve::SetSamples,

        canvas::RotateCurve,
        canvas::MoveCurve,
    }
}
