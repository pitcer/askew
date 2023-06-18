use crate::canvas::curve::control_points::kind::interpolation::Interpolation;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetSamples, GetWeight, MovePoint, SetSamples,
};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{DelegateEventHandler, Event, EventHandler, HandlerResult};

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
        self.curve.samples = event.0;
        Ok(())
    }
}

impl EventHandler<GetSamples> for InterpolationEventHandler<'_> {
    fn handle(&mut self, _event: GetSamples) -> HandlerResult<GetSamples> {
        Ok(self.curve.samples)
    }
}

delegate_handlers! {
    InterpolationEventHandler<'_> {
        GetControlPointsLength,
        AddControlPoint,
        MovePoint,
        DeletePoint,
    }
}

unimplemented_handlers! {
    InterpolationEventHandler<'_> {
        AddWeightedControlPoint,
        ChangeWeight,
        GetWeight,
    }
}
