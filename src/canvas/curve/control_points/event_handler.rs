use crate::canvas::v2::curve::bezier::event_handler::BezierV2EventHandler;
use crate::canvas::v2::curve::interpolation::event_handler::InterpolationCurveEventHandler;
use crate::canvas::v2::curve::polyline::event_handler::PolylineCurveEventHandler;
use crate::canvas::v2::curve::rational_bezier::event_handler::RationalBezierCurveEventHandler;
use crate::{
    canvas::curve::control_points::kind::bezier::event_handler::BezierEventHandler,
    canvas::curve::control_points::kind::convex_hull::event_handler::ConvexHullEventHandler,
    canvas::curve::control_points::kind::polyline::event_handler::PolylineEventHandler,
    canvas::curve::control_points::ControlPointsCurveKind,
    event::curve::control_points::{GetInterpolationNodes, SetInterpolationNodes},
    event::macros::delegate_events,
    event::{canvas, curve, Error},
    event::{DelegateEvent, Event, EventHandler, HandlerResult},
};

pub struct ControlPointsCurveEventHandler<'a> {
    curve: &'a mut ControlPointsCurveKind,
}

impl<'a> ControlPointsCurveEventHandler<'a> {
    pub fn new(curve: &'a mut ControlPointsCurveKind) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEvent<E> for ControlPointsCurveEventHandler<'a>
where
    E: Event,
    for<'b> PolylineEventHandler<'b>: EventHandler<E>,
    for<'b> PolylineCurveEventHandler<'b>: EventHandler<E>,
    for<'b> ConvexHullEventHandler<'b>: EventHandler<E>,
    for<'b> InterpolationCurveEventHandler<'b>: EventHandler<E>,
    for<'b> BezierEventHandler<'b>: EventHandler<E>,
    for<'b> BezierV2EventHandler<'b>: EventHandler<E>,
    for<'b> RationalBezierCurveEventHandler<'b>: EventHandler<E>,
{
    fn delegate(&mut self, event: E) -> HandlerResult<E> {
        match self.curve {
            ControlPointsCurveKind::Polyline(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::PolylineV2(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::ConvexHull(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::Interpolation(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::Bezier(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::RationalBezier(curve) => curve.event_handler().handle(event),
            ControlPointsCurveKind::BezierV2(curve) => curve.event_handler().handle(event),
        }
    }
}

impl EventHandler<SetInterpolationNodes> for ControlPointsCurveEventHandler<'_> {
    fn handle(&mut self, event: SetInterpolationNodes) -> HandlerResult<SetInterpolationNodes> {
        match self.curve {
            ControlPointsCurveKind::Interpolation(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

impl EventHandler<GetInterpolationNodes> for ControlPointsCurveEventHandler<'_> {
    fn handle(&mut self, event: GetInterpolationNodes) -> HandlerResult<GetInterpolationNodes> {
        match self.curve {
            ControlPointsCurveKind::Interpolation(curve) => curve.event_handler().handle(event),
            _ => Err(Error::Unimplemented),
        }
    }
}

delegate_events! {
    ControlPointsCurveEventHandler<'_> {
        curve::control_points::GetControlPointsLength,
        curve::control_points::AddControlPoint,
        curve::control_points::MovePoint,
        curve::control_points::DeletePoint,

        curve::control_points::weighted::AddWeightedControlPoint,
        curve::control_points::weighted::ChangeWeight,
        curve::control_points::weighted::GetWeight,

        curve::SetSamples,
        curve::GetSamples,

        canvas::RotateCurve,
        canvas::MoveCurve,
        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint,
    }
}
