#!/bin/bash

# Get version number from version tag
CRATE_VERSION=`echo $1 | cut -d'v' -f2`

set-cargo-version ./catalog/Cargo.toml $CRATE_VERSION
set-cargo-version ./collecting/Cargo.toml $CRATE_VERSION
set-cargo-version ./social/Cargo.toml $CRATE_VERSION
set-cargo-version ./webapi/Cargo.toml $CRATE_VERSION

cargo build --release
mkdir release && cp target/release/trenako-web-api release
