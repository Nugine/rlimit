#!/bin/bash -ex

# Always track the latest libc `main`, like the CI does.
# A cached clone is reused, but never trusted as-is.

if [ ! -d "temp/libc" ]; then
    mkdir -p temp

    pushd temp
        git clone https://github.com/rust-lang/libc.git -b main --depth=1
    popd
else
    pushd temp/libc
        git fetch --depth=1 origin main
        git reset --hard FETCH_HEAD
        git clean -fdxq
    popd
fi
