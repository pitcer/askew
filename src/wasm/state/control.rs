use std::time::Duration;

use anyhow::anyhow;
use async_io::Timer;

use crate::ui::handler::message::{HandlerMessage, YieldResponse};
use crate::wasm::state::State;
use crate::wasm::wit::control::{Host, TickInfo};

#[async_trait::async_trait]
impl Host for State {
    async fn print(&mut self, message: String) -> anyhow::Result<()> {
        println!("{message}");
        Ok(())
    }

    async fn tick(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_tick_info(&mut self) -> anyhow::Result<TickInfo> {
        let info = self.tick_state.as_tick_info();
        Ok(info)
    }

    async fn sleep(&mut self, seconds: u64, nanoseconds: u32) -> anyhow::Result<()> {
        let duration = Duration::new(seconds, nanoseconds);
        let timer = Timer::after(duration);
        timer.await;
        Ok(())
    }

    async fn lock(&mut self) -> anyhow::Result<()> {
        let None = self.lock_token else {
            return Err(anyhow!("Tried to lock without unlocking first"));
        };
        let token = self.lock.acquire().await;
        self.lock_token = Some(token);
        Ok(())
    }

    async fn unlock(&mut self) -> anyhow::Result<()> {
        let Some(token) = self.lock_token.take() else {
            return Err(anyhow!("Tried to unlock without locking first"));
        };
        self.lock.release(token);
        Ok(())
    }

    async fn yield_to_window(&mut self) -> anyhow::Result<()> {
        let (sender, receiver) = async_channel::bounded(1);
        let response = YieldResponse::new(sender);
        self.sender.send_event(HandlerMessage::TaskYield(response))?;
        receiver.recv().await?;
        Ok(())
    }

    async fn redraw(&mut self) -> anyhow::Result<()> {
        self.sender.send_event(HandlerMessage::Redraw)?;
        Ok(())
    }
}
