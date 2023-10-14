use anyhow::Result;
use tiny_skia::{Pixmap, PixmapMut};

pub mod base_polyline;
pub mod bezier_curve;
pub mod bezier_event_handler;
pub mod control_points_curve;
pub mod shape;
pub mod visual_path;

// TODO: will this trait be useful anywhere?
pub trait DrawOn {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>) -> Result<()>;
}

// TODO: in event handler add mut events that will call that method (all curves
// must implement this trait)
pub trait Update {
    fn update(&mut self) -> Result<()>;
}
