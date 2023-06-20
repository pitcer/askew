use crate::canvas::curve::event_handler::CurveEventHandler;
use crate::canvas::{math, Canvas};
use crate::event::canvas::{
    AddCurve, ChangeCurrentCurveIndex, DeleteCurve, GetConvexHull, SetConvexHull,
};
use crate::event::{DelegateEventHandler, Event, EventHandler, HandlerResult};

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
        let curve = self.canvas.create_curve(curve_type, None, None);
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
