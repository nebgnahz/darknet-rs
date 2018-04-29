#[cfg(feature = "gen")]
extern crate bindgen;

#[cfg(feature = "gen")]
fn gen() {
    use std::env;
    use std::path::PathBuf;
    let bindings = bindgen::Builder::default()
        .header("darknet-sys/include/darknet.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "gen"))]
fn gen() {}

fn main() {
    gen();

    println!("cargo:rustc-link-lib=darknet");

    if cfg!(feature = "nnpack") {
        println!("cargo:rustc-link-lib=nnpack");
        println!("cargo:rustc-link-lib=pthreadpool");
        println!("cargo:rustc-link-lib=pthread");
    }

    if cfg!(feature = "cuda") {
        println!("cargo:rustc-link-search=native=/usr/local/cuda/lib");
        println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
        println!("cargo:rustc-link-lib=cuda");
        println!("cargo:rustc-link-lib=cudart");
        println!("cargo:rustc-link-lib=cublas");
        println!("cargo:rustc-link-lib=curand");
    }
}
