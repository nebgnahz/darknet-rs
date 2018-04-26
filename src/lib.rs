//! Bindings for Darknet

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

#[macro_use]
extern crate failure;

/// FFI bindings.
pub mod ffi {
    #![allow(missing_docs)]
    #![allow(missing_debug_implementations)]
    #![allow(missing_copy_implementations)]
    #![allow(trivial_casts)]
    #![allow(trivial_numeric_casts)]
    #![allow(unused_import_braces)]
    #![allow(unused_qualifications)]
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    #[cfg(feature = "gen")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[cfg(not(feature = "gen"))]
    #[cfg(target_os = "macos")]
    include!("ffi_osx.rs");

    #[cfg(not(feature = "gen"))]
    #[cfg(target_os = "linux")]
    include!("ffi_linux.rs");
}

use std::ffi::{CStr, CString};
use std::path::Path;
pub use failure::Error;

mod errors {
    use std::path::PathBuf;

    #[derive(Debug, Fail)]
    /// Custom errors.
    pub enum DarknetError {
        #[fail(display = "invalid path: {:?}", _0)]
        /// Indicates that path was invalid
        InvalidPath(PathBuf),
    }
}

/// Network
#[derive(Debug)]
pub struct Network {
    net: *mut ffi::network,
}

fn path_to_cstring<P: AsRef<Path>>(path: P) -> Result<CString, Error> {
    let path = path.as_ref();
    let x = path.to_str()
        .ok_or(errors::DarknetError::InvalidPath(path.into()))?;
    let result = CString::new(x)?;
    Ok(result)
}

/// Perform simple detection with default threshold.
#[inline]
pub fn simple_detect(network: &Network, meta: &Meta, image: &Image) -> Vec<Detection> {
    let thres = 0.5;
    let hier_thresh = 0.5;
    let nms = 0.45;
    detect(network, meta, image, thres, hier_thresh, nms)
}

/// Perform detection.
pub fn detect(
    network: &Network,
    meta: &Meta,
    image: &Image,
    thresh: f32,
    hier_thresh: f32,
    nms: f32,
) -> Vec<Detection> {
    network.predict_image(image);
    let dets = network.get_network_boxes(image.0.w, image.0.h, thresh, hier_thresh);
    let detections = dets.postprocess(nms, meta);
    detections
}

impl Network {
    /// Create a new network.
    pub fn new<P: AsRef<Path>>(config: P, weight: P) -> Result<Self, Error> {
        let config = path_to_cstring(config)?.into_raw();
        let weight = path_to_cstring(weight)?.into_raw();
        let net = unsafe { ffi::load_network(config, weight, 0) };
        Ok(Network { net: net })
    }

    /// Return the width of the network.
    pub fn width(&self) -> usize {
        unsafe { ffi::network_width(self.net) as usize }
    }

    /// Return the height of the network.
    pub fn height(&self) -> usize {
        unsafe { ffi::network_height(self.net) as usize }
    }

    /// Perform prediction.
    pub fn predict(&self, data: *mut f32) -> *mut f32 {
        unsafe { ffi::network_predict(self.net, data) }
    }

    /// Perform prediction.
    pub fn predict_image(&self, image: &Image) -> *mut f32 {
        unsafe { ffi::network_predict_image(self.net, image.0) }
    }

    /// Get the network boxes.
    fn get_network_boxes(&self, w: i32, h: i32, thresh: f32, hier: f32) -> Detections_ {
        let mut num = 0;
        let det = unsafe {
            ffi::get_network_boxes(
                self.net,
                w,
                h,
                thresh,
                hier,
                std::ptr::null_mut(),
                0,
                &mut num,
            )
        };
        Detections_ {
            inner: det,
            num: num,
        }
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe {
            ffi::free_network(self.net);
        }
    }
}

/// A rectangle type.
pub type Rect = ffi::box_;

/// Detection. Note that the bounding box is centered at (x, y) and have width
/// `w` and height `h`.
#[derive(Debug, Clone)]
pub struct Detection {
    /// The class.
    pub class: i32,
    /// x coordinate.
    pub x: f32,
    /// y coordinate.
    pub y: f32,
    /// width.
    pub w: f32,
    /// height.
    pub h: f32,
    /// probability.
    pub prob: f32,
    /// name.
    pub name: String,
}

/// Internal Detection.
#[derive(Debug)]
struct Detections_ {
    inner: *mut ffi::detection,
    num: i32,
}

impl Drop for Detections_ {
    fn drop(&mut self) {
        unsafe {
            ffi::free_detections(self.inner, self.num);
        }
    }
}

impl Detections_ {
    /// Non-maximum suppression.
    fn nms(&self, total: i32, classes: i32, thresh: f32) {
        unsafe {
            ffi::do_nms_obj(self.inner, total, classes, thresh);
        }
    }

