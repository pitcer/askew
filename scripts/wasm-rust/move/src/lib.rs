wit_bindgen::generate!({
    path: "../../../wit/askew.wit",

    exports: {
        world: Move,
    }
});

struct Move;

impl Guest for Move {
    fn run(_argument: RunArgument) -> RunResult {
        let mut direction_forward = true;
        loop {
            {
                control::lock();
                let position = curve::get_position(0);
                if direction_forward && (position.0 > 300.0 || position.1 > 300.0) {
                    direction_forward = false;
                }
                if !direction_forward && (position.0 < 100.0 || position.1 < 100.0) {
                    direction_forward = true;
                }

                if direction_forward {
                    curve::move_by(0, 1.0, 1.0);
                } else {
                    curve::move_by(0, -1.0, -1.0);
                }
                control::unlock();
                control::redraw();
            }

            control::sleep(0, 10_000_000);
        }
    }
}
