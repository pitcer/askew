use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::config::CurveType;
use crate::request::macros::declare_requests;
use crate::request::PointId;

declare_requests! {
    { GetConvexHull () -> bool },
    { GetCurveType () -> CurveType },
    // Delegated from curve
    // { GetCurveCenter () -> Option<Point<f32>> },
    { GetCurrentPoint () -> Point<f32> },
    // Delegated from curve
    // { SelectPoint { guess: Point<f32>, radius: f32 } -> Option<PointId> },
    { GetCurvesLength () -> usize },
    { GetLength (usize) -> usize },
    { GetPointOnCurve (usize, PointId) -> Point<f32> },

    { mut ChangeCurrentPointWeight { weight: f32 } -> () },
    { mut DeleteCurrentPoint () -> () },
    { mut MoveCurrentPoint { shift: Vector<f32> } -> () },
    { mut AddPoint { point: Point<f32> } -> () },
    { mut ChangeCurrentPointIndex { change: i32 } -> () },
    { mut AddCurve () -> () },
    { mut DeleteCurve () -> () },
    { mut ChangeCurrentCurveIndex { change: i32 } -> () },
    { mut SetConvexHull (bool) -> () },
    { mut SetCurveType (CurveType) -> () },
    { mut RotateCurveById { angle: f32, curve: usize } -> () },
    // Delegated from curve
    // { mut RotateCurve { angle: f32 } -> () },
    // Delegated from curve
    // { mut MoveCurve { shift: Vector<f32> } -> () },
    { mut MovePointOnCurve (usize, PointId, Point<f32>) -> () },
}
