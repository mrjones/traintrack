#!/usr/bin/env bash
# 1. Install generic, Google protoc:
# - Best is to get a new version from: https://github.com/google/protobuf/releases
# - You can also do: sudo apt-get install protobuf-compiler  (But this is pretty old).
# 2. Install rust protoc plugin:
# - cargo install protobuf
# 3. Install pbjs/pbts:
# - npm install (already defined in project.json)
~/downloads/proto/bin/protoc --proto_path ./proto/ --plugin ~/.cargo/bin/protoc-gen-rust --rust_out ./src/ proto/*.proto

# ~/downloads/proto/bin/protoc --proto_path ./proto/ --js_out import_style=commonjs,binary:./webclient/src/ proto/webclient_api.proto

PBJS=./webclient/node_modules/protobufjs/bin/pbjs
PBTS=./webclient/node_modules/protobufjs/bin/pbts

$PBJS -t static-module -w commonjs -o webclient/src/webclient_api_pb.js proto/webclient_api.proto

$PBJS -t static-module proto/webclient_api.proto | $PBTS -o webclient/src/webclient_api_pb.d.ts -

D_TS_FILE=webclient/src/webclient_api_pb.d.ts

echo "// NOTE: THIS WAS ADDED MANUALLY" >> $D_TS_FILE
echo "// https://github.com/dcodeIO/protobuf.js/issues/780" >> $D_TS_FILE
echo "export enum Direction {" >> $D_TS_FILE
echo "    UPTOWN," >> $D_TS_FILE
echo "    DOWNTOWN," >> $D_TS_FILE
echo "}" >> $D_TS_FILE
