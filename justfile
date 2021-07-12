# https://github.com/casey/just

fmt:
    cargo fmt --all

check: fmt
    cargo check
    cargo clippy -- -D warnings

test: check
    cargo test --all-features -- --test-threads=1
    cargo run --example nofile

codegen:
    python3 -m scripts.search_resource > src/unix/__resources.rs
    python3 -m scripts.search_rlim > src/unix/__rlims.rs
