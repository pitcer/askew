use crate::canvas::curve::control_points::WeightedPoint;
use crate::canvas::curve::Curve;
use crate::canvas::event_handler::{CanvasEventHandler, CanvasEventHandlerMut};
use crate::canvas::{math, Canvas};
use crate::event::canvas::{
    AddPoint, ChangeCurrentPointIndex, ChangeCurrentPointWeight, DeleteCurrentPoint,
    GetCurrentPoint, GetCurveType, MoveCurrentPoint, RotateCurve, RotateCurveById, SetCurveType,
};
use crate::event::curve::control_points::weighted::{
    AddWeightedControlPoint, ChangeWeight, GetWeight,
};
use crate::event::curve::control_points::{
    AddControlPoint, DeletePoint, GetControlPointsLength, MovePoint,
};
use crate::event::curve::{GetPoint, GetSamples};
use crate::event::macros::{delegate_handlers, delegate_handlers_mut};
use crate::event::{
    canvas, curve, DelegateEventHandler, DelegateEventHandlerMut, Error, EventHandler,
    EventHandlerMut, HandlerResult,
};

impl EventHandlerMut<AddPoint> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: AddPoint) -> HandlerResult<AddPoint> {
        let default_weight = self.canvas.config.default_rational_bezier_weight;
        let AddPoint { point } = event;
        let weighted_point = WeightedPoint::new(point, default_weight);
        let result = self.delegate_mut(AddWeightedControlPoint::new(weighted_point));
        match result {
            Err(Error::Unimplemented) => self.delegate_mut(AddControlPoint::new(point)),
            _ => result,
        }
    }
}

impl EventHandlerMut<ChangeCurrentPointWeight> for CanvasEventHandlerMut<'_> {
    fn handle_mut(
        &mut self,
        event: ChangeCurrentPointWeight,
    ) -> HandlerResult<ChangeCurrentPointWeight> {
        let point_id = self.canvas.properties.current_point_index;
        let current_weight = self.as_immut().delegate(GetWeight::new(point_id))?;

        let change = event.weight;
        let weight = if change < 0.0 { current_weight / -change } else { current_weight * change };

        self.delegate_mut(ChangeWeight::new(point_id, weight))
    }
}

impl EventHandlerMut<DeleteCurrentPoint> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, _event: DeleteCurrentPoint) -> HandlerResult<DeleteCurrentPoint> {
        let point_id = self.canvas.properties.current_point_index;
        self.delegate_mut(DeletePoint::new(point_id))
    }
}

impl EventHandlerMut<MoveCurrentPoint> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: MoveCurrentPoint) -> HandlerResult<MoveCurrentPoint> {
        let point_id = self.canvas.properties.current_point_index;
        self.delegate_mut(MovePoint::new(point_id, event.shift))
    }
}

impl EventHandlerMut<ChangeCurrentPointIndex> for CanvasEventHandlerMut<'_> {
    fn handle_mut(
        &mut self,
        event: ChangeCurrentPointIndex,
    ) -> HandlerResult<ChangeCurrentPointIndex> {
        let point_id = self.canvas.properties.current_point_index;
        let length = self.as_immut().delegate(GetControlPointsLength)?;
        let new_id = math::rem_euclid(point_id as isize + event.change as isize, length as isize);
        self.canvas.properties.current_point_index = new_id;
        Ok(())
    }
}

impl EventHandlerMut<SetCurveType> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: SetCurveType) -> HandlerResult<SetCurveType> {
        let curve = &mut self.canvas.curves[self.canvas.properties.current_curve];
        replace_with::replace_with_or_abort(curve, |curve| {
            let samples = curve.event_handler().handle(GetSamples).ok();
            let points = match curve {
                Curve::Polyline(curve) => Some(curve.points.into_inner()),
                Curve::Interpolation(curve) => Some(curve.points.into_inner()),
                Curve::Bezier(curve) => Some(curve.points.into_inner()),
                Curve::RationalBezier(curve) => Some(
                    curve
                        .points
                        .into_inner()
                        .into_iter()
                        .map(WeightedPoint::point)
                        .collect::<Vec<_>>(),
                ),
                Curve::Trochoid(_) => None,
            };
            Canvas::create_curve(&self.canvas.config, event.0, points, samples)
        });
        Ok(())
    }
}

impl EventHandler<GetCurveType> for CanvasEventHandler<'_> {
    fn handle(&self, _event: GetCurveType) -> HandlerResult<GetCurveType> {
        Ok(self.canvas.curve_type())
    }
}

impl EventHandler<GetCurrentPoint> for CanvasEventHandler<'_> {
    fn handle(&self, _event: GetCurrentPoint) -> HandlerResult<GetCurrentPoint> {
        let point = self.delegate(GetPoint(self.canvas.properties.current_point_index))?;
        Ok(point)
    }
}

impl EventHandlerMut<RotateCurveById> for CanvasEventHandlerMut<'_> {
    fn handle_mut(&mut self, event: RotateCurveById) -> HandlerResult<RotateCurveById> {
        let curve = self
            .canvas
            .curves
            .get_mut(event.curve)
            .ok_or_else(|| Error::NoSuchCurve(event.curve))?;
        curve.event_handler_mut().handle_mut(RotateCurve::new(event.angle))?;
        Ok(())
    }
}

delegate_handlers! {
    CanvasEventHandler<'_> {
        curve::GetSamples,

        curve::control_points::GetInterpolationNodes,

        canvas::GetCurveCenter,
        canvas::SelectPoint,
        canvas::GetConvexHull,
        curve::GetPoint,
    }
}

delegate_handlers_mut! {
    CanvasEventHandlerMut<'_> {
        curve::SetSamples,

        curve::control_points::SetInterpolationNodes,

        canvas::MoveCurve,
        canvas::RotateCurve,
        canvas::SetConvexHull,
    }
}