    /// Filter detection.
    pub fn postprocess(&self, nms: f32, meta: &Meta) -> Vec<Detection> {
        if nms > 0.0 {
            self.nms(self.num, meta.num_classes(), nms);
        }

        let mut res = Vec::new();
        for j in 0..(self.num as isize) {
            let d = unsafe { *self.inner.offset(j) };
            let bbox = d.bbox;
            let probs = d.prob;
            for i in 0..(meta.num_classes() as isize) {
                let p = unsafe { *probs.offset(i) };
                if p > 0.0 {
                    let ffi::box_ { x, y, w, h } = bbox;
                    res.push(Detection {
                        class: i as i32,
                        x: x,
                        y: y,
                        w: w,
                        h: h,
                        prob: p,
                        name: meta.class_name(i as usize).to_string(),
                    });
                }
            }
        }

        res.sort_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap());
        res
    }
}

/// Metadata.
#[derive(Debug)]
pub struct Meta {
    names: Vec<String>,
}

impl From<ffi::metadata> for Meta {
    fn from(meta: ffi::metadata) -> Self {
        let names = (0..(meta.classes as isize))
            .map(|i| unsafe {
                CStr::from_ptr(*(meta.names.offset(i)))
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        Meta { names: names }
    }
}

impl Meta {
    /// Load a new metadata.
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        let meta = unsafe { ffi::get_metadata(filename) };
        Ok(meta.into())
    }

    /// Return the number of classes.
    pub fn num_classes(&self) -> i32 {
        self.names.len() as i32
    }

    /// Return the class name identified by the classification index.
    pub fn class_name(&self, i: usize) -> &str {
        &self.names[i]
    }
}

/// Image
#[derive(Debug, Clone)]
pub struct Image(pub ffi::image);

unsafe impl Send for Image {}

impl Image {
    /// Create a new image.
    pub fn new(w: i32, h: i32, c: i32) -> Self {
        Image(unsafe { ffi::make_image(w, h, c) })
    }

    /// Load a new image.
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        let img = unsafe { ffi::load_image(filename, 0, 0, 0) };
        Ok(Image(img))
    }

    /// Load a new image with color.
    pub fn load_color<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        let img = unsafe { ffi::load_image_color(filename, 0, 0) };
        Ok(Image(img))
    }

    /// Decode a new image (always color, 3 channels).
    pub fn decode_jpg(buf: &[u8]) -> Self {
        let image = unsafe { ffi::decode_image_jpg(buf.as_ptr(), buf.len() as i32, 3) };
        Image(image)
    }

    /// Draw a box based on the detection.
    pub fn draw_box(&mut self, d: &Detection, w: i32, r: f32, g: f32, b: f32) {
        let x1 = d.x - d.w / 2.;
        let x2 = d.x + d.w / 2.;
        let y1 = d.y - d.h / 2.;
        let y2 = d.y + d.h / 2.;

        unsafe {
            ffi::draw_box_width(
                self.0,
                x1 as i32,
                y1 as i32,
                x2 as i32,
                y2 as i32,
                w,
                r,
                g,
                b,
            )
        }
    }

    /// Save the image.
    pub fn save<P: AsRef<Path>>(&self, filename: P) -> Result<(), Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        unsafe { ffi::save_image(self.0, filename) };
        Ok(())
    }

    /// Save the image as jpg.
    pub fn save_jpg<P: AsRef<Path>>(&self, filename: P) -> Result<(), Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        unsafe { ffi::save_image_jpg(self.0, filename) };
        Ok(())
    }

    /// Save the image as jpg.
    pub fn encode_jpg(&self) -> Vec<u8> {
        let cap = (self.0.w * self.0.h * self.0.c) as usize;
        let mut data: Vec<u8> = Vec::with_capacity(cap);
        let size = unsafe { ffi::encode_image_jpg(self.0, data.as_ptr()) };
        unsafe {
            data.set_len(size as usize);
        }

        data
    }

    /// Resize and return a new image.
    pub fn resize(&self, w: i32, h: i32) -> Image {
        Image(unsafe { ffi::resize_image(self.0, w, h) })
    }

    /// Image width.
    pub fn width(&self) -> i32 {
        self.0.w
    }

    /// Image height.
    pub fn height(&self) -> i32 {
        self.0.h
    }

    /// Image channel.
    pub fn channel(&self) -> i32 {
        self.0.c
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            ffi::free_image(self.0);
        }
    }
}

/// Groundtruth.
#[derive(Debug)]
pub struct Groundtruth {
    boxes: Vec<ffi::box_label>,
}

impl Groundtruth {
    /// Load a new label.
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        let mut num = 0;
        let truth = unsafe { ffi::read_boxes(filename, &mut num) };

        let truth = (0..num)
            .map(|i| unsafe { *(truth.offset(i as isize)) })
            .collect();
        Ok(Groundtruth { boxes: truth })
    }

    /// Return the rectangle box at a particular index.
    pub fn box_at(&self, i: usize) -> Rect {
        Rect {
            x: self.boxes[i].x,
            y: self.boxes[i].y,
            w: self.boxes[i].w,
            h: self.boxes[i].h,
        }
    }
}
