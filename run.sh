#!/bin/env bash
cd "$(dirname $0)"
set -e

./build.sh

target="$(rustc -vV | sed -n 's|host: ||p')"
"./target/$target/release/nscompress" "$@"