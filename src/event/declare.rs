#![allow(clippy::wildcard_imports)]

use winit::dpi::PhysicalPosition;

use crate::event::macros::declare_handler;
use crate::{
    canvas::curve::control_points::event_handler::ControlPointsCurveEventHandler,
    canvas::curve::control_points::points::event_handler::ControlPointsEventHandler,
    canvas::curve::control_points::WeightedPoint,
    canvas::curve::event_handler::CurveEventHandler,
    canvas::curve::formula::event_handler::FormulaCurveEventHandler,
    canvas::curve::samples::event_handler::SamplesEventHandler,
    canvas::event_handler::CanvasEventHandler,
    canvas::math::point::Point,
    canvas::math::vector::Vector,
    config::CurveType,
    event::PointId,
    event::{Change, Direction},
    ui::frame::event_handler::CommandEventHandler,
};

pub mod input {
    use super::*;

    declare_handler! {
        CommandEventHandler<'_> {
            'events_mut: {
                ToggleConvexHull () -> (),
                ChangeWeight (Change) -> (),
                MouseClick (PhysicalPosition<f64>) -> (),
                MousePress (PhysicalPosition<f64>) -> (),
                MovePoint (Direction) -> (),
                Delete () -> (),
                Add () -> (),
                ChangeIndex (Change) -> (),
            }
        }
    }
}

pub mod canvas {
    use super::*;

    declare_handler! {
        CanvasEventHandler<'_> {
            'inherited: {
                curve::SetSamples,
                curve::GetSamples,
                curve::control_points::SetInterpolationNodes,
                curve::control_points::GetInterpolationNodes,
                curve::GetPoint,
            }

            'events_mut: {
                ChangeCurrentPointWeight { weight: f32 } -> (),
                DeleteCurrentPoint () -> (),
                MoveCurrentPoint { shift: Vector<f32> } -> (),
                AddPoint { point: Point<f32> } -> (),
                ChangeCurrentPointIndex { change: i32 } -> (),
                AddCurve () -> (),
                DeleteCurve () -> (),
                ChangeCurrentCurveIndex { change: i32 } -> (),

                SetConvexHull (bool) -> (),

                SetCurveType (CurveType) -> (),

                RotateCurveById { angle: f32, curve: usize } -> (),
                RotateCurve { angle: f32 } -> (),
                MoveCurve { shift: Vector<f32> } -> (),

                MovePointOnCurve (usize, PointId, Point<f32>) -> (),
            }

            'events: {
                GetConvexHull () -> bool,

                GetCurveType () -> CurveType,

                GetCurveCenter () -> Option<Point<f32>>,

                GetCurrentPoint () -> Point<f32>,
                SelectPoint { guess: Point<f32>, radius: f32 } -> Option<PointId>,

                GetCurvesLength () -> usize,
                GetLength (usize) -> usize,
                GetPointOnCurve (usize, PointId) -> Point<f32>,
            }
        }
    }
}

pub mod curve {
    use super::*;

    declare_handler! {
        CurveEventHandler<'_> {
            'inherited: {
                control_points::DeletePoint,
                control_points::MovePoint,
                control_points::AddControlPoint,
                control_points::GetControlPointsLength,
                control_points::weighted::AddWeightedControlPoint,
                control_points::weighted::ChangeWeight,
                control_points::weighted::GetWeight,
                control_points::SetInterpolationNodes,
                control_points::GetInterpolationNodes,
                SetSamples,
                GetSamples,
                formula::SetTrochoidProperties,

                canvas::RotateCurve,
                canvas::MoveCurve,
                canvas::GetCurveCenter,
                canvas::SelectPoint,
            }

