#!/usr/bin/env bash

rm -rf src && mkdir src &&
svd2rust -i SWM050.svd &&
form -i lib.rs -o src && rm lib.rs &&
cargo fmt &&
rustfmt build.rs
