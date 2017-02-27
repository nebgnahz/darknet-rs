pub mod ffi;

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

impl ::std::fmt::Debug for Detections {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Detected {} objects", self.inner.num)
    }
}

#[repr(C)]
pub struct InputImage {
    inner: ffi::image,
}

impl InputImage {
    pub fn new(w: i32, h: i32, c: i32) -> InputImage {
        InputImage {
            inner: unsafe {
                make_image(w, h, c)
            }
        }
    }

    pub fn data_mut(&mut self) -> *mut f32 {
        self.inner.data
    }
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
    fn darknet_detect(dn: *mut ffi::Darknet, image: ffi::image) -> ffi::Detections;
    fn detections_drop(dt: ffi::Detections);
    fn make_image(w: i32, h: i32, c: i32) -> ffi::image;
}
