use crate::canvas::curve::control_points::{ControlPointsCurveKind, WeightedPoint};
use crate::canvas::curve::formula::FormulaCurveKind;
use crate::canvas::curve::CurveKind;
use crate::canvas::event_handler::CanvasEventHandler;
use crate::canvas::math;
use crate::config::CurveType;
use crate::event::canvas::{
    AddPoint, ChangeCurrentPointIndex, ChangeCurrentPointWeight, DeleteCurrentPoint, GetCurveType,
    MoveCurrentPoint, SetCurveType,
};
use crate::event::curve::control_points::weighted::{
    AddWeightedControlPoint, ChangeWeight, GetWeight,
};
use crate::event::curve::control_points::{
    AddControlPoint, DeletePoint, GetControlPointsLength, MovePoint,
};
use crate::event::macros::delegate_handlers;
use crate::event::{canvas, curve, DelegateEventHandler, Error, EventHandler, HandlerResult};

impl EventHandler<AddPoint> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: AddPoint) -> HandlerResult<AddPoint> {
        let default_weight = self.canvas.properties.default_weight;
        let AddPoint { point } = event;
        let weighted_point = WeightedPoint::new(point, default_weight);
        let result = self.delegate(AddWeightedControlPoint::new(weighted_point));
        match result {
            Err(Error::Unimplemented) => self.delegate(AddControlPoint::new(point)),
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
        let new_id = math::rem_euclid(point_id as isize + event.change as isize, length as isize);
        self.canvas.properties.current_point_index = new_id;
        Ok(())
    }
}

impl EventHandler<SetCurveType> for CanvasEventHandler<'_> {
    fn handle(&mut self, event: SetCurveType) -> HandlerResult<SetCurveType> {
        todo!()
        // let curve = mem::take(&mut self.canvas.curves[self.canvas.properties.current_curve]);
        // match curve {
        //     CurveKind::ControlPoints(curve) =>
        //         let points = match curve {
        //         ControlPointsCurveKind::Polyline(_) => CurveType::Polyline,
        //         ControlPointsCurveKind::ConvexHull(_) => CurveType::ConvexHull,
        //         ControlPointsCurveKind::Interpolation(_) => CurveType::Interpolation,
        //         ControlPointsCurveKind::Bezier(_) => CurveType::Bezier,
        //         ControlPointsCurveKind::RationalBezier(_) => CurveType::RationalBezier,
        //     },
        //     CurveKind::Formula(curve) => match curve {
        //         FormulaCurveKind::Trochoid(_) => CurveType::Trochoid,
        //     },
        // }
        // Ok(())
    }
}

impl EventHandler<GetCurveType> for CanvasEventHandler<'_> {
    fn handle(&mut self, _event: GetCurveType) -> HandlerResult<GetCurveType> {
        Ok(match self.canvas.current_curve_mut() {
            CurveKind::ControlPoints(curve) => match curve {
                ControlPointsCurveKind::Polyline(_) => CurveType::Polyline,
                ControlPointsCurveKind::ConvexHull(_) => CurveType::ConvexHull,
                ControlPointsCurveKind::Interpolation(_) => CurveType::Interpolation,
                ControlPointsCurveKind::Bezier(_) => CurveType::Bezier,
                ControlPointsCurveKind::RationalBezier(_) => CurveType::RationalBezier,
            },
            CurveKind::Formula(curve) => match curve {
                FormulaCurveKind::Trochoid(_) => CurveType::Trochoid,
            },
        })
    }
}

delegate_handlers! {
    CanvasEventHandler<'_> {
        curve::SetSamples,
        curve::GetSamples,

        curve::control_points::SetInterpolationNodes,
        curve::control_points::GetInterpolationNodes,

        canvas::MoveCurve,
        canvas::RotateCurve,
    }
}
