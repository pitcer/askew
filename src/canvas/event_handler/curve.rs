use winit::dpi::PhysicalPosition;

use crate::canvas::curve::control_points::WeightedPoint;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math;
use crate::canvas::math::point::Point;
use crate::event::canvas::{
    AddPoint, ChangeCurrentPointIndex, ChangeCurrentPointWeight, DeleteCurrentPoint,
    MoveCurrentPoint,
};
use crate::event::curve::control_points::weighted::{
    AddWeightedControlPoint, ChangeWeight, GetWeight,
};
use crate::event::curve::control_points::{
    AddControlPoint, DeletePoint, GetControlPointsLength, MovePoint,
};
use crate::event::macros::delegate_handlers;
use crate::event::{curve, DelegateEventHandler, Error, EventHandler, HandlerResult};

impl EventHandler<AddPoint> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: AddPoint) -> HandlerResult<AddPoint> {
        fn scale_position(position: PhysicalPosition<f64>) -> Point<f32> {
            Point::new(position.x as f32, position.y as f32)
        }

        let default_weight = self.canvas.properties.default_weight;
        let position = scale_position(event.position);
        let point = WeightedPoint::new(position, default_weight);
        let result = self.delegate(AddWeightedControlPoint::new(point));
        match result {
            Err(Error::Unimplemented) => self.delegate(AddControlPoint::new(position)),
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
        let current_weight = self.delegate(GetWeight::new(point_id))?;

        let change = event.weight;
        let weight = if change < 0.0 {
            current_weight / -change
        } else {
            current_weight * change
        };

        self.delegate(ChangeWeight::new(point_id, weight))
    }
}

impl EventHandler<DeleteCurrentPoint> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: DeleteCurrentPoint) -> HandlerResult<DeleteCurrentPoint> {
        let point_id = self.canvas.properties.current_point_index;
        self.delegate(DeletePoint::new(point_id))
    }
}

impl EventHandler<MoveCurrentPoint> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: MoveCurrentPoint) -> HandlerResult<MoveCurrentPoint> {
        let point_id = self.canvas.properties.current_point_index;
        self.delegate(MovePoint::new(point_id, event.shift))
    }
}

impl EventHandler<ChangeCurrentPointIndex> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: ChangeCurrentPointIndex) -> HandlerResult<ChangeCurrentPointIndex> {
        let point_id = self.canvas.properties.current_point_index;
        let length = self.delegate(GetControlPointsLength)?;
        let new_id = math::rem_euclid((point_id + event.change as usize) as isize, length as isize);
        self.canvas.properties.current_point_index = new_id;
        Ok(())
    }
}

delegate_handlers! {
    CanvasEventHandler<'_> {
        curve::SetSamples,
        curve::GetSamples,
    }
}
