extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native={}", "C:\\gstreamer\\1.0\\x86_64\\lib");

    println!("cargo:rustc-link-search=native={}", ".\\native");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=dylib=ovraudio64");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("native/OVR_Audio.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("src/ovraudio");
    bindings
        .write_to_file(out_path.join("sys.rs"))
        .expect("Couldn't write bindings!");
}
