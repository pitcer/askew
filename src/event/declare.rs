#![allow(clippy::wildcard_imports)]

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
    canvas::curve::event_handler::CurveEventHandler,
    canvas::curve::formula::event_handler::FormulaCurveEventHandler,
    canvas::curve::formula::trochoid::event_handler::TrochoidEventHandler,
    canvas::curve::samples::event_handler::SamplesEventHandler,
    canvas::event_handler::CanvasEventHandler,
    canvas::math::point::Point,
    canvas::math::vector::Vector,
    config::CurveType,
    event::macros::declare_events,
    event::PointId,
    event::{Change, Direction},
    ui::frame::event_handler::CommandEventHandler,
};

pub mod input {
    use super::*;

    declare_events! {
        CommandEventHandler<'_> {
            ~ {
                canvas::AddCurve,
            }

            ToggleConvexHull () -> (),
            ChangeWeight (Change) -> (),
            MouseClick (PhysicalPosition<f64>) -> (),
            MovePoint (Direction) -> (),
            Delete () -> (),
            ChangeIndex (Change) -> (),
        }
    }
}

pub mod canvas {
    use super::*;

    declare_events! {
        CanvasEventHandler<'_> {
            ~ {
                curve::SetSamples,
                curve::GetSamples,
                curve::control_points::SetInterpolationNodes,
                curve::control_points::GetInterpolationNodes,
            }

            ChangeCurrentPointWeight { weight: f32 } -> (),
            DeleteCurrentPoint () -> (),
            MoveCurrentPoint { shift: Vector<f32> } -> (),
            AddPoint { point: Point<f32> } -> (),
            ChangeCurrentPointIndex { change: i32 } -> (),
            AddCurve () -> (),
            DeleteCurve () -> (),
            ChangeCurrentCurveIndex { change: i32 } -> (),

            SetConvexHull (bool) -> (),
            GetConvexHull () -> bool,

            SetCurveType (CurveType) -> (),
            GetCurveType () -> CurveType,

            RotateCurve { angle: f32 } -> (),
            MoveCurve { shift: Vector<f32> } -> (),
        }
    }
}

pub mod curve {
    use super::*;

    declare_events! {
        CurveEventHandler<'_> {
            ~ {
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

                canvas::RotateCurve,
                canvas::MoveCurve,
            }
        }

        SamplesEventHandler<'_> {
            SetSamples (u32) -> (),
            GetSamples () -> u32,
        }
    }

    pub mod formula {
        use super::*;

        declare_events! {
            FormulaCurveEventHandler<'_> {
                ~ {
                    curve::SetSamples,
                    curve::GetSamples
                }

                ! {
                    curve::control_points::DeletePoint,
                    curve::control_points::MovePoint,
                    curve::control_points::AddControlPoint,
                    curve::control_points::GetControlPointsLength,
                    curve::control_points::weighted::AddWeightedControlPoint,
                    curve::control_points::weighted::ChangeWeight,
                    curve::control_points::weighted::GetWeight,
                }
            }

            TrochoidEventHandler<'_> {
                ~ {
                    curve::SetSamples,
                    curve::GetSamples,
                }
            }
        }
    }

    pub mod control_points {
        use crate::canvas::curve::control_points::kind::interpolation::InterpolationNodes;

        use super::*;

        declare_events! {
            ControlPointsCurveEventHandler<'_> {
                ~ {
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
                }
            }

            BezierEventHandler<'_> {
                ~ {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,
                    curve::SetSamples,
                    curve::GetSamples,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                }

                ! {
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                }
            }

            ConvexHullEventHandler<'_> {
                ~ {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                }

                ! {
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                    curve::SetSamples,
                    curve::GetSamples,
                }
            }

            InterpolationEventHandler<'_> {
                ~ {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,
                    curve::SetSamples,
                    curve::GetSamples,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                }

                ! {
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                }

                SetInterpolationNodes { nodes: InterpolationNodes } -> (),
                GetInterpolationNodes () -> InterpolationNodes,
            }

            PolylineEventHandler<'_> {
                ~ {
                    GetControlPointsLength,
                    AddControlPoint,
                    MovePoint,
                    DeletePoint,

                    canvas::RotateCurve,
                    canvas::MoveCurve,
                }

                ! {
                    weighted::AddWeightedControlPoint,
                    weighted::ChangeWeight,
                    weighted::GetWeight,
                    curve::SetSamples,
                    curve::GetSamples,
                }
            }

            ControlPointsEventHandler<'_> {
                ~ {
                    canvas::RotateCurve,
                    canvas::MoveCurve,
                }

                GetControlPointsLength () -> usize,
                AddControlPoint { point: Point<f32> } -> (),
                MovePoint { id: PointId, shift: Vector<f32> } -> (),
                DeletePoint { id: PointId } -> (),
            }
        }

        pub mod weighted {
            use super::*;

            declare_events! {
                RationalBezierEventHandler<'_> {
                    ~ {
                        control_points::GetControlPointsLength,
                        control_points::MovePoint,
                        control_points::DeletePoint,
                        curve::SetSamples,
                        curve::GetSamples,

                        canvas::RotateCurve,
                        canvas::MoveCurve,
                    }

                    ! {
                        control_points::AddControlPoint
                    }

                    ChangeWeight { id: PointId, weight: f32 } -> (),
                    AddWeightedControlPoint { point: WeightedPoint<f32, f32> } -> (),
                    GetWeight { id: PointId } -> f32,
                }
            }
        }
    }
}
