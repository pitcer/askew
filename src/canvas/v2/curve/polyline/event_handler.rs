use crate::canvas::curve::control_points::points::event_handler::{
    ControlPointsEventHandler, ControlPointsEventHandlerMut,
};
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::event::macros::{
    delegate_handlers, delegate_handlers_mut, unimplemented_handlers, unimplemented_handlers_mut,
};
use crate::event::{
    canvas, curve, DelegateEventHandler, DelegateEventHandlerMut, Event, EventHandler,
    EventHandlerMut, EventMut,
};

#[deprecated]
pub struct PolylineCurveEventHandler<'a> {
    curve: &'a PolylineCurve,
}

#[deprecated]
pub struct PolylineCurveEventHandlerMut<'a> {
    curve: &'a mut PolylineCurve,
}

impl<'a> PolylineCurveEventHandler<'a> {
    #[must_use]
    pub fn new(curve: &'a PolylineCurve) -> Self {
        Self { curve }
    }
}

impl<'a> PolylineCurveEventHandlerMut<'a> {
    pub fn new(curve: &'a mut PolylineCurve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for PolylineCurveEventHandler<'a>
where
    E: Event,
    for<'b> ControlPointsEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler(&self) -> Self::Delegate<'_> {
        self.curve.points.event_handler()
    }
}

impl<'a, E> DelegateEventHandlerMut<E> for PolylineCurveEventHandlerMut<'a>
where
    E: EventMut,
    for<'b> ControlPointsEventHandlerMut<'b>: EventHandlerMut<E>,
{
    type Delegate<'b> = ControlPointsEventHandlerMut<'b> where Self: 'b;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_> {
        self.curve.points.event_handler_mut()
    }
}

delegate_handlers! {
    PolylineCurveEventHandler<'_> {
        curve::control_points::GetControlPointsLength,

        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint,
    }
}

delegate_handlers_mut! {
    PolylineCurveEventHandlerMut<'_> {
        curve::control_points::AddControlPoint,
        curve::control_points::MovePoint,
        curve::control_points::DeletePoint,

        canvas::RotateCurve,
        canvas::MoveCurve,
    }
}

unimplemented_handlers! {
    PolylineCurveEventHandler<'_> {
        curve::control_points::weighted::GetWeight,
        curve::GetSamples,
    }
}

unimplemented_handlers_mut! {
    PolylineCurveEventHandlerMut<'_> {
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::SetSamples,
    }
}
