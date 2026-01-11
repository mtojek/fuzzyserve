#!/bin/bash

docker run --platform linux/arm64 -v "$PWD":/work -w /work messense/rust-musl-cross:aarch64-musl
# cargo build --target aarch64-unknown-linux-musl
