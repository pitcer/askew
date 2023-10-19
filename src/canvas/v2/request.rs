use std::fmt::Debug;

use crate::canvas::curve::control_points::WeightedPoint;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::v2::curve::interpolation::InterpolationNodes;
use crate::canvas::v2::curve::trochoid::TrochoidCurveProperties;
use crate::event::PointId;
use crate::request::macros::{declare_requests, delegate_requests};
use crate::request::sieve::RequestSieve;

declare_requests! {
    // ControlPoints requests
    { mut AddControlPoint { point: Point<f32> } -> () },
    { mut MovePoint { id: PointId, shift: Vector<f32> } -> () },
    { mut DeletePoint { id: PointId } -> () },
    { mut RotateCurve { angle: f32 } -> () },
    { mut MoveCurve { shift: Vector<f32> } -> () },
    { mut ChangeWeight { id: PointId, weight: f32 } -> () },
    { mut AddWeightedControlPoint { point: WeightedPoint<f32, f32> } -> () },
    { GetControlPointsLength () -> usize },
    { GetCurveCenter () -> Option<Point<f32>> },
    { SelectPoint { guess: Point<f32>, radius: f32 } -> Option<PointId> },
    { GetPoint (PointId) -> Point<f32> },
    { GetWeight { id: PointId } -> f32 },

    // Samples requests
    { mut SetSamples (u32) -> () },
    { GetSamples () -> u32 },

    // InterpolationCurve requests
    { GetInterpolationNodes () -> InterpolationNodes },
    { mut SetInterpolationNodes { nodes: InterpolationNodes } -> () },

    // TrochoidCurve requests
    { mut SetTrochoidProperties (TrochoidCurveProperties) -> () },
}

pub struct Identity;

pub type CanvasIdentityRequestSieve<T> = RequestSieve<Identity, T>;

delegate_requests! {
    <T> CanvasIdentityRequestSieve<T> {
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
        { mut SetInterpolationNodes => T },
        { GetInterpolationNodes => T },

        // TrochoidCurve requests
        { mut SetTrochoidProperties => T },
    }
}

pub struct ExcludeControlPoints;

pub type ExcludeControlPointsRequestSieve<T> = RequestSieve<ExcludeControlPoints, T>;

delegate_requests! {
    <T> ExcludeControlPointsRequestSieve<T> {
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
