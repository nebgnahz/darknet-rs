pub mod ffi;
use std::ffi::CString;
    
pub struct List {
    list: *mut ffi::list,
}

impl List {
    pub fn from_data_cfg(filename: &str) -> List {
        let c_filename = CString::new(filename).unwrap().into_raw();
        List {
            list: unsafe { ffi::read_data_cfg(c_filename) }
        }
    }
}

