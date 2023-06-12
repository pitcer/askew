set positional-arguments

run-wayland *arguments:
    cargo run --features wayland $@

run-x11 *arguments:
    cargo run --features x11 $@
