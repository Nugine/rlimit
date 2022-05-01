#!/bin/bash -e
echo "updating libc"
if [ ! -d "libc" ]; then
    git clone https://github.com/rust-lang/libc.git -b master --depth=1
else
    pushd libc
    git pull
    popd
fi
mkdir -p target
python3 ./scripts/codegen.py > target/out.rs
rustfmt target/out.rs
cp  target/out.rs src/bindings.rs
echo "done"