            'events: {
                GetPoint (PointId) -> Point<f32>,
            }
        }
    }

    declare_handler! {
        SamplesEventHandler<'_> {
            'events_mut: {
                SetSamples (u32) -> (),
            }

            'events: {
                GetSamples () -> u32,
            }
        }
    }

    pub mod formula {
        use crate::canvas::v2::curve::trochoid::event_handler::TrochoidCurveEventHandler;
        use crate::canvas::v2::curve::trochoid::TrochoidCurveProperties;

        use super::*;

        declare_handler! {
            FormulaCurveEventHandler<'_> {
                'inherited: {
                    curve::SetSamples,
                    curve::GetSamples,
                    SetTrochoidProperties,
                }

                'unimplemented: {
                    curve::control_points::DeletePoint,
                    curve::control_points::MovePoint,
                    curve::control_points::AddControlPoint,
                    curve::control_points::GetControlPointsLength,
                    curve::control_points::weighted::AddWeightedControlPoint,
                    curve::control_points::weighted::ChangeWeight,
                    curve::control_points::weighted::GetWeight,
                }
            }
        }

        declare_handler! {
            TrochoidCurveEventHandler<'_> {
                'inherited: {
                    curve::SetSamples,
                    curve::GetSamples,
                }

                'events_mut: {
                    SetTrochoidProperties (TrochoidCurveProperties) -> (),
                }
            }
        }
    }

    pub mod control_points {
        use crate::canvas::v2::curve::bezier::event_handler::BezierCurveEventHandler;
        use crate::canvas::v2::curve::interpolation::event_handler::InterpolationCurveEventHandler;
        use crate::canvas::v2::curve::interpolation::InterpolationNodes;
        use crate::canvas::v2::curve::polyline::event_handler::PolylineCurveEventHandler;

        use super::*;

        declare_handler! {
            ControlPointsCurveEventHandler<'_> {
                'inherited: {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                    curve::SetSamples,
                    curve::GetSamples,
                    SetInterpolationNodes,
                    GetInterpolationNodes,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                    canvas::GetCurveCenter,
                    canvas::SelectPoint,
                    curve::GetPoint
                }
            }
        }

        declare_handler! {
            BezierCurveEventHandler<'_> {
                'inherited: {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,
                    curve::SetSamples,
                    curve::GetSamples,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                    canvas::GetCurveCenter,
                    canvas::SelectPoint,
                    curve::GetPoint
                }

                'unimplemented: {
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                }
            }
        }

        declare_handler! {
            InterpolationCurveEventHandler<'_> {
                'inherited: {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,
                    curve::SetSamples,
                    curve::GetSamples,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                    canvas::GetCurveCenter,
                    canvas::SelectPoint,
                    curve::GetPoint
                }

                'unimplemented: {
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                }

                'events_mut: {
                    SetInterpolationNodes { nodes: InterpolationNodes } -> (),
                }

                'events: {
                    GetInterpolationNodes () -> InterpolationNodes,
                }
            }
        }

        declare_handler! {
            PolylineCurveEventHandler<'_> {
                'inherited: {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                    canvas::GetCurveCenter,
                    canvas::SelectPoint,
                    curve::GetPoint
                }

                'unimplemented: {
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                    curve::SetSamples,
                    curve::GetSamples,
                }
            }
        }

        declare_handler! {
            ControlPointsEventHandler<'_> {
                'inherited: {
                    canvas::RotateCurve,
                    canvas::MoveCurve,
                    canvas::GetCurveCenter,
                    canvas::SelectPoint,
                    curve::GetPoint
                }

                'events_mut: {
                    AddControlPoint { point: Point<f32> } -> (),
                    MovePoint { id: PointId, shift: Vector<f32> } -> (),
                    DeletePoint { id: PointId } -> (),
                }

                'events: {
                    GetControlPointsLength () -> usize,
                }
            }
        }

        pub mod weighted {
            use crate::canvas::v2::curve::rational_bezier::event_handler::RationalBezierCurveEventHandler;

            use super::*;

            declare_handler! {
                RationalBezierCurveEventHandler<'_> {
                    'inherited: {
                        control_points::GetControlPointsLength,
                        control_points::MovePoint,
                        control_points::DeletePoint,
                        curve::SetSamples,
                        curve::GetSamples,

                        canvas::RotateCurve,
                        canvas::MoveCurve,
                        canvas::GetCurveCenter,
                        canvas::SelectPoint,
                        curve::GetPoint
                    }

                    'unimplemented: {
                        control_points::AddControlPoint
                    }

                    'events_mut: {
                        ChangeWeight { id: PointId, weight: f32 } -> (),
                        AddWeightedControlPoint { point: WeightedPoint<f32, f32> } -> (),
                    }

                    'events: {
                        GetWeight { id: PointId } -> f32,
                    }
                }
            }
        }
    }
}
