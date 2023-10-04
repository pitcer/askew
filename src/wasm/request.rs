use crate::wasm::askew::CurveId;

#[derive(Debug)]
pub enum Request {
    RotateCurve { id: CurveId, angle_radians: f32 },
    Sleep { seconds: u64, nanoseconds: u32 },
}

#[derive(Debug)]
pub enum Response {
    Empty,

    /// Wake from sleep
    Sleep,
}
