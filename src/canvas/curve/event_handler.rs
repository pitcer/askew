use crate::canvas::curve::control_points::event_handler::ControlPointsCurveEventHandler;
use crate::canvas::curve::formula::event_handler::FormulaCurveEventHandler;
use crate::canvas::curve::CurveKind;
use crate::event::canvas::{GetCurveCenter, MoveCurve, RotateCurve, SelectPoint};
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::GetPoint;
use crate::event::macros::delegate_events;
use crate::event::{curve, DelegateEvent, Error, Event, EventHandler, HandlerResult};

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

impl EventHandler<SetInterpolationNodes> for CurveEventHandler<'_> {
    fn handle(&mut self, event: SetInterpolationNodes) -> HandlerResult<SetInterpolationNodes> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetInterpolationNodes> for CurveEventHandler<'_> {
    fn handle(&mut self, event: GetInterpolationNodes) -> HandlerResult<GetInterpolationNodes> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<MoveCurve> for CurveEventHandler<'_> {
    fn handle(&mut self, event: MoveCurve) -> HandlerResult<MoveCurve> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<RotateCurve> for CurveEventHandler<'_> {
    fn handle(&mut self, event: RotateCurve) -> HandlerResult<RotateCurve> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetCurveCenter> for CurveEventHandler<'_> {
    fn handle(&mut self, event: GetCurveCenter) -> HandlerResult<GetCurveCenter> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetPoint> for CurveEventHandler<'_> {
    fn handle(&mut self, event: GetPoint) -> HandlerResult<GetPoint> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<SelectPoint> for CurveEventHandler<'_> {
    fn handle(&mut self, event: SelectPoint) -> HandlerResult<SelectPoint> {
        match self.curve {
            CurveKind::ControlPoints(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
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
