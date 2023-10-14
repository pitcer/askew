use crate::canvas::curve::control_points::kind::polyline::Polyline;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::canvas::v2::curve::polyline::PolylineCurve;
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{canvas, curve, DelegateEventHandler, Event, EventHandler};

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

    fn delegate_handler(&mut self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

delegate_handlers! {
    PolylineCurveEventHandler<'_> {
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
    PolylineCurveEventHandler<'_> {
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::control_points::weighted::GetWeight,
        curve::SetSamples,
        curve::GetSamples,
    }
}
