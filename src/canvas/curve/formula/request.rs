use crate::canvas::v2::request::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetInterpolationNodes, GetPoint, GetWeight, MoveCurve, MovePoint, RotateCurve,
    SelectPoint, SetInterpolationNodes, SetTrochoidProperties,
};
use crate::canvas::v2::request::{GetSamples, SetSamples};
use crate::request::macros::delegate_requests;
use crate::request::sieve::RequestSieve;

#[deprecated(note = "remove after testing sieve")]
pub type FormulaCurveRequestSieve<T> = RequestSieve<FormulaCurve, T>;

#[deprecated(note = "remove after testing sieve")]
pub struct FormulaCurve;

delegate_requests! {
    <T> FormulaCurveRequestSieve<&mut T> {
        // ControlPoints requests
        { mut AddControlPoint => ! },
        { mut MovePoint => ! },
        { mut DeletePoint => ! },
        { mut RotateCurve => ! },
        { mut MoveCurve => ! },
        { mut ChangeWeight => ! },
        { mut AddWeightedControlPoint => ! },

        // Samples requests
        { mut SetSamples => T },

        // InterpolationCurve requests
        { mut SetInterpolationNodes => ! },

        // TrochoidCurve requests
        { mut SetTrochoidProperties => T },
    }
}

delegate_requests! {
    <T> FormulaCurveRequestSieve<&T> {
        // ControlPoints requests
        { GetControlPointsLength => ! },
        { GetCurveCenter => ! },
        { SelectPoint => ! },
        { GetPoint => ! },
        { GetWeight => ! },

        // Samples requests
        { GetSamples => T },

        // InterpolationCurve requests
        { GetInterpolationNodes => ! },
    }
}
