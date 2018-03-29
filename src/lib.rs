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

mod ffi {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

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

    /// Perform prediction
    pub fn predict(&mut self, data: *mut f32) -> *mut f32 {
        unsafe { ffi::network_predict(self.net, data) }
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        unsafe {
            ffi::free_network(self.net);
        }
    }
}
