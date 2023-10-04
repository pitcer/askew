use crate::wasm::wit::curve::CurveId;

#[derive(Debug)]
pub enum Request {
    GetPosition { id: CurveId },
    MoveCurve { id: CurveId, horizontal: f32, vertical: f32 },
    RotateCurve { id: CurveId, angle_radians: f32 },
    Sleep { seconds: u64, nanoseconds: u32 },
}

#[derive(Debug)]
pub enum Response {
    Empty,

    /// Wake from sleep
    Sleep,

    GetPosition {
        horizontal: f32,
        vertical: f32,
    },
}
