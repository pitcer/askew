#[derive(Debug)]
pub enum Request {
    RotateCurve { id: usize, angle: f32 },
    Sleep { seconds: u64 },
}

#[derive(Debug)]
pub enum Response {
    Empty,
}
