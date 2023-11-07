pub mod alpha;
pub mod bezier;

pub type Progress = f32;

pub trait Transition {
    type Item;

    fn next_step(&mut self) -> Option<&Self::Item>;
}
