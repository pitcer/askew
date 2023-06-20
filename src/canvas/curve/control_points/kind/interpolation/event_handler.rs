use crate::canvas::curve::control_points::kind::interpolation::Interpolation;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::{GetSamples, SetSamples};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{canvas, curve, DelegateEventHandler, Event, EventHandler, HandlerResult};

pub struct InterpolationEventHandler<'a> {
    curve: &'a mut Interpolation,
}

impl<'a> InterpolationEventHandler<'a> {
    pub fn new(curve: &'a mut Interpolation) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for InterpolationEventHandler<'a>
where
    E: Event,
    for<'b> ControlPointsEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler(&mut self) -> Self::Delegate<'_> {
        self.curve.points.event_handler()
    }
}

impl EventHandler<SetSamples> for InterpolationEventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<GetSamples> for InterpolationEventHandler<'_> {
    fn handle(&mut self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<SetInterpolationNodes> for InterpolationEventHandler<'_> {
    fn handle(&mut self, event: SetInterpolationNodes) -> HandlerResult<SetInterpolationNodes> {
        self.curve.nodes = event.nodes;
        Ok(())
    }
}

impl EventHandler<GetInterpolationNodes> for InterpolationEventHandler<'_> {
    fn handle(&mut self, _event: GetInterpolationNodes) -> HandlerResult<GetInterpolationNodes> {
        Ok(self.curve.nodes)
    }
}

delegate_handlers! {
    InterpolationEventHandler<'_> {
        curve::control_points::GetControlPointsLength,
        curve::control_points::AddControlPoint,
        curve::control_points::MovePoint,
        curve::control_points::DeletePoint,

        canvas::RotateCurve,
        canvas::MoveCurve,
        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint,
    }
}

unimplemented_handlers! {
    InterpolationEventHandler<'_> {
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::control_points::weighted::GetWeight,
    }
}