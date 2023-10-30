wit_bindgen::generate!({
    path: "../../../wit/askew.wit",

    exports: {
        world: Fibonacci,
    }
});

struct Fibonacci;

impl Guest for Fibonacci {
    fn run(argument: RunArgument) -> RunResult {
        let Some(argument) = argument else {
            return Err("Missing argument".to_owned());
        };
        let Ok(n) = argument.parse::<u32>() else {
            return Err("Argument should be u32-compatible".to_owned());
        };
        let fibonacci = fibonacci(n);
        Ok(Some(format!("{fibonacci}")))
    }
}

fn fibonacci(n: u32) -> u64 {
    if n == 26 {
        control::yield_to_window();
    }
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
