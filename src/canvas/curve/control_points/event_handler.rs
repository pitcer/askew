use crate::canvas::curve::control_points::rational_bezier::RationalBezierPoint;
use crate::canvas::curve::control_points::{ControlPointsCurve, GetControlPoints};
use crate::event::curve::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetWeight, MovePoint,
};
use crate::event::handler::{AddPointHandler, DeletePointHandler, MovePointHandler};
use crate::event::{Error, EventHandler, HandlerResult};

pub struct CurveEventHandler<'a> {
    curve: &'a mut ControlPointsCurve,
}

impl<'a> CurveEventHandler<'a> {
    pub fn new(curve: &'a mut ControlPointsCurve) -> Self {
        Self { curve }
    }
}

impl<'a> EventHandler<ChangeWeight> for CurveEventHandler<'a> {
    fn handle(&mut self, event: ChangeWeight) -> HandlerResult<ChangeWeight> {
        match self.curve {
            ControlPointsCurve::Polyline(_)
            | ControlPointsCurve::ConvexHull(_)
            | ControlPointsCurve::Interpolation(_)
            | ControlPointsCurve::Bezier(_) => Err(Error::Unimplemented),
            ControlPointsCurve::RationalBezier(curve) => {
                let ChangeWeight { id, weight } = event;
                curve.change_point_weight(id, weight);
                Ok(())
            }
        }
    }
}

impl<'a> EventHandler<DeletePoint> for CurveEventHandler<'a> {
    fn handle(&mut self, event: DeletePoint) -> HandlerResult<DeletePoint> {
        let id = event.id;
        match self.curve {
            ControlPointsCurve::Polyline(curve) => {
                curve.handle_delete_point(id).map_err(Error::Other)
            }
            ControlPointsCurve::ConvexHull(curve) => {
                curve.handle_delete_point(id).map_err(Error::Other)
            }
            ControlPointsCurve::Interpolation(curve) => {
                curve.handle_delete_point(id).map_err(Error::Other)
            }
            ControlPointsCurve::Bezier(curve) => {
                curve.handle_delete_point(id).map_err(Error::Other)
            }
            ControlPointsCurve::RationalBezier(curve) => {
                curve.handle_delete_point(id).map_err(Error::Other)
            }
        }
    }
}

impl<'a> EventHandler<MovePoint> for CurveEventHandler<'a> {
    fn handle(&mut self, event: MovePoint) -> HandlerResult<MovePoint> {
        let MovePoint { id, shift } = event;
        match self.curve {
            ControlPointsCurve::Polyline(curve) => {
                curve.handle_move_point(id, shift).map_err(Error::Other)
            }
            ControlPointsCurve::ConvexHull(curve) => {
                curve.handle_move_point(id, shift).map_err(Error::Other)
            }
            ControlPointsCurve::Interpolation(curve) => {
                curve.handle_move_point(id, shift).map_err(Error::Other)
            }
            ControlPointsCurve::Bezier(curve) => {
                curve.handle_move_point(id, shift).map_err(Error::Other)
            }
            ControlPointsCurve::RationalBezier(curve) => {
                curve.handle_move_point(id, shift).map_err(Error::Other)
            }
        }
    }
}

impl<'a> EventHandler<AddControlPoint> for CurveEventHandler<'a> {
    fn handle(&mut self, event: AddControlPoint) -> HandlerResult<AddControlPoint> {
        let point = event.position;
        match self.curve {
            ControlPointsCurve::Polyline(curve) => {
                curve.handle_add_point(point).map_err(Error::Other)
            }
            ControlPointsCurve::ConvexHull(curve) => {
                curve.handle_add_point(point).map_err(Error::Other)
            }
            ControlPointsCurve::Interpolation(curve) => {
                curve.handle_add_point(point).map_err(Error::Other)
            }
            ControlPointsCurve::Bezier(curve) => {
                curve.handle_add_point(point).map_err(Error::Other)
            }
            ControlPointsCurve::RationalBezier(_) => Err(Error::Unimplemented),
        }
    }
}

impl<'a> EventHandler<AddWeightedControlPoint> for CurveEventHandler<'a> {
    fn handle(&mut self, event: AddWeightedControlPoint) -> HandlerResult<AddWeightedControlPoint> {
        match self.curve {
            ControlPointsCurve::Polyline(_)
            | ControlPointsCurve::ConvexHull(_)
            | ControlPointsCurve::Interpolation(_)
            | ControlPointsCurve::Bezier(_) => Err(Error::Unimplemented),
            ControlPointsCurve::RationalBezier(curve) => {
                let AddWeightedControlPoint { position, weight } = event;
                let point = RationalBezierPoint::new(position, weight);
                curve.handle_add_point(point).map_err(Error::Other)
            }
        }
    }
}

impl<'a> EventHandler<GetWeight> for CurveEventHandler<'a> {
    fn handle(&mut self, event: GetWeight) -> HandlerResult<GetWeight> {
        match self.curve {
            ControlPointsCurve::Polyline(_)
            | ControlPointsCurve::ConvexHull(_)
            | ControlPointsCurve::Interpolation(_)
            | ControlPointsCurve::Bezier(_) => Err(Error::Unimplemented),
            ControlPointsCurve::RationalBezier(curve) => curve
                .control_points()
                .get(event.id)
                .ok_or_else(|| Error::NoSuchPoint(event.id))
                .map(|point| point.weight),
        }
    }
}

impl<'a> EventHandler<GetControlPointsLength> for CurveEventHandler<'a> {
    fn handle(&mut self, _event: GetControlPointsLength) -> HandlerResult<GetControlPointsLength> {
        match self.curve {
            ControlPointsCurve::Polyline(curve) => Ok(curve.control_points().length()),
            ControlPointsCurve::ConvexHull(curve) => Ok(curve.control_points().length()),
            ControlPointsCurve::Interpolation(curve) => Ok(curve.control_points().length()),
            ControlPointsCurve::Bezier(curve) => Ok(curve.control_points().length()),
            ControlPointsCurve::RationalBezier(curve) => Ok(curve.control_points().length()),
        }
    }
}
