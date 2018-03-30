fn main() {
    println!("cargo:rustc-link-search=native=darknet-sys");
    println!("cargo:rustc-link-lib=darknet");

    // CUDA
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib");
    println!("cargo:rustc-link-lib=cuda");
    println!("cargo:rustc-link-lib=cudart");
    println!("cargo:rustc-link-lib=cublas");
    println!("cargo:rustc-link-lib=curand");
}
