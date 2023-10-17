use crate::canvas::curve::Curve;
use crate::canvas::v2::curve::trochoid::event_handler::TrochoidCurveEventHandler;
use crate::event::macros::{delegate_events, unimplemented_handlers};
use crate::event::{curve, DelegateEvent, Error, Event, EventHandler, HandlerResult};

pub struct FormulaCurveEventHandler<'a> {
    curve: &'a mut Curve,
}

impl<'a> FormulaCurveEventHandler<'a> {
    pub fn new(curve: &'a mut Curve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEvent<E> for FormulaCurveEventHandler<'a>
where
    E: Event,
    for<'b> TrochoidCurveEventHandler<'b>: EventHandler<E>,
{
    fn delegate(&mut self, event: E) -> HandlerResult<E> {
        match self.curve {
            Curve::Trochoid(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

delegate_events! {
    FormulaCurveEventHandler<'_> {
        curve::SetSamples,
        curve::GetSamples,
        curve::formula::SetTrochoidProperties,
    }
}

unimplemented_handlers! {
    FormulaCurveEventHandler<'_> {
        curve::control_points::DeletePoint,
        curve::control_points::MovePoint,
        curve::control_points::AddControlPoint,
        curve::control_points::GetControlPointsLength,
        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::control_points::weighted::GetWeight,
    }
}
