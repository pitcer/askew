use crate::canvas::curve::control_points::kind::polyline::Polyline;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{curve, DelegateEventHandler, Event, EventHandler};

pub struct PolylineEventHandler<'a> {
    curve: &'a mut Polyline,
}

impl<'a> PolylineEventHandler<'a> {
    pub fn new(curve: &'a mut Polyline) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for PolylineEventHandler<'a>
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
    PolylineEventHandler<'_> {
        curve::control_points::GetControlPointsLength,
        curve::control_points::AddControlPoint,
        curve::control_points::MovePoint,
        curve::control_points::DeletePoint,
    }
}

unimplemented_handlers! {
    PolylineEventHandler<'_> {
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::control_points::weighted::GetWeight,
        curve::SetSamples,
        curve::GetSamples,
    }
}
