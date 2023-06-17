pub mod curve {
    use crate::canvas::curve::control_points::event_handler::CurveEventHandler;
    use crate::canvas::math::point::Point;
    use crate::canvas::math::vector::Vector;
    use crate::event::macros::declare_events;
    use crate::event::PointId;

    declare_events! {
        CurveEventHandler<'_> {
            ChangeWeight { id: PointId, weight: f32 } -> (),
            DeletePoint { id: PointId } -> (),
            MovePoint { id: PointId, shift: Vector<f32> } -> (),
            AddControlPoint { position: Point<f32> } -> (),
            AddWeightedControlPoint { position: Point<f32>, weight: f32 } -> (),
            GetWeight { id: PointId } -> f32,
            GetControlPointsLength () -> usize,
        }
    }
}

pub mod canvas {
    use winit::dpi::PhysicalPosition;

    use crate::canvas::event_handler::CanvasEventHandler;
    use crate::canvas::math::vector::Vector;
    use crate::event::macros::declare_events;

    declare_events! {
        CanvasEventHandler<'_> {
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
    use crate::event::canvas::{AddCurve, AddPoint};
    use crate::event::macros::declare_events;
    use crate::event::{Change, Direction};
    use crate::ui::frame::event_handler::FrameEventHandler;
    use crate::ui::frame::mode::Mode;

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
        use crate::event::macros::declare_events;
        use crate::ui::frame::event_handler::FrameEventHandler;

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
