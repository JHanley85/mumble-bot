extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native={}", "C:\\gstreamer\\1.0\\x86_64\\lib");
    println!("cargo:rustc-link-search=native={}", ".\\native");
    println!("cargo:rustc-link-lib=dylib=ovraudio64");

    // let bindings = bindgen::Builder::default()
    //     .header("native/OVR_Audio.h")
    //     .generate()
    //     .expect("Unable to generate bindings");

    // let out_path = PathBuf::from("src/ovraudio");
    // bindings
    //     .write_to_file(out_path.join("sys.rs"))
    //     .expect("Couldn't write bindings!");
}
