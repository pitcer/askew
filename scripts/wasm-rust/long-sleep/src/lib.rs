wit_bindgen::generate!({
    path: "../../../wit/askew.wit",

    exports: {
        world: LongSleep,
    }
});

struct LongSleep;

impl Guest for LongSleep {
    fn run(_argument: RunArgument) -> RunResult {
        loop {
            control::sleep(5, 0);
            control::tick();
        }
    }
}
