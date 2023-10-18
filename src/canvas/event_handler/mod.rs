use crate::canvas::curve::event_handler::{CurveEventHandler, CurveEventHandlerMut};
use crate::canvas::{math, Canvas};
use crate::event::canvas::{
    AddCurve, ChangeCurrentCurveIndex, DeleteCurve, GetCurvesLength, GetLength, GetPointOnCurve,
    MovePointOnCurve,
};
use crate::event::curve::control_points::{GetControlPointsLength, MovePoint};
use crate::event::curve::GetPoint;
use crate::event::{
    DelegateEventHandler, DelegateEventHandlerMut, Event, EventHandler, EventHandlerMut, EventMut,
    HandlerResult,
};

pub mod curve;

#[derive(Debug)]
pub struct CanvasEventHandler<'a> {
    canvas: &'a Canvas,
}

#[derive(Debug)]
pub struct CanvasEventHandlerMut<'a> {
    canvas: &'a mut Canvas,
}

impl<'a> CanvasEventHandler<'a> {
    #[must_use]
    pub fn new(canvas: &'a Canvas) -> Self {
        Self { canvas }
    }
}

impl<'a> CanvasEventHandlerMut<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        Self { canvas }
    }

    #[must_use]
    pub fn as_immut(&'a self) -> CanvasEventHandler<'a> {
        CanvasEventHandler::new(self.canvas)
    }
}

impl<E> DelegateEventHandler<E> for CanvasEventHandler<'_>
where
    E: Event,
    for<'b> CurveEventHandler<'b>: EventHandler<E>,
{
    type Delegate<'b> = CurveEventHandler<'b> where Self: 'b;

    fn delegate_handler(&self) -> Self::Delegate<'_> {
        self.canvas.current_curve().event_handler()
    }
}

impl<E> DelegateEventHandlerMut<E> for CanvasEventHandlerMut<'_>
where
    E: EventMut,
    for<'b> CurveEventHandlerMut<'b>: EventHandlerMut<E>,
{
    type Delegate<'b> = CurveEventHandlerMut<'b> where Self: 'b;

    fn delegate_handler_mut(&mut self) -> Self::Delegate<'_> {
        self.canvas.current_curve_mut().event_handler_mut()
    }
}

impl EventHandlerMut<AddCurve> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, _event: AddCurve) -> HandlerResult<AddCurve> {
        let curve_type = self.canvas.config.default_curve_type;
        let curve = Canvas::create_curve(&self.canvas.config, curve_type, None, None);
        self.canvas.curves.push(curve);
        self.canvas.properties.current_curve += 1;
        Ok(())
    }
}

impl EventHandlerMut<DeleteCurve> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, _event: DeleteCurve) -> HandlerResult<DeleteCurve> {
        let current_curve = self.canvas.properties.current_curve;
        self.canvas.curves.remove(current_curve);
        Ok(())
    }
}

impl EventHandlerMut<ChangeCurrentCurveIndex> for CanvasEventHandlerMut<'_> {
    fn handle_mut(
        &mut self,
        event: ChangeCurrentCurveIndex,
    ) -> HandlerResult<ChangeCurrentCurveIndex> {
        self.canvas.properties.current_curve = math::rem_euclid(
            self.canvas.properties.current_curve as isize + event.change as isize,
            self.canvas.curves.len() as isize,
        );
        Ok(())
    }
}

impl EventHandler<GetLength> for CanvasEventHandler<'_> {
    fn handle(&self, event: GetLength) -> HandlerResult<GetLength> {
        let length = self.canvas.curves[event.0].event_handler().handle(GetControlPointsLength)?;
        Ok(length)
    }
}

impl EventHandler<GetCurvesLength> for CanvasEventHandler<'_> {
    fn handle(&self, _event: GetCurvesLength) -> HandlerResult<GetCurvesLength> {
        let length = self.canvas.curves.len();
        Ok(length)
    }
}

impl EventHandler<GetPointOnCurve> for CanvasEventHandler<'_> {
    fn handle(&self, event: GetPointOnCurve) -> HandlerResult<GetPointOnCurve> {
        let point = self.canvas.curves[event.0].event_handler().handle(GetPoint(event.1))?;
        Ok(point)
    }
}

impl EventHandlerMut<MovePointOnCurve> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: MovePointOnCurve) -> HandlerResult<MovePointOnCurve> {
        let point = self.canvas.curves[event.0].event_handler().handle(GetPoint(event.1))?;
        let shift = event.2 - point;
        self.canvas.curves[event.0]
            .event_handler_mut()
            .handle_mut(MovePoint::new(event.1, shift))?;
        Ok(())
    }
}
