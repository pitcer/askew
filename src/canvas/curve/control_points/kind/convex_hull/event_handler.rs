use crate::canvas::curve::control_points::kind::convex_hull::ConvexHull;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetSamples, GetWeight, MovePoint, SetSamples,
};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{DelegateEventHandler, Event, EventHandler};

pub struct ConvexHullEventHandler<'a> {
    curve: &'a mut ConvexHull,
}

impl<'a> ConvexHullEventHandler<'a> {
    pub fn new(curve: &'a mut ConvexHull) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for ConvexHullEventHandler<'a>
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
    ConvexHullEventHandler<'_> {
        GetControlPointsLength,
        AddControlPoint,
        MovePoint,
        DeletePoint,
    }
}

unimplemented_handlers! {
    ConvexHullEventHandler<'_> {
        AddWeightedControlPoint,
        ChangeWeight,
        GetWeight,

        SetSamples,
        GetSamples,
    }
}
