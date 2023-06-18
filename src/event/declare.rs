use winit::dpi::PhysicalPosition;

use crate::{
    canvas::curve::control_points::event_handler::ControlPointsCurveEventHandler,
    canvas::curve::control_points::kind::bezier::event_handler::BezierEventHandler,
    canvas::curve::control_points::kind::convex_hull::event_handler::ConvexHullEventHandler,
    canvas::curve::control_points::kind::interpolation::event_handler::InterpolationEventHandler,
    canvas::curve::control_points::kind::polyline::event_handler::PolylineEventHandler,
    canvas::curve::control_points::kind::rational_bezier::event_handler::RationalBezierEventHandler,
    canvas::curve::control_points::points::event_handler::ControlPointsEventHandler,
    canvas::curve::control_points::WeightedPoint,
    canvas::event_handler::CanvasEventHandler,
    canvas::math::point::Point,
    canvas::math::vector::Vector,
    event::canvas::{AddCurve, AddPoint},
    event::curve::{GetSamples, SetSamples},
    event::macros::declare_events,
    event::PointId,
    event::{Change, Direction},
    ui::frame::event_handler::FrameEventHandler,
    ui::frame::mode::Mode,
};

pub mod curve {
    use super::{
        declare_events, BezierEventHandler, ControlPointsCurveEventHandler,
        ControlPointsEventHandler, ConvexHullEventHandler, InterpolationEventHandler, Point,
        PointId, PolylineEventHandler, RationalBezierEventHandler, Vector, WeightedPoint,
    };

    declare_events! {
        ControlPointsEventHandler<'_> {
            GetControlPointsLength () -> usize,
            AddControlPoint { point: Point<f32> } -> (),
            MovePoint { id: PointId, shift: Vector<f32> } -> (),
            DeletePoint { id: PointId } -> (),
        }

        RationalBezierEventHandler<'_>: GetControlPointsLength, MovePoint, DeletePoint, SetSamples,
            GetSamples
        {
            ChangeWeight { id: PointId, weight: f32 } -> (),
            AddWeightedControlPoint { point: WeightedPoint<f32, f32> } -> (),
            GetWeight { id: PointId } -> f32,
        }

        BezierEventHandler<'_>: GetControlPointsLength, AddControlPoint, MovePoint, DeletePoint,
            AddWeightedControlPoint, ChangeWeight, GetWeight, SetSamples, GetSamples {}

        ConvexHullEventHandler<'_>: GetControlPointsLength, AddControlPoint, MovePoint,
            DeletePoint, AddWeightedControlPoint, ChangeWeight, GetWeight, SetSamples, GetSamples {}

        InterpolationEventHandler<'_>: GetControlPointsLength, AddControlPoint, MovePoint,
            DeletePoint, AddWeightedControlPoint, ChangeWeight, GetWeight, SetSamples, GetSamples {}

        PolylineEventHandler<'_>: GetControlPointsLength, AddControlPoint, MovePoint, DeletePoint,
            AddWeightedControlPoint, ChangeWeight, GetWeight, SetSamples, GetSamples {}

        ControlPointsCurveEventHandler<'_>: DeletePoint, MovePoint, AddControlPoint,
            GetControlPointsLength, AddWeightedControlPoint, ChangeWeight, GetWeight,
            SetSamples, GetSamples
        {
            SetSamples (u32) -> (),
            GetSamples () -> u32,
        }
    }
}

pub mod canvas {
    use super::{
        declare_events, CanvasEventHandler, GetSamples, PhysicalPosition, SetSamples, Vector,
    };

    declare_events! {
        CanvasEventHandler<'_>: SetSamples, GetSamples {
            ChangeCurrentPointWeight { weight: f32 } -> (),
            DeleteCurrentPoint () -> (),
            MoveCurrentPoint { shift: Vector<f32> } -> (),
            AddPoint { position: PhysicalPosition<f64> } -> (),
            ChangeCurrentPointIndex { change: i32 } -> (),
            AddCurve () -> (),
            DeleteCurve () -> (),
            ChangeCurrentCurveIndex { change: i32 } -> (),

            SetConvexHull (bool) -> (),
            GetConvexHull () -> bool,
        }
    }
}

pub mod input {
    use super::{declare_events, AddCurve, AddPoint, Change, Direction, FrameEventHandler, Mode};

    declare_events! {
        FrameEventHandler<'_>: AddPoint, AddCurve {
            ToggleConvexHull () -> (),
            ChangeWeight (Change) -> (),
            MovePoint (Direction) -> (),
            Delete () -> (),
            ChangeMode (Mode) -> (),
            ChangeIndex (Change) -> (),
        }
    }

    pub mod command {
        use super::{declare_events, FrameEventHandler};

        declare_events! {
            FrameEventHandler<'_> {
                EnterCommand () -> (),
                ReceiveCharacter (char) -> (),
                ExecuteCommand () -> (),
                ExitMode () -> (),
            }
        }
    }
}
