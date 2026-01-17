#!/bin/bash

docker run -it --platform linux/arm64 -v "$PWD":/work -w /work messense/rust-musl-cross:aarch64-musl /bin/bash
# cargo build --target aarch64-unknown-linux-musl
