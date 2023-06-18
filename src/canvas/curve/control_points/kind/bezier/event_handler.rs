use crate::canvas::curve::control_points::kind::bezier::Bezier;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetWeight, MovePoint,
};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{DelegateEventHandler, Event, EventHandler};

pub struct BezierEventHandler<'a> {
    curve: &'a mut Bezier,
}

impl<'a> BezierEventHandler<'a> {
    pub fn new(curve: &'a mut Bezier) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for BezierEventHandler<'a>
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
    BezierEventHandler<'_> {
        GetControlPointsLength,
        AddControlPoint,
        MovePoint,
        DeletePoint,
    }
}

unimplemented_handlers! {
    BezierEventHandler<'_> {
        AddWeightedControlPoint,
        ChangeWeight,
        GetWeight,
    }
}
