use std::f32::consts;

wit_bindgen::generate!({
    path: "../../../wit/askew.wit",

    exports: {
        world: Rotate,
    }
});

struct Rotate;

impl Guest for Rotate {
    fn run(_argument: RunArgument) -> RunResult {
        loop {
            curve::rotate_by(0, 2.0 * consts::PI * 1.0 / 360.0);
            control::redraw();
            control::sleep(0, 10_000_000);
        }
    }
}
