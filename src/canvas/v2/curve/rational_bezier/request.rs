use crate::canvas::curve::samples::Samples;
use crate::canvas::v2::curve::rational_bezier::{RationalBezierCurve, WeightedControlPoints};
use crate::canvas::v2::request::declare::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetInterpolationNodes, GetPoint, GetWeight, MoveCurve, MovePoint, RotateCurve,
    SelectPoint, SetInterpolationNodes, SetTrochoidProperties,
};
use crate::canvas::v2::request::declare::{GetSamples, SetSamples};
use crate::request::macros::delegate_requests;
use crate::request::{RequestSubHandler, RequestSubHandlerMut};

delegate_requests! {
    RationalBezierCurve {
        // ControlPoints requests
        { mut AddControlPoint => ! },
        { mut MovePoint => WeightedControlPoints },
        { mut DeletePoint => WeightedControlPoints },
        { mut RotateCurve => WeightedControlPoints },
        { mut MoveCurve => WeightedControlPoints },
        { mut ChangeWeight => WeightedControlPoints },
        { mut AddWeightedControlPoint => WeightedControlPoints },
        { GetControlPointsLength => WeightedControlPoints },
        { GetCurveCenter => WeightedControlPoints },
        { SelectPoint => WeightedControlPoints },
        { GetPoint => WeightedControlPoints },
        { GetWeight => WeightedControlPoints },

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

impl RequestSubHandler<WeightedControlPoints> for RationalBezierCurve {
    fn sub_handler(&self) -> &WeightedControlPoints {
        &self.points
    }
}

impl RequestSubHandlerMut<WeightedControlPoints> for RationalBezierCurve {
    fn sub_handler_mut(&mut self) -> &mut WeightedControlPoints {
        &mut self.points
    }
}

impl RequestSubHandler<Samples> for RationalBezierCurve {
    fn sub_handler(&self) -> &Samples {
        &self.samples
    }
}

impl RequestSubHandlerMut<Samples> for RationalBezierCurve {
    fn sub_handler_mut(&mut self) -> &mut Samples {
        &mut self.samples
    }
}
