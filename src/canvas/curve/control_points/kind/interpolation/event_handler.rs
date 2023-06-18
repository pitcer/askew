use crate::canvas::curve::control_points::kind::interpolation::Interpolation;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetWeight, MovePoint,
};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{DelegateEventHandler, Event, EventHandler};

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
