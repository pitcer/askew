set positional-arguments

run-wayland *arguments:
    cargo run --no-default-features --features wayland $@

clippy-wayland:
    cargo clippy --no-default-features --features wayland
