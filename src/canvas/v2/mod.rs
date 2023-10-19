use tiny_skia::PixmapMut;

pub mod base_line;
pub mod control_points_curve;
pub mod curve;
pub mod request;
pub mod shape;
pub mod visual_path;

// TODO: will this trait be useful anywhere?
pub trait DrawOn {
    fn draw_on(&self, pixmap: &mut PixmapMut<'_>);
}

// TODO: in event handler add mut events that will call that method (all curves
// must implement this trait)
pub trait Update {
    fn update(&mut self);
}
