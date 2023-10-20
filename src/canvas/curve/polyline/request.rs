use crate::canvas::control_points::point::CurveControlPoints;
use crate::canvas::curve::polyline::PolylineCurve;
use crate::canvas::curve::request::declare::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetInterpolationNodes, GetPoint, GetWeight, MoveCurve, MovePoint, RotateCurve,
    SelectPoint, SetInterpolationNodes, SetTrochoidProperties,
};
use crate::canvas::curve::request::declare::{GetSamples, SetSamples};
use crate::request::macros::delegate_requests;
use crate::request::{RequestSubHandler, RequestSubHandlerMut};

delegate_requests! {
    PolylineCurve {
        // ControlPoints requests
        { mut AddControlPoint => CurveControlPoints },
        { mut MovePoint => CurveControlPoints },
        { mut DeletePoint => CurveControlPoints },
        { mut RotateCurve => CurveControlPoints },
        { mut MoveCurve => CurveControlPoints },
        { mut ChangeWeight => ! },
        { mut AddWeightedControlPoint => ! },
        { GetControlPointsLength => CurveControlPoints },
        { GetCurveCenter => CurveControlPoints },
        { SelectPoint => CurveControlPoints },
        { GetPoint => CurveControlPoints },
        { GetWeight => ! },

        // Samples requests
        { mut SetSamples => ! },
        { GetSamples => ! },

        // InterpolationCurve requests
        { mut SetInterpolationNodes => ! },
        { GetInterpolationNodes => ! },

        // TrochoidCurve requests
        { mut SetTrochoidProperties => ! },
    }
}

impl RequestSubHandler<CurveControlPoints> for PolylineCurve {
    fn sub_handler(&self) -> &CurveControlPoints {
        &self.points
    }
}

impl RequestSubHandlerMut<CurveControlPoints> for PolylineCurve {
    fn sub_handler_mut(&mut self) -> &mut CurveControlPoints {
        &mut self.points
    }
}
