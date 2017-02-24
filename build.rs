extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("wrapper.h")
        .whitelist_recursively(true)
        .whitelisted_type("list")
        .whitelisted_function("read_data_cfg")
        .whitelisted_function("option_find_str")
        .link("darknet-sys/darknet.so")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
