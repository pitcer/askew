use crate::canvas::curve::control_points::event_handler::{
    ControlPointsCurveEventHandler, ControlPointsCurveEventHandlerMut,
};
use crate::canvas::curve::formula::event_handler::{
    FormulaCurveEventHandler, FormulaCurveEventHandlerMut,
};
use crate::canvas::curve::Curve;
use crate::canvas::v2::Update;
use crate::event::canvas::{
    GetConvexHull, GetCurveCenter, MoveCurve, RotateCurve, SelectPoint, SetConvexHull,
};
use crate::event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes};
use crate::event::curve::formula::SetTrochoidProperties;
use crate::event::curve::GetPoint;
use crate::event::macros::{delegate_events, delegate_events_mut};
use crate::event::{
    curve, DelegateEvent, DelegateEventMut, Error, Event, EventHandler, EventHandlerMut, EventMut,
    HandlerResult,
};

pub struct CurveEventHandler<'a> {
    curve: &'a Curve,
}

pub struct CurveEventHandlerMut<'a> {
    curve: &'a mut Curve,
}

impl<'a> CurveEventHandler<'a> {
    #[must_use]
    pub fn new(curve: &'a Curve) -> Self {
        Self { curve }
    }
}

impl<'a> CurveEventHandlerMut<'a> {
    pub fn new(curve: &'a mut Curve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEvent<E> for CurveEventHandler<'a>
where
    E: Event,
    for<'b> FormulaCurveEventHandler<'b>: EventHandler<E>,
    for<'b> ControlPointsCurveEventHandler<'b>: EventHandler<E>,
{
    fn delegate(&self, event: E) -> HandlerResult<E> {
        match self.curve {
            Curve::Polyline(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandler::new(self.curve).handle(event)
            }
            Curve::Trochoid(_) => FormulaCurveEventHandler::new(self.curve).handle(event),
        }
    }
}

impl<'a, E> DelegateEventMut<E> for CurveEventHandlerMut<'a>
where
    E: EventMut,
    for<'b> FormulaCurveEventHandlerMut<'b>: EventHandlerMut<E>,
    for<'b> ControlPointsCurveEventHandlerMut<'b>: EventHandlerMut<E>,
{
    fn delegate_mut(&mut self, event: E) -> HandlerResult<E> {
        match self.curve {
            Curve::Polyline(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandlerMut::new(self.curve).handle_mut(event)
            }
            Curve::Trochoid(_) => FormulaCurveEventHandlerMut::new(self.curve).handle_mut(event),
        }
    }
}

impl EventHandlerMut<SetInterpolationNodes> for CurveEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: SetInterpolationNodes) -> HandlerResult<SetInterpolationNodes> {
        match self.curve {
            Curve::Polyline(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandlerMut::new(self.curve).handle_mut(event)
            }
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetInterpolationNodes> for CurveEventHandler<'_> {
    fn handle(&self, event: GetInterpolationNodes) -> HandlerResult<GetInterpolationNodes> {
        match self.curve {
            Curve::Polyline(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandler::new(self.curve).handle(event)
            }
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandlerMut<MoveCurve> for CurveEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: MoveCurve) -> HandlerResult<MoveCurve> {
        match self.curve {
            Curve::Polyline(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandlerMut::new(self.curve).handle_mut(event)
            }
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandlerMut<RotateCurve> for CurveEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: RotateCurve) -> HandlerResult<RotateCurve> {
        match self.curve {
            Curve::Polyline(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandlerMut::new(self.curve).handle_mut(event)
            }
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetCurveCenter> for CurveEventHandler<'_> {
    fn handle(&self, event: GetCurveCenter) -> HandlerResult<GetCurveCenter> {
        match self.curve {
            Curve::Trochoid(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandler::new(self.curve).handle(event)
            }
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetPoint> for CurveEventHandler<'_> {
    fn handle(&self, event: GetPoint) -> HandlerResult<GetPoint> {
        match self.curve {
            Curve::Trochoid(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandler::new(self.curve).handle(event)
            }
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<SelectPoint> for CurveEventHandler<'_> {
    fn handle(&self, event: SelectPoint) -> HandlerResult<SelectPoint> {
        match self.curve {
            Curve::Trochoid(_)
            | Curve::Interpolation(_)
            | Curve::Bezier(_)
            | Curve::RationalBezier(_) => {
                ControlPointsCurveEventHandler::new(self.curve).handle(event)
            }
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandlerMut<SetTrochoidProperties> for CurveEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: SetTrochoidProperties) -> HandlerResult<SetTrochoidProperties> {
        match self.curve {
            Curve::Trochoid(_) => FormulaCurveEventHandlerMut::new(self.curve).handle_mut(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetConvexHull> for CurveEventHandler<'_> {
    fn handle(&self, _event: GetConvexHull) -> HandlerResult<GetConvexHull> {
        match &self.curve {
            Curve::Polyline(curve) => Ok(curve.control_points.convex_hull.properties.visible),
            Curve::Interpolation(curve) => Ok(curve.control_points.convex_hull.properties.visible),
            Curve::Bezier(curve) => Ok(curve.control_points.convex_hull.properties.visible),
            Curve::RationalBezier(curve) => Ok(curve.control_points.convex_hull.properties.visible),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandlerMut<SetConvexHull> for CurveEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: SetConvexHull) -> HandlerResult<SetConvexHull> {
        match self.curve {
            Curve::Polyline(curve) => {
                curve.control_points.convex_hull.properties.visible = event.0;
                curve.update();
            }
            Curve::Interpolation(curve) => {
                curve.control_points.convex_hull.properties.visible = event.0;
                curve.update();
            }
            Curve::Bezier(curve) => {
                curve.control_points.convex_hull.properties.visible = event.0;
                curve.update();
            }
            Curve::RationalBezier(curve) => {
                curve.control_points.convex_hull.properties.visible = event.0;
                curve.update();
            }
            _ => return Err(Error::Unimplemented),
        }
        Ok(())
    }
}

delegate_events! {
    CurveEventHandler<'_> {
        curve::control_points::GetControlPointsLength,

        curve::control_points::weighted::GetWeight,

        curve::GetSamples,
    }
}

delegate_events_mut! {
    CurveEventHandlerMut<'_> {
        curve::control_points::DeletePoint,
        curve::control_points::MovePoint,
        curve::control_points::AddControlPoint,

        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,

        curve::SetSamples,
    }
}
