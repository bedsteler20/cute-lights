use std::env;

pub fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // check if a feature is enabled
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("cute_lights.h");
}
