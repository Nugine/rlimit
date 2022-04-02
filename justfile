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

local-codegen:
    #!/bin/bash -e
    cd {{justfile_directory()}}
    mkdir -p target
    OUTRS=target/out.rs
    ./scripts/codegen.sh $OUTRS
    echo "arch={{arch()}}, os={{os()}}"
    echo "saved in $OUTRS"
