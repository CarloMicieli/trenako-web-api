#!/bin/bash

# Get version number from version tag
CRATE_VERSION=`echo $1 | cut -d'v' -f2`

set-cargo-version ./workspace/catalog/Cargo.toml $CRATE_VERSION
set-cargo-version ./workspace/collecting/Cargo.toml $CRATE_VERSION
set-cargo-version ./workspace/social/Cargo.toml $CRATE_VERSION
set-cargo-version ./workspace/webapi/Cargo.toml $CRATE_VERSION

cargo build --release
mkdir release && cp target/release/trenako-web-api release

docker build -f ./.docker/webapi.dockerfile -t latest -t $1 -t $CRATE_VERSION .