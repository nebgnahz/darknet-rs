pub mod ffi;
use std::ffi::CStr;

#[repr(C)]
#[derive(Debug)]
pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Debug)]
struct Detection {
    rect: Rect,
    label: String,
    prob: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct Detections {
    num: usize,
    detections: Vec<Detection>,
    proc_time_in_ms: f32,
}

impl Detection {
    fn csv(&self) -> String {
        format!("{}, {}, {}, {}, {}, {}",
                self.label, self.prob, self.rect.x, self.rect.y, self.rect.w, self.rect.h)
    }
}

impl Detections {
    pub fn print_csv(&self) {
        for i in 0..self.num {
            println!("{}, {}", self.proc_time_in_ms, self.detections[i].csv());
        }
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
    pub fn new(data: &str, network_file: &str, weight_file: &str, label_file: &str) -> Self {
        let config = ffi::DarknetConfig {
            datacfg: data.as_ptr(),
            network_file: network_file.as_ptr(),
            weight_file: weight_file.as_ptr(),
            label_file: label_file.as_ptr(),
        };

        Darknet {
            inner: unsafe {
                darknet_new(config)
            },
        }
    }

    pub fn detect(&mut self, image: InputImage) -> Detections {
        let c_detections = unsafe { darknet_detect(self.inner, image.inner) };
        let num = c_detections.num;
        let mut detections = Vec::with_capacity(num as usize);
        for i in 0..(num as isize) {
            let label = unsafe {
                CStr::from_ptr(*c_detections.labels.offset(i)).to_string_lossy().into_owned()
            };
            let d = unsafe {
                Detection {
                    label: label,
                    prob:  *(c_detections.probs.offset(i)),
                    rect: Rect {
                        x: (*c_detections.rects.offset(i)).x,
                        y: (*c_detections.rects.offset(i)).y,
                        w: (*c_detections.rects.offset(i)).w,
                        h: (*c_detections.rects.offset(i)).h,
                    },
                }
            };
            detections.push(d);
        }
        unsafe { detections_drop(c_detections) }
        Detections {
            num: num as usize,
            detections: detections,
            proc_time_in_ms: c_detections.proc_time_in_ms,
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

extern "C" {
    fn darknet_new(config: ffi::DarknetConfig) -> *mut ffi::Darknet;
    fn darknet_drop(dn: *mut ffi::Darknet);
    fn darknet_detect(dn: *mut ffi::Darknet, image: ffi::image) -> ffi::Detections;
    fn detections_drop(dt: ffi::Detections);
    fn make_image(w: i32, h: i32, c: i32) -> ffi::image;
}
