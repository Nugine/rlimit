#!/bin/bash -e
./scripts/download-libc.sh
cargo run -p rlimit-codegen
rustfmt src/bindings.rs
echo "done"
