use crate::canvas::curve::event_handler::CurveEventHandler;
use crate::canvas::{math, Canvas};
use crate::event::canvas::{
    AddCurve, ChangeCurrentCurveIndex, DeleteCurve, GetConvexHull, GetCurvesLength, GetLength,
    GetPointOnCurve, MovePointOnCurve, SetConvexHull,
};
use crate::event::curve::control_points::{GetControlPointsLength, MovePoint};
use crate::event::curve::GetPoint;
use crate::event::{DelegateEventHandler, Error, Event, EventHandler, HandlerResult};

pub mod curve;

#[derive(Debug)]
pub struct CanvasEventHandler<'a> {
    canvas: &'a mut Canvas,
}

impl<'a> CanvasEventHandler<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        Self { canvas }
    }
}

impl<E> DelegateEventHandler<E> for CanvasEventHandler<'_>
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
        let curve_type = self.canvas.properties.default_curve_type;
        let curve = Canvas::create_curve(
            &self.canvas.properties,
            &self.canvas.config,
            curve_type,
            None,
            None,
        );
        self.canvas.curves.push(curve);
        self.canvas.properties.current_curve += 1;
        Ok(())
    }
}

impl EventHandler<DeleteCurve> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: DeleteCurve) -> HandlerResult<DeleteCurve> {
        let current_curve = self.canvas.properties.current_curve;
        self.canvas.curves.remove(current_curve);
        Ok(())
    }
}

impl EventHandler<ChangeCurrentCurveIndex> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: ChangeCurrentCurveIndex) -> HandlerResult<ChangeCurrentCurveIndex> {
        self.canvas.properties.current_curve = math::rem_euclid(
            self.canvas.properties.current_curve as isize + event.change as isize,
            self.canvas.curves.len() as isize,
        );
        Ok(())
    }
}

impl EventHandler<GetLength> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: GetLength) -> HandlerResult<GetLength> {
        let length = self.canvas.curves[event.0].event_handler().handle(GetControlPointsLength)?;
        Ok(length)
    }
}

impl EventHandler<GetCurvesLength> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: GetCurvesLength) -> HandlerResult<GetCurvesLength> {
        let length = self.canvas.curves.len();
        Ok(length)
    }
}

impl EventHandler<GetPointOnCurve> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: GetPointOnCurve) -> HandlerResult<GetPointOnCurve> {
        let point = self.canvas.curves[event.0].event_handler().handle(GetPoint(event.1))?;
        Ok(point)
    }
}

impl EventHandler<MovePointOnCurve> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: MovePointOnCurve) -> HandlerResult<MovePointOnCurve> {
        let point = self.canvas.curves[event.0].event_handler().handle(GetPoint(event.1))?;
        let shift = event.2 - point;
        self.canvas.curves[event.0].event_handler().handle(MovePoint::new(event.1, shift))?;
        Ok(())
    }
}
