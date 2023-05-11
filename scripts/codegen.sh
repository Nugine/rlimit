#!/bin/bash -e
./scripts/download-libc.sh
cargo run -p rlimit-codegen
cargo fmt
echo "done"
