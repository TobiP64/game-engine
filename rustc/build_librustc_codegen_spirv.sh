#!/bin/bash

cd "${0%/*}"
mkdir tmp
cd tmp
git clone https://github.com/EmbarkStudios/rust-gpu.git
cd rust-gpu/crates/rustc_codegen_spirv/
cargo build --release
cd "${0%/*}"
cp tmp/rust-gpu/target/release/*.so
rm -rf tmp/