#!/bin/bash

echo "Provisioning VM"
echo "Rustup"
curl https://sh.rustup.rs -sSf | sh -s -- -y
echo "Add Toolchain for ios"
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
cargo install cargo-lipo