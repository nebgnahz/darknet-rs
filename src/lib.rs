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

pub fn test_detector(datacfg: &str, _cfgfile: &str, _weightfile: &str,
                     _thresh: f64, _hier_thresh: f64) {
    let options = List::read_data_cfg(datacfg);
    let _list = options.find_str("names", "data/names.list");
    
}
    
