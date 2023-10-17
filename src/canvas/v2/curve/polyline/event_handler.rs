use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::event::macros::{
    delegate_handlers, delegate_handlers_mut, unimplemented_handlers, unimplemented_handlers_mut,
};
use crate::event::{
    canvas, curve, DelegateEventHandler, DelegateEventHandlerMut, Event, EventHandler,
    EventHandlerMut, EventMut,
};

pub struct PolylineCurveEventHandler<'a> {
    curve: &'a mut PolylineCurve,
}

impl<'a> PolylineCurveEventHandler<'a> {
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
        self.curve.control_points.points.event_handler()
    }
}

impl<'a, E> DelegateEventHandlerMut<E> for PolylineCurveEventHandler<'a>
where
    E: EventMut,
    for<'b> ControlPointsEventHandler<'b>: EventHandlerMut<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
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
    PolylineCurveEventHandler<'_> {
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
    PolylineCurveEventHandler<'_> {
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::SetSamples,
    }
}
