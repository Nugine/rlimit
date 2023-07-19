# https://github.com/casey/just

dev:
    just fmt
    just check
    just test

fmt:
    cargo fmt --all

check:
    cargo check
    cargo clippy -- -D warnings

test:
    cargo test --all-features -- --test-threads=1 --nocapture
    cargo run --example nofile

doc:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --open --all-features

codegen:
    #!/bin/bash -e
    cd {{justfile_directory()}}
    ./scripts/codegen.sh

sync-version:
    cargo set-version -p rlimit 0.10.1

publish:
    cargo publish -p rlimit
