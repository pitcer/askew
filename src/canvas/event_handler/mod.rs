use crate::canvas::curve::control_points::polyline::Polyline;
use crate::canvas::curve::control_points::{ControlPointsCurve, CurvePoints};
use crate::canvas::curve::Curve;
use crate::canvas::Canvas;
use crate::event::canvas::{
    AddCurve, ChangeCurrentCurveIndex, DeleteCurve, GetConvexHull, SetConvexHull,
};
use crate::event::{EventHandler, HandlerResult};
use crate::ui::frame::mode::Mode;

pub mod curve;

pub struct CanvasEventHandler<'a> {
    canvas: &'a mut Canvas,
    mode: Mode,
}

impl<'a> CanvasEventHandler<'a> {
    pub fn new(canvas: &'a mut Canvas, mode: Mode) -> Self {
        Self { canvas, mode }
    }
}

impl<'a> EventHandler<GetConvexHull> for CanvasEventHandler<'a> {
    fn handle(&mut self, _event: GetConvexHull) -> HandlerResult<GetConvexHull> {
        Ok(self.canvas.properties.show_convex_hull)
    }
}

impl<'a> EventHandler<SetConvexHull> for CanvasEventHandler<'a> {
    fn handle(&mut self, event: SetConvexHull) -> HandlerResult<SetConvexHull> {
        self.canvas.properties.show_convex_hull = event.0;
        Ok(())
    }
}

impl EventHandler<AddCurve> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: AddCurve) -> HandlerResult<AddCurve> {
        self.canvas
            .curves
            .push(Curve::ControlPoints(ControlPointsCurve::Polyline(
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
