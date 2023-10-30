use crate::wasm::wit::curve::CurveId;

#[derive(Debug)]
pub enum Request {
    GetPosition { id: CurveId },
    MoveCurve { id: CurveId, horizontal: f32, vertical: f32 },
    RotateCurve { id: CurveId, angle_radians: f32 },
    Yield,
}

#[derive(Debug)]
pub enum Response {
    Empty,
    Yield,
    GetPosition { horizontal: f32, vertical: f32 },
}
