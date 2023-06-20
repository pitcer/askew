use crate::canvas::curve::control_points::kind::bezier::Bezier;
use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::event::curve::control_points::weighted;
use crate::event::curve::{control_points, GetSamples, SetSamples};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{canvas, curve, DelegateEventHandler, Event, EventHandler, HandlerResult};

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

impl EventHandler<SetSamples> for BezierEventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<GetSamples> for BezierEventHandler<'_> {
    fn handle(&mut self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

delegate_handlers! {
    BezierEventHandler<'_> {
        control_points::GetControlPointsLength,
        control_points::AddControlPoint,
        control_points::MovePoint,
        control_points::DeletePoint,

        canvas::RotateCurve,
        canvas::MoveCurve,
        canvas::GetCurveCenter,
        canvas::SelectPoint,
        curve::GetPoint
    }
}

unimplemented_handlers! {
    BezierEventHandler<'_> {
        weighted::AddWeightedControlPoint,
        weighted::ChangeWeight,
        weighted::GetWeight,
    }
}
