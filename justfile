set positional-arguments

run-wayland *arguments:
    cargo run --no-default-features --features wayland $@

clippy-wayland:
    cargo clippy --no-default-features --features wayland

build-release:
    RUSTFLAGS="-Clink-arg=-fuse-ld=lld" cargo build --release
