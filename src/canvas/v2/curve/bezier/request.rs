use crate::canvas::curve::control_points::CurveControlPoints;
use crate::canvas::curve::samples::Samples;
use crate::canvas::v2::curve::bezier::BezierCurve;
use crate::canvas::v2::request::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetInterpolationNodes, GetPoint, GetWeight, MoveCurve, MovePoint, RotateCurve,
    SelectPoint, SetInterpolationNodes, SetTrochoidProperties,
};
use crate::canvas::v2::request::{GetSamples, SetSamples};
use crate::request::macros::delegate_requests;
use crate::request::{RequestSubHandler, RequestSubHandlerMut};

delegate_requests! {
    BezierCurve {
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
        { mut SetSamples => Samples },
        { GetSamples => Samples },

        // InterpolationCurve requests
        { mut SetInterpolationNodes => ! },
        { GetInterpolationNodes => ! },

        // TrochoidCurve requests
        { mut SetTrochoidProperties => ! },
    }
}

impl RequestSubHandler<CurveControlPoints> for BezierCurve {
    fn sub_handler(&self) -> &CurveControlPoints {
        &self.points
    }
}

impl RequestSubHandlerMut<CurveControlPoints> for BezierCurve {
    fn sub_handler_mut(&mut self) -> &mut CurveControlPoints {
        &mut self.points
    }
}

impl RequestSubHandler<Samples> for BezierCurve {
    fn sub_handler(&self) -> &Samples {
        &self.samples
    }
}

impl RequestSubHandlerMut<Samples> for BezierCurve {
    fn sub_handler_mut(&mut self) -> &mut Samples {
        &mut self.samples
    }
}
