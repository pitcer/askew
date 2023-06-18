use crate::{
    canvas::curve::control_points::kind::bezier::event_handler::BezierEventHandler,
    canvas::curve::control_points::kind::convex_hull::event_handler::ConvexHullEventHandler,
    canvas::curve::control_points::kind::interpolation::event_handler::InterpolationEventHandler,
    canvas::curve::control_points::kind::polyline::event_handler::PolylineEventHandler,
    canvas::curve::control_points::kind::rational_bezier::event_handler::RationalBezierEventHandler,
    canvas::curve::control_points::ControlPointsCurveKind,
    event::curve::{
        AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint,
        GetControlPointsLength, GetWeight, MovePoint,
    },
    event::macros::delegate_events,
    event::{DelegateEvent, Event, EventHandler, HandlerResult},
};

pub struct CurveEventHandler<'a> {
    curve: &'a mut ControlPointsCurveKind,
}

impl<'a> CurveEventHandler<'a> {
    pub fn new(curve: &'a mut ControlPointsCurveKind) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEvent<E> for CurveEventHandler<'a>
where
    E: Event,
    for<'b> PolylineEventHandler<'b>: EventHandler<E>,
    for<'b> ConvexHullEventHandler<'b>: EventHandler<E>,
    for<'b> InterpolationEventHandler<'b>: EventHandler<E>,
    for<'b> BezierEventHandler<'b>: EventHandler<E>,
    for<'b> RationalBezierEventHandler<'b>: EventHandler<E>,
{
    fn delegate(&mut self, event: E) -> HandlerResult<E> {
        match self.curve {
            ControlPointsCurveKind::Polyline(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::ConvexHull(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::Interpolation(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::Bezier(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::RationalBezier(curve) => curve.event_handler().handle(event),
        }
    }
}

delegate_events! {
    CurveEventHandler<'_> {
        GetControlPointsLength,
        AddControlPoint,
        MovePoint,
        DeletePoint,
        AddWeightedControlPoint,
        ChangeWeight,
        GetWeight,
    }
}
