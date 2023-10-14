use crate::canvas::curve::control_points::event_handler::ControlPointsCurveEventHandler;
use crate::canvas::curve::control_points::ControlPointsCurveKind;
use crate::canvas::curve::formula::event_handler::FormulaCurveEventHandler;
use crate::canvas::curve::CurveKind;
use crate::canvas::v2::Update;
use crate::event::canvas::{
    GetConvexHull, GetCurveCenter, MoveCurve, RotateCurve, SelectPoint, SetConvexHull,
};
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::formula::SetTrochoidProperties;
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

impl EventHandler<SetTrochoidProperties> for CurveEventHandler<'_> {
    fn handle(&mut self, event: SetTrochoidProperties) -> HandlerResult<SetTrochoidProperties> {
        match self.curve {
            CurveKind::Formula(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetConvexHull> for CurveEventHandler<'_> {
    fn handle(&mut self, _event: GetConvexHull) -> HandlerResult<GetConvexHull> {
        match self.curve {
            CurveKind::ControlPoints(curve) => match curve {
                ControlPointsCurveKind::PolylineV2(curve) => {
                    Ok(curve.control_points.convex_hull.properties.visible)
                }
                ControlPointsCurveKind::BezierV2(curve) => {
                    Ok(curve.control_points.convex_hull.properties.visible)
                }
                _ => Err(Error::Unimplemented),
            },
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<SetConvexHull> for CurveEventHandler<'_> {
    fn handle(&mut self, event: SetConvexHull) -> HandlerResult<SetConvexHull> {
        match self.curve {
            CurveKind::ControlPoints(curve) => match curve {
                ControlPointsCurveKind::PolylineV2(curve) => {
                    curve.control_points.convex_hull.properties.visible = event.0;
                    curve.update();
                }
                ControlPointsCurveKind::BezierV2(curve) => {
                    curve.control_points.convex_hull.properties.visible = event.0;
                    curve.update();
                }
                _ => return Err(Error::Unimplemented),
            },
            _ => return Err(Error::Unimplemented),
        }
        Ok(())
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
