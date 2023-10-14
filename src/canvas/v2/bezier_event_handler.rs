use crate::canvas::curve::control_points::points::event_handler::ControlPointsEventHandler;
use crate::canvas::v2::bezier_curve::BezierCurve;
use crate::event::curve::control_points::weighted;
use crate::event::curve::{control_points, GetSamples, SetSamples};
use crate::event::macros::{delegate_handlers, unimplemented_handlers};
use crate::event::{canvas, curve, DelegateEventHandler, Event, EventHandler, HandlerResult};

pub struct BezierV2EventHandler<'a> {
    curve: &'a mut BezierCurve,
}

impl<'a> BezierV2EventHandler<'a> {
    pub fn new(curve: &'a mut BezierCurve) -> Self {
        Self { curve }
    }
}

impl<'a, E> DelegateEventHandler<E> for BezierV2EventHandler<'a>
where
    E: Event,
    for<'b> ControlPointsEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = ControlPointsEventHandler<'b> where Self: 'b;

    fn delegate_handler(&mut self) -> Self::Delegate<'_> {
        self.curve.control_points.points.event_handler()
    }
}

impl EventHandler<SetSamples> for BezierV2EventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

impl EventHandler<GetSamples> for BezierV2EventHandler<'_> {
    fn handle(&mut self, event: GetSamples) -> HandlerResult<GetSamples> {
        self.curve.samples.event_handler().handle(event)
    }
}

delegate_handlers! {
    BezierV2EventHandler<'_> {
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
    BezierV2EventHandler<'_> {
        weighted::AddWeightedControlPoint,
        weighted::ChangeWeight,
        weighted::GetWeight,
    }
}
