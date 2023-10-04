use std::f32::consts;

wit_bindgen::generate!({
    path: "../../../wit/askew.wit",

    exports: {
        world: Rotate,
    },
});

struct Rotate;

impl Guest for Rotate {
    fn run() -> Result<(), ()> {
        loop {
            // askew::rotate_curve(0, 2.0 * consts::PI * 1.0 / 360.0);
            askew::sleep(0, 10_000_000);
        }
    }
}
