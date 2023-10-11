use anyhow::Result;
use tiny_skia::PixmapMut;

pub mod base_polyline;
pub mod bezier;
pub mod control_points_curve;
pub mod shape;
pub mod visual_path;

// TODO: will this trait be useful anywhere?
pub trait DrawOn {
    fn draw_on(&self, pixmap: PixmapMut<'_>) -> Result<()>;
}
