use crate::canvas::curve::control_points::event_handler::ControlPointsCurveEventHandler;
use crate::canvas::curve::formula::event_handler::FormulaCurveEventHandler;
use crate::canvas::curve::CurveKind;
use crate::event::macros::delegate_events;
use crate::event::{curve, DelegateEvent, Event, EventHandler, HandlerResult};

pub struct CurveEventHandler<'a> {
    curve: &'a mut CurveKind,
}

impl<'a> CurveEventHandler<'a> {
    pub fn new(curve: &'a mut CurveKind) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEvent<E> for CurveEventHandler<'a>
where
    E: Event,
    for<'b> FormulaCurveEventHandler<'b>: EventHandler<E>,
    for<'b> ControlPointsCurveEventHandler<'b>: EventHandler<E>,
{
    fn delegate(&mut self, event: E) -> HandlerResult<E> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            CurveKind::Formula(curve) => curve.event_handler().handle(event),
        }
    }
}

delegate_events! {
    CurveEventHandler<'_> {
        curve::control_points::DeletePoint,
        curve::control_points::MovePoint,
        curve::control_points::AddControlPoint,
        curve::control_points::GetControlPointsLength,
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::control_points::weighted::GetWeight,
        curve::SetSamples,
        curve::GetSamples,
    }
}
