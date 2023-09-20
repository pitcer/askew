#[derive(Debug)]
pub enum Request {
    RotateCurve { id: usize, angle: f32 },
}

#[derive(Debug)]
pub enum Response {}
