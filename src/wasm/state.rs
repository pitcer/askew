use std::time::Duration;

use anyhow::{anyhow, Result};
use async_channel::Sender;
use async_io::Timer;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiView};

use crate::canvas::math::point::Point;
use crate::canvas::math::vector::Vector;
use crate::canvas::request::declare::RotateCurveById;
use crate::canvas::shape::request::declare::{GetCurveCenter, MoveCurve};
use crate::request::{RequestSubHandler, RequestSubHandlerMut};
use crate::ui::handler::message::{HandlerMessage, HandlerSender};
use crate::ui::shared::SharedFrame;
use crate::ui::task::lock::{LockToken, TaskLock};
use crate::wasm::wit::curve::CurveId;
use crate::wasm::wit::{control, curve};

pub struct State {
    sender: HandlerSender,
    frame: SharedFrame,
    lock: TaskLock,
    lock_token: Option<LockToken>,

    wasi_table: Table,
    wasi_context: WasiCtx,
}

impl State {
    #[must_use]
    pub fn new(
        sender: HandlerSender,
        frame: SharedFrame,
        lock: TaskLock,
        wasi_table: Table,
        wasi_context: WasiCtx,
    ) -> Self {
        let lock_token = None;
        Self { sender, frame, lock, lock_token, wasi_table, wasi_context }
    }
}

#[derive(Debug)]
pub struct YieldResponse(Sender<()>);

impl YieldResponse {
    fn new(sender: Sender<()>) -> Self {
        Self(sender)
    }

    pub fn send(&self) {
        self.0.try_send(()).expect("Cannot send yield response");
    }
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.wasi_table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.wasi_table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.wasi_context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi_context
    }
}

#[async_trait::async_trait]
impl curve::Host for State {
    async fn get_position(&mut self, _id: CurveId) -> Result<(f32, f32)> {
        let frame = self.frame.lock().await;
        // TODO: get position of curve specified by id
        let center = frame.sub_handle(GetCurveCenter)?;
        // TODO: return None instead of (0, 0)
        let center = center.unwrap_or_else(|| Point::new(0.0, 0.0));
        Ok((center.horizontal(), center.vertical()))
    }

    async fn move_by(&mut self, _id: CurveId, horizontal: f32, vertical: f32) -> Result<()> {
        let mut frame = self.frame.lock().await;
        // TODO: move curve specified by id
        let shift = Vector::new(horizontal, vertical);
        frame.sub_handle_mut(MoveCurve::new(shift))?;
        Ok(())
    }

    async fn rotate_by(&mut self, id: CurveId, angle_radians: f32) -> Result<()> {
        let mut frame = self.frame.lock().await;
        frame.sub_handle_mut(RotateCurveById::new(angle_radians, id as usize))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl control::Host for State {
    async fn sleep(&mut self, seconds: u64, nanoseconds: u32) -> Result<()> {
        let duration = Duration::new(seconds, nanoseconds);
        let timer = Timer::after(duration);
        timer.await;
        Ok(())
    }

    async fn lock(&mut self) -> Result<()> {
        let None = self.lock_token else {
            return Err(anyhow!("Tried to lock without unlocking first"));
        };
        let token = self.lock.acquire().await;
        self.lock_token = Some(token);
        Ok(())
    }

    async fn unlock(&mut self) -> Result<()> {
        let Some(token) = self.lock_token.take() else {
            return Err(anyhow!("Tried to unlock without locking first"));
        };
        self.lock.release(token);
        Ok(())
    }

    async fn yield_to_window(&mut self) -> Result<()> {
        let (sender, receiver) = async_channel::bounded(1);
        let response = YieldResponse::new(sender);
        self.sender.send_event(HandlerMessage::TaskYield(response))?;
        receiver.recv().await?;
        Ok(())
    }

    async fn redraw(&mut self) -> Result<()> {
        self.sender.send_event(HandlerMessage::Redraw)?;
        Ok(())
    }
}
