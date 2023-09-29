use std::f32::consts;

wit_bindgen::generate!({
    path: "../../../../wit/askew.wit",

    exports: {
        world: Rotate,
    }
});

struct Rotate;

impl Guest for Rotate {
    fn run() -> Result<(), ()> {
        askew::rotate_curve(0, consts::PI / 2.0);
        askew::sleep();
        askew::rotate_curve(0, consts::PI / 2.0);
        Ok(())
    }
}
