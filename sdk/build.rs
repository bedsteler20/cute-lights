use std::env;

pub fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // check if a feature is enabled
    if cfg!(feature = "c_api") {
        cbindgen::Builder::new()
            .with_crate(crate_dir)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file("target/include/cute_lights.h");
    }
}
