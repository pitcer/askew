set positional-arguments

run-wayland *arguments:
    cargo run --no-default-features --features wayland "$@"

clippy-wayland:
    cargo clippy --no-default-features --features wayland

build-release:
    RUSTFLAGS="-Clink-arg=-fuse-ld=lld" cargo build --release

build-wasm-rust-scripts:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd scripts/wasm-rust/
    cargo build

    for file in $(cd target/wasm32-unknown-unknown/debug/ && ls *.wasm); do
        wasm-tools component new target/wasm32-unknown-unknown/debug/$file -o $file
    done
