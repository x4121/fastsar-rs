#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

TAG="$(git tag)"

GNU_TARGET=x86_64-unknown-linux-gnu
MUSL_TARGET=x86_64-unknown-linux-musl
OSX_TARGET=x86_64-apple-darwin

# build Linux (GNU)
cargo build --release --target $GNU_TARGET

# build Linux (musl)
docker run -v "$DIR":/volume --rm -t clux/muslrust cargo build --release

#build OSX
docker run -v "$DIR":/root/src -w /root/src --rm joseluisq/rust-linux-darwin-builder:1.49.0 \
    sh -c "cargo build --release --target x86_64-apple-darwin"

# zip
tar -C "$DIR/target/$GNU_TARGET/release" \
    -czf "fastsar-$TAG-$GNU_TARGET.tar.gz" \
    fastsar
tar -C "$DIR/target/$MUSL_TARGET/release" \
    -czf "fastsar-$TAG-$MUSL_TARGET.tar.gz" \
    fastsar
tar -C "$DIR/target/$OSX_TARGET/release" \
    -czf "fastsar-$TAG-$OSX_TARGET.tar.gz" \
    fastsar
