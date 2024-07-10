use std::ffi::{c_int, CStr};
use std::process::exit;

use crate::git::raw::giterr_last;

pub mod raw;

pub fn check(activity: &'static str, status: c_int) -> c_int {
    if status < 0 {
        unsafe {
            let err = &*giterr_last();
            println!("error while {}: {} ({})", activity, CStr::from_ptr(err.message).to_string_lossy(), err.klass);
            exit(1);
        }
    }
    status
}