use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::request::declare::RotateCurveById;
use crate::canvas::shape::request::declare::{GetCurveCenter, MoveCurve};
use crate::request::{RequestSubHandler, RequestSubHandlerMut};
use crate::wasm::state::State;
use crate::wasm::wit::shape::Host;
use crate::wasm::wit::shape::ObjectId;

#[async_trait::async_trait]
impl Host for State {
    async fn get_position(&mut self, _id: ObjectId) -> anyhow::Result<(f32, f32)> {
        let frame = self.frame.lock().await;
        // TODO: get position of curve specified by id
        let center = frame.sub_handle(GetCurveCenter)?;
        // TODO: return None instead of (0, 0)
        let center = center.unwrap_or_else(|| Point::new(0.0, 0.0));
        Ok((center.horizontal(), center.vertical()))
    }

    async fn move_by(
        &mut self,
        _id: ObjectId,
        horizontal: f32,
        vertical: f32,
    ) -> anyhow::Result<()> {
        let mut frame = self.frame.lock().await;
        // TODO: move curve specified by id
        let shift = Vector::new(horizontal, vertical);
        frame.sub_handle_mut(MoveCurve::new(shift))?;
        Ok(())
    }

    async fn rotate_by(&mut self, id: ObjectId, angle_radians: f32) -> anyhow::Result<()> {
        let mut frame = self.frame.lock().await;
        frame.sub_handle_mut(RotateCurveById::new(angle_radians, id as usize))?;
        Ok(())
    }
}
