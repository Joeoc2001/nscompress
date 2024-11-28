#!/bin/env bash
cd "$(dirname $0)"
set -e


target="$(rustc -vV | sed -n 's|host: ||p')"
rustup component add rust-src --toolchain nightly > /dev/null

#export RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -Clink-arg=-Wl,-z,nostart-stop-gc"
#export RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -Clink-args=-nostartfiles -Clink-arg=-Wl,-z,nostart-stop-gc,--build-id=none,-O3,-s,--exclude-libs,ALL,--gc-sections,-Bsymbolic,-n,-N"
#export RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -Clink-args=-nostartfiles -Clink-arg=-Wl,-z,nostart-stop-gc,--build-id=none,-O3,-s,--exclude-libs,ALL,--gc-sections,-Bsymbolic,-n,-N,--no-dynamic-linker,--no-pie,--no-eh-frame-hdr"
#export RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -Ctarget-cpu=native -Crelocation-model=static -Clink-args=-Wl,-n,-N,--no-dynamic-linker,--no-pie,--build-id=none,--no-eh-frame-hdr"
cargo +nightly build --release \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    -Z build-std-features="optimize_for_size" \
    --target "$target" # --no-default-features

ls -al "target/$target/release/nscompress"