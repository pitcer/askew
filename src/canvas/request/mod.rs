use crate::canvas::control_points::point::WeightedPoint;
use crate::canvas::curve::request::declare::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetInterpolationNodes, GetPoint, GetSamples, GetWeight, MoveCurve, MovePoint,
    RotateCurve, SelectPoint, SetInterpolationNodes, SetSamples,
};
use crate::canvas::curve::Curve;
use crate::canvas::request::declare::{
    AddCurve, AddPoint, ChangeCurrentCurveIndex, ChangeCurrentPointIndex, ChangeCurrentPointWeight,
    DeleteCurrentPoint, DeleteCurve, GetConvexHull, GetCurrentPoint, GetCurveType, GetCurvesLength,
    GetLength, GetPointOnCurve, MoveCurrentPoint, MovePointOnCurve, RotateCurveById, SetConvexHull,
    SetCurveType,
};
use crate::canvas::{math, Canvas};
use crate::request::macros::delegate_requests;
use crate::request::{
    Error, RequestHandler, RequestHandlerMut, RequestSubHandler, RequestSubHandlerMut, Response,
    ResponseMut,
};

pub mod declare;

delegate_requests! {
    Canvas {
        { GetSamples => Curve },
        { GetInterpolationNodes => Curve },
        { GetCurveCenter => Curve },
        { SelectPoint => Curve },
        { GetPoint => Curve },
        { mut SetSamples => Curve },
        { mut SetInterpolationNodes => Curve },
        { mut MoveCurve => Curve },
        { mut RotateCurve => Curve },

        // TODO: reimplement in curve internals
        { GetConvexHull => ! },
        { mut SetConvexHull => ! },
    }
}

impl RequestSubHandler<Curve> for Canvas {
    fn sub_handler(&self) -> &Curve {
        self.current_curve()
    }
}

impl RequestSubHandlerMut<Curve> for Canvas {
    fn sub_handler_mut(&mut self) -> &mut Curve {
        self.current_curve_mut()
    }
}

impl RequestHandlerMut<AddCurve> for Canvas {
    fn handle_mut(&mut self, _event: AddCurve) -> ResponseMut<AddCurve> {
        let curve_type = self.config.default_curve_type;
        let curve = Canvas::create_curve(&self.config, curve_type, None, None);
        self.curves.push(curve);
        self.properties.current_curve += 1;
        Ok(())
    }
}

impl RequestHandlerMut<DeleteCurve> for Canvas {
    fn handle_mut(&mut self, _event: DeleteCurve) -> ResponseMut<DeleteCurve> {
        let current_curve = self.properties.current_curve;
        self.curves.remove(current_curve);
        Ok(())
    }
}

impl RequestHandlerMut<ChangeCurrentCurveIndex> for Canvas {
    fn handle_mut(
        &mut self,
        event: ChangeCurrentCurveIndex,
    ) -> ResponseMut<ChangeCurrentCurveIndex> {
        self.properties.current_curve = math::rem_euclid(
            self.properties.current_curve as isize + event.change as isize,
            self.curves.len() as isize,
        );
        Ok(())
    }
}

impl RequestHandler<GetLength> for Canvas {
    fn handle(&self, event: GetLength) -> Response<GetLength> {
        let length = self.curves[event.0].handle(GetControlPointsLength)?;
        Ok(length)
    }
}

impl RequestHandler<GetCurvesLength> for Canvas {
    fn handle(&self, _event: GetCurvesLength) -> Response<GetCurvesLength> {
        let length = self.curves.len();
        Ok(length)
    }
}

impl RequestHandler<GetPointOnCurve> for Canvas {
    fn handle(&self, event: GetPointOnCurve) -> Response<GetPointOnCurve> {
        let point = self.curves[event.0].handle(GetPoint(event.1))?;
        Ok(point)
    }
}

