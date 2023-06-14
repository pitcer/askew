set positional-arguments

run-wayland *arguments:
    cargo run --features wayland $@

run-x11 *arguments:
    cargo run --features x11 $@

clippy-wayland:
    cargo clippy --features wayland

clippy-x11:
    cargo clippy --features x11
