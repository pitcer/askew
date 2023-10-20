use winit::dpi::PhysicalPosition;

use crate::request::macros::declare_requests;
use crate::request::{Change, Direction};

declare_requests! {
    { mut ToggleConvexHull () -> () },
    { mut ChangeWeight (Change) -> () },
    { mut MouseClick (PhysicalPosition<f64>) -> () },
    { mut MousePress (PhysicalPosition<f64>) -> () },
    { mut MovePoint (Direction) -> () },
    { mut Delete () -> () },
    { mut Add () -> () },
    { mut ChangeIndex (Change) -> () },
}
