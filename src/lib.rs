pub mod ffi;

#[repr(C)]
pub struct Size {
    width: i32,
    height: i32,
}

#[repr(C)]
pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[repr(C)]
pub struct Detections {
    inner: ffi::Detections,
}

#[repr(C)]
pub struct InputImage {
    inner: ffi::InputImage
}

pub struct Darknet {
    inner: *mut ffi::Darknet,
}

impl Darknet {
    pub fn new() -> Self {
        Darknet {
            inner: unsafe {darknet_new() },
        }
    }

    pub fn size(&self) -> Size {
        unsafe { darknet_size(self.inner) }
    }

    pub fn detect(&mut self, image: InputImage) -> Detections {
        Detections {
            inner: unsafe { darknet_detect(self.inner, image.inner) }
        }
    }
}

impl Drop for Darknet {
    fn drop(&mut self) {
        unsafe {
            darknet_drop(self.inner);
        }
    }
}

impl Drop for Detections {
    fn drop(&mut self) {
        unsafe {
            detections_drop(self.inner);
        }
    }
}

extern "C" {
    fn darknet_new() -> *mut ffi::Darknet;
    fn darknet_drop(dn: *mut ffi::Darknet);
    fn darknet_size(dn: *const ffi::Darknet) -> Size;
    fn darknet_detect(dn: *mut ffi::Darknet, image: ffi::InputImage) -> ffi::Detections;
    fn detections_drop(dt: ffi::Detections);
}
