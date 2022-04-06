# https://github.com/casey/just

fmt:
    cargo fmt --all

check: fmt
    cargo check
    cargo clippy -- -D warnings

test: check
    cargo test --all-features -- --test-threads=1 --nocapture
    cargo run --example nofile

doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --open --all-features

codegen:
    #!/bin/bash -e
    cd {{justfile_directory()}}
    ./scripts/codegen.sh
