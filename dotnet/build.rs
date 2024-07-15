fn main() {
    let header_path = "../clib/cute_lights.h";
    println!("cargo:rerun-if-changed={}", header_path);
    let target_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = format!("{}/bindings.rs", target_dir);
    // Remove the unended header includes
    let header_contents = std::fs::read_to_string(header_path)
        .unwrap()
        .replace("#include <stdarg.h>", "")
        .replace("#include <stdlib.h>", "");
    bindgen::Builder::default()
        .header_contents("cute_lights.h", &header_contents)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    csbindgen::Builder::default()
        .input_bindgen_file(&out_path) // read from bindgen generated code
        .csharp_dll_name("libcutelight.so")
        .csharp_namespace("CuteLights.Sdk")
        .generate_to_file(out_path, "./src/NativeMethods.g.cs".to_string())
        .unwrap();
}
