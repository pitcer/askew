use crate::canvas::transition::progress::Progress;

#[derive(Debug, Copy, Clone)]
pub struct LinearMapping {
    from: f32,
    to: f32,
}

impl LinearMapping {
    #[must_use]
    pub fn new(from: f32, to: f32) -> Self {
        Self { from, to }
    }

    #[must_use]
    pub fn map(&self, progress: Progress) -> f32 {
        self.from * (1.0 - progress.0) + self.to * progress.0
    }
}
