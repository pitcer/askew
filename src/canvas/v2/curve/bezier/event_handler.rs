use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::canvas::v2::curve::bezier::BezierCurve;
use crate::event::curve::control_points::weighted;
use crate::event::curve::{control_points, GetSamples, SetSamples};
use crate::event::macros::{
    delegate_handlers, delegate_handlers_mut, unimplemented_handlers, unimplemented_handlers_mut,
};
use crate::event::{
    canvas, curve, DelegateEventHandler, DelegateEventHandlerMut, Event, EventHandler,
    EventHandlerMut, EventMut, HandlerResult,
};

pub struct BezierCurveEventHandler<'a> {
    curve: &'a mut BezierCurve,
}

impl<'a> BezierCurveEventHandler<'a> {
    pub fn new(curve: &'a mut BezierCurve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for BezierCurveEventHandler<'a>
where
    E: Event,
    for<'b> ControlPointsEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler(&self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

impl<'a, E> DelegateEventHandlerMut<E> for BezierCurveEventHandler<'a>
where
    E: EventMut,
    for<'b> ControlPointsEventHandler<'b>: EventHandlerMut<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

impl EventHandlerMut<SetSamples> for BezierCurveEventHandler<'_> {
    fn handle_mut(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle_mut(event)
    }
}

impl EventHandler<GetSamples> for BezierCurveEventHandler<'_> {
    fn handle(&self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

delegate_handlers! {
    BezierCurveEventHandler<'_> {
        control_points::GetControlPointsLength,

        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint
    }
}

delegate_handlers_mut! {
    BezierCurveEventHandler<'_> {
        control_points::AddControlPoint,
        control_points::MovePoint,
        control_points::DeletePoint,

        canvas::RotateCurve,
        canvas::MoveCurve,
    }
}

unimplemented_handlers! {
    BezierCurveEventHandler<'_> {
        weighted::GetWeight,
    }
}

unimplemented_handlers_mut! {
    BezierCurveEventHandler<'_> {
        weighted::AddWeightedControlPoint,
        weighted::ChangeWeight,
    }
}
