use crate::canvas::Canvas;
use crate::ui::command::CommandState;
use crate::ui::frame::mode::ModeState;
use tiny_skia::Pixmap;

pub struct FrameView<'a> {
    // NOTE: A flaw in my event system -- event handler needs mutable reference even for immutable
    // requests, so unfortunately we need mutable borrow here.
    pub canvas: &'a mut Canvas,
    pub background: &'a Option<Pixmap>,
    pub command: &'a CommandState,
    pub mode: &'a ModeState,
}

impl<'a> FrameView<'a> {
    pub fn new(
        canvas: &'a mut Canvas,
        background: &'a Option<Pixmap>,
        command: &'a CommandState,
        mode: &'a ModeState,
    ) -> Self {
        Self {
            canvas,
            background,
            command,
            mode,
        }
    }
}
