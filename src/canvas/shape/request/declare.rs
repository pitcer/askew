use crate::canvas::control_points::point::WeightedPoint;
use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::shape::interpolation::InterpolationNodes;
use crate::canvas::shape::trochoid::TrochoidCurveProperties;
use crate::request::macros::declare_requests;
use crate::request::PointId;

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
