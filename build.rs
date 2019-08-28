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

extern crate built;
fn main() {
    let tt_version = std::env::var("TRAINTRACK_VERSION").unwrap_or("UNKNOWN".to_string());
    println!("cargo:rustc-env=TRAINTRACK_VERSION={}", tt_version);

    built::write_built_file().expect("Failed to acquire build-time information");

    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(Serialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
    config.compile_protos(&["proto/feedproxy_api.proto",
                                  "proto/gtfs-realtime.proto",
                                  "proto/nyct-subway.proto",
                                  "proto/webclient_api.proto"],
                                &["proto/"]).unwrap();

}