impl RequestHandlerMut<MovePointOnCurve> for Canvas {
    fn handle_mut(&mut self, event: MovePointOnCurve) -> ResponseMut<MovePointOnCurve> {
        let point = self.curves[event.0].handle(GetPoint(event.1))?;
        let shift = event.2 - point;
        self.curves[event.0].handle_mut(MovePoint::new(event.1, shift))?;
        Ok(())
    }
}

impl RequestHandlerMut<AddPoint> for Canvas {
    fn handle_mut(&mut self, event: AddPoint) -> ResponseMut<AddPoint> {
        let default_weight = self.config.default_rational_bezier_weight;
        let AddPoint { point } = event;
        let weighted_point = WeightedPoint::new(point, default_weight);
        let result =
            self.sub_handler_mut().handle_mut(AddWeightedControlPoint::new(weighted_point));
        match result {
            Err(Error::Unimplemented { .. }) => {
                self.sub_handler_mut().handle_mut(AddControlPoint::new(point))
            }
            _ => result,
        }
    }
}

impl RequestHandlerMut<ChangeCurrentPointWeight> for Canvas {
    fn handle_mut(
        &mut self,
        event: ChangeCurrentPointWeight,
    ) -> ResponseMut<ChangeCurrentPointWeight> {
        let point_id = self.properties.current_point_index;
        let current_weight = self.sub_handler().handle(GetWeight::new(point_id))?;

        let change = event.weight;
        let weight = if change < 0.0 { current_weight / -change } else { current_weight * change };

        self.sub_handler_mut().handle_mut(ChangeWeight::new(point_id, weight))
    }
}

impl RequestHandlerMut<DeleteCurrentPoint> for Canvas {
    fn handle_mut(&mut self, _event: DeleteCurrentPoint) -> ResponseMut<DeleteCurrentPoint> {
        let point_id = self.properties.current_point_index;
        self.sub_handler_mut().handle_mut(DeletePoint::new(point_id))
    }
}

impl RequestHandlerMut<MoveCurrentPoint> for Canvas {
    fn handle_mut(&mut self, event: MoveCurrentPoint) -> ResponseMut<MoveCurrentPoint> {
        let point_id = self.properties.current_point_index;
        self.sub_handler_mut().handle_mut(MovePoint::new(point_id, event.shift))
    }
}

impl RequestHandlerMut<ChangeCurrentPointIndex> for Canvas {
    fn handle_mut(
        &mut self,
        event: ChangeCurrentPointIndex,
    ) -> ResponseMut<ChangeCurrentPointIndex> {
        let point_id = self.properties.current_point_index;
        let length = self.sub_handler().handle(GetControlPointsLength)?;
        let new_id = math::rem_euclid(point_id as isize + event.change as isize, length as isize);
        self.properties.current_point_index = new_id;
        Ok(())
    }
}

impl RequestHandlerMut<SetCurveType> for Canvas {
    fn handle_mut(&mut self, event: SetCurveType) -> ResponseMut<SetCurveType> {
        let curve = &mut self.curves[self.properties.current_curve];
        replace_with::replace_with_or_abort(curve, |curve| {
            let samples = curve.handle(GetSamples).ok();
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
            Canvas::create_curve(&self.config, event.0, points, samples)
        });
        Ok(())
    }
}

impl RequestHandler<GetCurveType> for Canvas {
    fn handle(&self, _event: GetCurveType) -> Response<GetCurveType> {
        Ok(self.curve_type())
    }
}

impl RequestHandler<GetCurrentPoint> for Canvas {
    fn handle(&self, _event: GetCurrentPoint) -> Response<GetCurrentPoint> {
        let point = self.sub_handler().handle(GetPoint(self.properties.current_point_index))?;
        Ok(point)
    }
}

impl RequestHandlerMut<RotateCurveById> for Canvas {
    fn handle_mut(&mut self, event: RotateCurveById) -> ResponseMut<RotateCurveById> {
        let curve =
            self.curves.get_mut(event.curve).ok_or_else(|| Error::NoSuchCurve(event.curve))?;
        curve.handle_mut(RotateCurve::new(event.angle))?;
        Ok(())
    }
}
