#!/usr/bin/env bash
# Get protc from: https://github.com/google/protobuf/releases
# sudo apt-get install protobuf-compiler
~/src/bin/protoc --proto_path ./proto/ --plugin ~/.cargo/bin/protoc-gen-rust --rust_out ./src/ proto/*.proto
