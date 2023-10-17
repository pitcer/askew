use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::canvas::v2::curve::interpolation::InterpolationCurve;
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::macros::{
    delegate_handlers, delegate_handlers_mut, unimplemented_handlers, unimplemented_handlers_mut,
};
use crate::event::{
    canvas, curve, DelegateEventHandler, DelegateEventHandlerMut, Event, EventHandler,
    EventHandlerMut, EventMut, HandlerResult,
};

pub struct InterpolationCurveEventHandler<'a> {
    curve: &'a mut InterpolationCurve,
}

impl<'a> InterpolationCurveEventHandler<'a> {
    pub fn new(curve: &'a mut InterpolationCurve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for InterpolationCurveEventHandler<'a>
where
    E: Event,
    for<'b> ControlPointsEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler(&self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

impl<'a, E> DelegateEventHandlerMut<E> for InterpolationCurveEventHandler<'a>
where
    E: EventMut,
    for<'b> ControlPointsEventHandler<'b>: EventHandlerMut<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

impl EventHandlerMut<SetSamples> for InterpolationCurveEventHandler<'_> {
    fn handle_mut(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle_mut(event)
    }
}

impl EventHandler<GetSamples> for InterpolationCurveEventHandler<'_> {
    fn handle(&self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandlerMut<SetInterpolationNodes> for InterpolationCurveEventHandler<'_> {
    fn handle_mut(&mut self, event: SetInterpolationNodes) -> HandlerResult<SetInterpolationNodes> {
        self.curve.properties.nodes = event.nodes;
        Ok(())
    }
}

impl EventHandler<GetInterpolationNodes> for InterpolationCurveEventHandler<'_> {
    fn handle(&self, _event: GetInterpolationNodes) -> HandlerResult<GetInterpolationNodes> {
        Ok(self.curve.properties.nodes)
    }
}

delegate_handlers! {
    InterpolationCurveEventHandler<'_> {
        curve::control_points::GetControlPointsLength,

        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint,
    }
}

delegate_handlers_mut! {
    InterpolationCurveEventHandler<'_> {
        curve::control_points::AddControlPoint,
        curve::control_points::MovePoint,
        curve::control_points::DeletePoint,

        canvas::RotateCurve,
        canvas::MoveCurve,
    }
}

unimplemented_handlers! {
    InterpolationCurveEventHandler<'_> {
        curve::control_points::weighted::GetWeight,
    }
}

unimplemented_handlers_mut! {
    InterpolationCurveEventHandler<'_> {
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
    }
}
