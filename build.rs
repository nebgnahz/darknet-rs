extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("darknet-sys/src/wrapper.h")
        .whitelist_recursively(true)
        .whitelisted_type("Rect")
        .whitelisted_type("Detections")
        .whitelisted_type("Darknet")
        .whitelisted_type("InputImage")
        .whitelisted_type("image")
        .whitelisted_function("darknet_new")
        .whitelisted_function("darknet_drop")
        .whitelisted_function("darknet_detect")
        .whitelisted_function("make_image")
        .link("darknet-sys/darknet.so")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native=darknet-sys");
    println!("cargo:rustc-link-lib=darknet");

    // CUDA
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cuda");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=cublas");
    println!("cargo:rustc-link-lib=curand");
}
