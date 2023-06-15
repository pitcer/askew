use anyhow::Result;

use crate::canvas::math::vector::Vector;

pub trait ChangePointWeightHandler {
    fn handle_change_point_weight(
        &mut self,
        point_index: usize,
        weight_change: impl Fn(f32) -> f32,
    ) -> Result<(), CurveEventError>;
}

pub trait DeletePointHandler {
    fn handle_delete_point(&mut self, point_index: usize) -> Result<()>;
}

pub trait MovePointHandler {
    fn handle_move_point(&mut self, point_index: usize, position_change: Vector<f32>)
        -> Result<()>;
}

pub trait AddPointHandler {
    type Point;

    fn handle_add_point(&mut self, point: Self::Point) -> Result<()>;
}

#[derive(Debug, thiserror::Error)]
pub enum CurveEventError {
    #[error("unimplemented handler")]
    Unimplemented,
}
