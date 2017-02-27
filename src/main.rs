extern crate cv;
extern crate darknet;
use darknet::*;

fn cv_mat_to_darknet_image(mat: &cv::Mat) -> darknet::InputImage {
    let data: *const u8 = mat.data();
    let h = mat.rows;
    let w = mat.cols;
    let c = mat.channels;

    let mut out = darknet::InputImage::new(w, h, c);
    let out_data = out.data_mut();
    let mut count = 0;
    for k in 0..c {
        for y in 0..h {
            for x in 0..w {
                let offset = (c * (w * y + x) + k) as isize;
                unsafe {
                    let v = *(data.offset(offset)) as f32 / 255.0;
                    *out_data.offset(count) = v;
                }
                count += 1;
            }
        }
    }
    out
}

fn main() {
    let mut dn = Darknet::new();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/darknet-sys/data/dog.jpg");
    let image = cv::Mat::from_path(path, cv::imgcodecs::ImreadModes::ImreadColor)
        .expect("failed to load image");
    let image = cv_mat_to_darknet_image(&image);
    let detections = dn.detect(image);
    println!("{:?}", detections);
}
