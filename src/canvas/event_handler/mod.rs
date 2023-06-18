use crate::canvas::curve::control_points::kind::polyline::Polyline;
use crate::canvas::curve::control_points::{ControlPointsCurveKind, CurvePoints};
use crate::canvas::curve::event_handler::CurveEventHandler;
use crate::canvas::curve::CurveKind;
use crate::canvas::Canvas;
use crate::event::canvas::{
    AddCurve, ChangeCurrentCurveIndex, DeleteCurve, GetConvexHull, SetConvexHull,
};
use crate::event::{DelegateEventHandler, Event, EventHandler, HandlerResult};

pub mod curve;

pub struct CanvasEventHandler<'a> {
    canvas: &'a mut Canvas,
}

impl<'a> CanvasEventHandler<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        Self { canvas }
    }
}

impl<'a, E> DelegateEventHandler<E> for CanvasEventHandler<'a>
where
    E: Event,
    for<'b> CurveEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = CurveEventHandler<'b> where Self: 'b;

    fn delegate_handler(&mut self) -> Self::Delegate<'_> {
        self.canvas.current_curve_mut().event_handler()
    }
}

impl EventHandler<GetConvexHull> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: GetConvexHull) -> HandlerResult<GetConvexHull> {
        Ok(self.canvas.properties.show_convex_hull)
    }
}

impl EventHandler<SetConvexHull> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: SetConvexHull) -> HandlerResult<SetConvexHull> {
        self.canvas.properties.show_convex_hull = event.0;
        Ok(())
    }
}

impl EventHandler<AddCurve> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: AddCurve) -> HandlerResult<AddCurve> {
        self.canvas
            .curves
            .push(CurveKind::ControlPoints(ControlPointsCurveKind::Polyline(
                Polyline::new(CurvePoints::new(vec![])),
            )));
        self.canvas.properties.current_curve += 1;
        Ok(())
    }
}

impl EventHandler<DeleteCurve> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: DeleteCurve) -> HandlerResult<DeleteCurve> {
        // TODO: delete curve
        Ok(())
    }
}

impl EventHandler<ChangeCurrentCurveIndex> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: ChangeCurrentCurveIndex) -> HandlerResult<ChangeCurrentCurveIndex> {
        self.canvas.properties.current_curve =
            (self.canvas.properties.current_curve as i32 + event.change) as usize;
        Ok(())
    }
}
