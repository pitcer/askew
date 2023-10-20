use crate::canvas::curve::control_points::CurveControlPoints;
use crate::canvas::curve::samples::Samples;
use crate::canvas::v2::curve::interpolation::InterpolationCurve;
use crate::canvas::v2::request::declare::{
    AddControlPoint, AddWeightedControlPoint, ChangeWeight, DeletePoint, GetControlPointsLength,
    GetCurveCenter, GetInterpolationNodes, GetPoint, GetWeight, MoveCurve, MovePoint, RotateCurve,
    SelectPoint, SetInterpolationNodes, SetTrochoidProperties,
};
use crate::canvas::v2::request::declare::{GetSamples, SetSamples};
use crate::request::macros::delegate_requests;
use crate::request::{
    RequestHandler, RequestHandlerMut, RequestSubHandler, RequestSubHandlerMut, Response,
    ResponseMut,
};

delegate_requests! {
    InterpolationCurve {
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

        // TrochoidCurve requests
        { mut SetTrochoidProperties => ! },
    }
}

impl RequestHandler<GetInterpolationNodes> for InterpolationCurve {
    fn handle(&self, _event: GetInterpolationNodes) -> Response<GetInterpolationNodes> {
        Ok(self.properties.nodes)
    }
}

impl RequestHandlerMut<SetInterpolationNodes> for InterpolationCurve {
    fn handle_mut(&mut self, event: SetInterpolationNodes) -> ResponseMut<SetInterpolationNodes> {
        self.properties.nodes = event.nodes;
        Ok(())
    }
}

impl RequestSubHandler<CurveControlPoints> for InterpolationCurve {
    fn sub_handler(&self) -> &CurveControlPoints {
        &self.points
    }
}

impl RequestSubHandlerMut<CurveControlPoints> for InterpolationCurve {
    fn sub_handler_mut(&mut self) -> &mut CurveControlPoints {
        &mut self.points
    }
}

impl RequestSubHandler<Samples> for InterpolationCurve {
    fn sub_handler(&self) -> &Samples {
        &self.samples
    }
}

impl RequestSubHandlerMut<Samples> for InterpolationCurve {
    fn sub_handler_mut(&mut self) -> &mut Samples {
        &mut self.samples
    }
}
