use crate::canvas::curve::Curve;
use crate::canvas::v2::curve::trochoid::event_handler::{
    TrochoidCurveEventHandler, TrochoidCurveEventHandlerMut,
};
use crate::event::macros::{
    delegate_events, delegate_events_mut, unimplemented_handlers, unimplemented_handlers_mut,
};
use crate::event::{
    curve, DelegateEvent, DelegateEventMut, Error, Event, EventHandler, EventHandlerMut, EventMut,
    HandlerResult,
};

pub struct FormulaCurveEventHandler<'a> {
    curve: &'a Curve,
}

pub struct FormulaCurveEventHandlerMut<'a> {
    curve: &'a mut Curve,
}

impl<'a> FormulaCurveEventHandler<'a> {
    #[must_use]
    pub fn new(curve: &'a Curve) -> Self {
        Self { curve }
    }
}

impl<'a> FormulaCurveEventHandlerMut<'a> {
    pub fn new(curve: &'a mut Curve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEvent<E> for FormulaCurveEventHandler<'a>
where
    E: Event,
    for<'b> TrochoidCurveEventHandler<'b>: EventHandler<E>,
{
    fn delegate(&self, event: E) -> HandlerResult<E> {
        match self.curve {
            Curve::Trochoid(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl<'a, E> DelegateEventMut<E> for FormulaCurveEventHandlerMut<'a>
where
    E: EventMut,
    for<'b> TrochoidCurveEventHandlerMut<'b>: EventHandlerMut<E>,
{
    fn delegate_mut(&mut self, event: E) -> HandlerResult<E> {
        match self.curve {
            Curve::Trochoid(curve) => curve.event_handler_mut().handle_mut(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

delegate_events_mut! {
    FormulaCurveEventHandlerMut<'_> {
        curve::SetSamples,
        curve::formula::SetTrochoidProperties,
    }
}

delegate_events! {
    FormulaCurveEventHandler<'_> {
        curve::GetSamples,
    }
}

unimplemented_handlers! {
    FormulaCurveEventHandler<'_> {
        curve::control_points::GetControlPointsLength,
        curve::control_points::weighted::GetWeight,
    }
}

unimplemented_handlers_mut! {
    FormulaCurveEventHandlerMut<'_> {
        curve::control_points::DeletePoint,
        curve::control_points::MovePoint,
        curve::control_points::AddControlPoint,
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
    }
}
