pub mod ffi;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};
    
fn c_string_to_rust_string(c_string: *const c_char) -> String {
    unsafe {
        CStr::from_ptr(c_string).to_string_lossy().into_owned()
    }
}

pub struct List {
    inner: *mut ffi::list,
}

impl List {
    pub fn read_data_cfg(filename: &str) -> List {
        let c_filename = CString::new(filename).unwrap().into_raw();
        List {
            inner: unsafe { ffi::read_data_cfg(c_filename) }
        }
    }

    pub fn find_str(&self, key: &str, default: &str) -> String {
        let c_char = unsafe {
            ffi::option_find_str(self.inner,
                                 CString::new(key).unwrap().into_raw(),
                                 CString::new(default).unwrap().into_raw())
        };
        c_string_to_rust_string(c_char)
    }
}

#[repr(C)]
pub struct Box {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

pub fn detect(_img: *const f32, _size: usize) -> Vec<Box> {
    Vec::new()
}
    
