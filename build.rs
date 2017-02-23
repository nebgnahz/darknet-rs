extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("darknet-sys/src/gemm.h")
        .header("darknet-sys/src/layer.h")
        .header("darknet-sys/src/dropout_layer.h")
        .header("darknet-sys/src/crnn_layer.h")
        .header("darknet-sys/src/utils.h")
        .header("darknet-sys/src/network.h")
        .header("darknet-sys/src/cost_layer.h")
        .header("darknet-sys/src/deconvolutional_layer.h")
        .header("darknet-sys/src/im2col.h")
        .header("darknet-sys/src/option_list.h")
        .header("darknet-sys/src/image.h")
        .header("darknet-sys/src/gru_layer.h")
        .header("darknet-sys/src/matrix.h")
        .header("darknet-sys/src/data.h")
        .header("darknet-sys/src/batchnorm_layer.h")
        .header("darknet-sys/src/region_layer.h")
        .header("darknet-sys/src/cuda.h")
        .header("darknet-sys/src/stb_image_write.h")
        .header("darknet-sys/src/activation_layer.h")
        .header("darknet-sys/src/list.h")
        .header("darknet-sys/src/avgpool_layer.h")
        .header("darknet-sys/src/maxpool_layer.h")
        .header("darknet-sys/src/stb_image.h")
        .header("darknet-sys/src/normalization_layer.h")
        .header("darknet-sys/src/softmax_layer.h")
        .header("darknet-sys/src/demo.h")
        .header("darknet-sys/src/rnn_layer.h")
        .header("darknet-sys/src/tree.h")
        .header("darknet-sys/src/blas.h")
        .header("darknet-sys/src/classifier.h")
        .header("darknet-sys/src/shortcut_layer.h")
        .header("darknet-sys/src/reorg_layer.h")
        .header("darknet-sys/src/local_layer.h")
        .header("darknet-sys/src/route_layer.h")
        .header("darknet-sys/src/activations.h")
        .header("darknet-sys/src/box.h")
        .header("darknet-sys/src/crop_layer.h")
        .header("darknet-sys/src/parser.h")
        .header("darknet-sys/src/col2im.h")
        .header("darknet-sys/src/convolutional_layer.h")
        .header("darknet-sys/src/connected_layer.h")
        .header("darknet-sys/src/detection_layer.h")
        .link("darknet-sys/darknet.so")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
