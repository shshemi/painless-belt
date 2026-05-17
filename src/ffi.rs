use core::ffi::c_char;
use std::ffi::CStr;
use std::ptr;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    #[error("sandbox_init failed: {0}")]
    Sandbox(String),
    #[error("profile contained an interior NUL byte")]
    InvalidProfile,
    #[error("parameter {0:?} contained an interior NUL byte")]
    InvalidParameter(String),
}

pub fn sandbox_init(profile: &str, flags: u64) -> Result<(), Error> {
    let mut errorbuf: *mut c_char = ptr::null_mut();
    let rc = unsafe { c_api::sandbox_init(profile.as_ptr() as *const i8, flags, &mut errorbuf) };
    if rc == 0 {
        if !errorbuf.is_null() {
            unsafe { c_api::sandbox_free_error(errorbuf) };
        }
        return Ok(());
    }
    let msg = if errorbuf.is_null() {
        String::new()
    } else {
        let s = unsafe { CStr::from_ptr(errorbuf) }
            .to_string_lossy()
            .into_owned();
        unsafe { c_api::sandbox_free_error(errorbuf) };
        s
    };
    Err(Error::Sandbox(msg))
}

mod c_api {
    use std::ffi::{c_char, c_int};

    #[allow(non_upper_case_globals)]
    unsafe extern "C" {
        pub fn sandbox_init(
            profile: *const c_char,
            flags: u64,
            errorbuf: *mut *mut c_char,
        ) -> c_int;

        pub fn sandbox_free_error(errorbuf: *mut c_char);
    }
}
