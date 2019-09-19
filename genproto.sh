#!/bin/bash -eu
#
# Copyright 2018 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

#!/usr/bin/env bash
# 1. Install generic, Google protoc:
# - Best is to get a new version from: https://github.com/google/protobuf/releases
# - You can also do: sudo apt-get install protobuf-compiler  (But this is pretty old).
# 2. Install rust protoc plugin:
# - cargo install protobuf
# 3. Install pbjs/pbts:
# - npm install (already defined in project.json)
~/downloads/proto/bin/protoc --proto_path ./proto/ --plugin ~/.cargo/bin/protoc-gen-rust --rust_out ./ttpredict/src/ proto/gtfs-realtime.proto

PBJS=./webclient/node_modules/protobufjs/bin/pbjs
PBTS=./webclient/node_modules/protobufjs/bin/pbts

API_PROTO=proto/webclient_api.proto

WEBCLIENT_API_D_TS=webclient/src/webclient_api_pb.d.ts
WEBCLIENT_API_JS=webclient/src/webclient_api_pb.js

$PBJS -t static-module -w commonjs -o $WEBCLIENT_API_JS $API_PROTO
$PBJS -t static-module $API_PROTO | $PBTS -o $WEBCLIENT_API_D_TS -

#echo "// NOTE: THIS WAS ADDED MANUALLY" >> $WEBCLIENT_API_D_TS
#echo "// https://github.com/dcodeIO/protobuf.js/issues/780" >> $WEBCLIENT_API_D_TS
#echo "export enum Direction {" >> $WEBCLIENT_API_D_TS
#echo "    UPTOWN," >> $WEBCLIENT_API_D_TS
#echo "    DOWNTOWN," >> $WEBCLIENT_API_D_TS
#echo "}" >> $WEBCLIENT_API_D_TS

LICENSE_TXT=$(cat <<-END
// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
END
)

GTFS_LICENSE_TXT=$(cat <<-END
// Copyright 2015 The GTFS Specifications Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
END
)

echo "${LICENSE_TXT}"$'\n\n'"$(cat $WEBCLIENT_API_D_TS)" > $WEBCLIENT_API_D_TS
echo "${LICENSE_TXT}"$'\n\n'"$(cat $WEBCLIENT_API_JS)" > $WEBCLIENT_API_JS
