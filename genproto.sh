#!/usr/bin/env bash
# Get protc from: https://github.com/google/protobuf/releases
# sudo apt-get install protobuf-compiler  <-- this is pretty old
~/downloads/proto/bin/protoc --proto_path ./proto/ --plugin ~/.cargo/bin/protoc-gen-rust --rust_out ./src/ proto/*.proto
~/downloads/proto/bin/protoc --proto_path ./proto/ --js_out import_style=commonjs,binary:./webclient/src/ proto/webclient_api.proto
