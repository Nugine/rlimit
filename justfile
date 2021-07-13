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
    python3 -m scripts.search_resource > tmp
    python3 -m scripts.replace tmp src/unix/resource.rs '// #begin-codegen' '// #end-codegen'
    python3 -m scripts.search_rlim > tmp
    python3 -m scripts.replace tmp src/unix.rs '// #begin-codegen' '// #end-codegen'
    rm tmp
