use winit::dpi::PhysicalPosition;

use crate::canvas::curve::control_points::WeightedPoint;
use crate::canvas::curve::CurveKind;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math;
use crate::canvas::math::point::Point;
use crate::event::canvas::{
    AddPoint, ChangeCurrentPointIndex, ChangeCurrentPointWeight, DeleteCurrentPoint,
    MoveCurrentPoint,
};
use crate::event::curve::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetSamples, GetWeight, MovePoint, SetSamples,
};
use crate::event::{Error, EventHandler, HandlerResult};

impl EventHandler<AddPoint> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: AddPoint) -> HandlerResult<AddPoint> {
        fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
            Point::new(position.x as f32, position.y as f32)
        }

        let default_weight = self.canvas.properties.default_weight;
        let current_curve = self.canvas.current_curve_mut();
        let CurveKind::ControlPoints(curve) = current_curve else {
            return Err(Error::Unimplemented);
        };
        let mut handler = curve.event_handler();
        let position = scale_position(event.position);
        let point = WeightedPoint::new(position, default_weight);
        let result = handler.handle(AddWeightedControlPoint::new(point));
        match result {
            Err(Error::Unimplemented) => handler.handle(AddControlPoint::new(position)),
            _ => result,
        }
    }
}

impl EventHandler<ChangeCurrentPointWeight> for CanvasEventHandler<'_> {
    fn handle(
        &mut self,
        event: ChangeCurrentPointWeight,
    ) -> HandlerResult<ChangeCurrentPointWeight> {
        let point_id = self.canvas.properties.current_point_index;
        let curve = self.canvas.current_curve_mut();
        let CurveKind::ControlPoints(curve) = curve else {
            return Err(Error::Unimplemented);
        };
        let mut handler = curve.event_handler();
        let current_weight = handler.handle(GetWeight::new(point_id))?;

        let change = event.weight;
        let weight = if change < 0.0 {
            current_weight / -change
        } else {
            current_weight * change
        };

        handler.handle(ChangeWeight::new(point_id, weight))
    }
}

impl EventHandler<DeleteCurrentPoint> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: DeleteCurrentPoint) -> HandlerResult<DeleteCurrentPoint> {
        let point_id = self.canvas.properties.current_point_index;
        let curve = self.canvas.current_curve_mut();
        let CurveKind::ControlPoints(curve) = curve else {
            return Err(Error::Unimplemented);
        };
        let mut handler = curve.event_handler();
        handler.handle(DeletePoint::new(point_id))
    }
}

impl EventHandler<MoveCurrentPoint> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: MoveCurrentPoint) -> HandlerResult<MoveCurrentPoint> {
        let point_id = self.canvas.properties.current_point_index;
        let curve = self.canvas.current_curve_mut();
        let CurveKind::ControlPoints(curve) = curve else {
            return Err(Error::Unimplemented);
        };
        let mut handler = curve.event_handler();
        handler.handle(MovePoint::new(point_id, event.shift))
    }
}

impl EventHandler<ChangeCurrentPointIndex> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: ChangeCurrentPointIndex) -> HandlerResult<ChangeCurrentPointIndex> {
        let point_id = self.canvas.properties.current_point_index;
        let curve = self.canvas.current_curve_mut();
        let CurveKind::ControlPoints(curve) = curve else {
            return Err(Error::Unimplemented);
        };
        let length = curve.event_handler().handle(GetControlPointsLength)?;
        let new_id = math::rem_euclid((point_id + event.change as usize) as isize, length as isize);
        self.canvas.properties.current_point_index = new_id;
        Ok(())
    }
}

impl EventHandler<SetSamples> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: SetSamples) -> HandlerResult<SetSamples> {
        let curve = self.canvas.current_curve_mut();
        let CurveKind::ControlPoints(curve) = curve else {
            return Err(Error::Unimplemented);
        };
        curve.event_handler().handle(event)
    }
}

impl EventHandler<GetSamples> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: GetSamples) -> HandlerResult<GetSamples> {
        let curve = self.canvas.current_curve_mut();
        let CurveKind::ControlPoints(curve) = curve else {
            return Err(Error::Unimplemented);
        };
        curve.event_handler().handle(event)
    }
}
