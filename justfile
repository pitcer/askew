set positional-arguments

run-wayland *arguments:
    cargo run --no-default-features --features wayland $@

clippy-wayland:
    cargo clippy --no-default-features --features wayland

build-release:
    RUSTFLAGS="-Clink-arg=-fuse-ld=lld" cargo build --release

build-wasm-example name:
    cargo build --package {{name}} --target wasm32-unknown-unknown
    wasm-tools component new target/wasm32-unknown-unknown/debug/{{name}}.wasm -o {{name}}.wasm
