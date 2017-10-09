extern crate built;
fn main() {
    let tt_version = std::env::var("TRAINTRACK_VERSION").unwrap_or("UNKNOWN".to_string());
    println!("cargo:rustc-env=TRAINTRACK_VERSION={}", tt_version);
//    if tt_version == "UNKNOWN" {
//        println!("cargo:warning=TRAINTRACK_VERSION not set");
//    }

    built::write_built_file().expect("Failed to acquire build-time information");
}
