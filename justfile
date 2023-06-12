run-wayland *flags:
    cargo run --features wayland {{flags}}

run-x11 *flags:
    cargo run --features x11 {{flags}}
