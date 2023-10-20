use crate::canvas::v2::request::declare::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetInterpolationNodes, GetPoint, GetSamples, GetWeight, MoveCurve, MovePoint,
    RotateCurve, SelectPoint, SetInterpolationNodes, SetSamples, SetTrochoidProperties,
};
use crate::request::macros::delegate_requests;
use crate::request::sieve::RequestSieve;

pub mod declare;

pub struct ExcludeControlPoints;

pub type ExcludeControlPointsRequests<T> = RequestSieve<ExcludeControlPoints, T>;

delegate_requests! {
    <T> ExcludeControlPointsRequests<T> {
        // ControlPoints requests
        { mut AddControlPoint => ! },
        { mut MovePoint => ! },
        { mut DeletePoint => ! },
        { mut RotateCurve => ! },
        { mut MoveCurve => ! },
        { mut ChangeWeight => ! },
        { mut AddWeightedControlPoint => ! },
        { GetControlPointsLength => ! },
        { GetCurveCenter => ! },
        { SelectPoint => ! },
        { GetPoint => ! },
        { GetWeight => ! },

        // Samples requests
        { mut SetSamples => T },
        { GetSamples => T },

        // InterpolationCurve requests
        { mut SetInterpolationNodes => T },
        { GetInterpolationNodes => T },

        // TrochoidCurve requests
        { mut SetTrochoidProperties => T },
    }
}

pub struct ExcludeInterpolation;

pub type ExcludeInterpolationRequests<T> = RequestSieve<ExcludeInterpolation, T>;

delegate_requests! {
    <T> ExcludeInterpolationRequests<T> {
        // ControlPoints requests
        { mut AddControlPoint => T },
        { mut MovePoint => T },
        { mut DeletePoint => T },
        { mut RotateCurve => T },
        { mut MoveCurve => T },
        { mut ChangeWeight => T },
        { mut AddWeightedControlPoint => T },
        { GetControlPointsLength => T },
        { GetCurveCenter => T },
        { SelectPoint => T },
        { GetPoint => T },
        { GetWeight => T },

        // Samples requests
        { mut SetSamples => T },
        { GetSamples => T },

        // InterpolationCurve requests
        { mut SetInterpolationNodes => ! },
        { GetInterpolationNodes => ! },

        // TrochoidCurve requests
        { mut SetTrochoidProperties => T },
    }
}
