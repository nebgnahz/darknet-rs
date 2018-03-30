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
    include!("ffi.rs");
}

use std::ffi::CString;
use std::path::Path;
use failure::Error;

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

/// Perform detection.
pub fn detect<P: AsRef<Path>>(
    mut network: Network,
    meta: Meta,
    image: P,
    thresh: f32,
    hier_thresh: f32,
    nms: f32,
) -> Result<Vec<Detection>, Error> {
    let image = Image::load(image)?;
    network.predict_image(image);
    let dets = network.get_network_boxes(image.w, image.h, thresh, hier_thresh);
    let detections = dets.postprocess(nms, &meta);
    Ok(detections)
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
    pub fn predict(&mut self, data: *mut f32) -> *mut f32 {
        unsafe { ffi::network_predict(self.net, data) }
    }

    /// Perform prediction.
    pub fn predict_image(&mut self, image: Image) -> *mut f32 {
        unsafe { ffi::network_predict_image(self.net, image) }
    }

    /// Get the network boxes.
    fn get_network_boxes(&mut self, w: i32, h: i32, thresh: f32, hier: f32) -> Detections_ {
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

/// Detection.
#[derive(Debug, Copy, Clone)]
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
            self.nms(self.num, meta.classes, nms);
        }

        let mut res = Vec::new();
        for j in 0..(self.num as isize) {
            let d = unsafe { *self.inner.offset(j) };
            let bbox = d.bbox;
            let probs = d.prob;
            for i in 0..(meta.classes as isize) {
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
                    });
                }
            }
        }

        res.sort_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap());
        res
    }
}

/// Metadata.
pub type Meta = ffi::metadata;

impl Meta {
    /// Load a new metadata.
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        let meta = unsafe { ffi::get_metadata(filename) };
        Ok(meta)
    }
}

/// Image
pub type Image = ffi::image;

impl Image {
    /// Create a new image.
    pub fn new(w: i32, h: i32, c: i32) -> Self {
        unsafe { ffi::make_image(w, h, c) }
    }

    /// Load a new image.
    pub fn load<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        let filename = path_to_cstring(filename)?.into_raw();
        let img = unsafe { ffi::load_image(filename, 0, 0, 0) };
        Ok(img)
    }
}
