#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]

use std::{ffi::CStr, mem::MaybeUninit};
include!("bindings.rs");

pub unsafe fn panic_on_hal_error<F, T>(f: F) -> T
where
    F: FnOnce(*mut i32) -> T,
{
    let mut status = MaybeUninit::uninit();
    let out = f(status.as_mut_ptr());
    let status = unsafe { status.assume_init() };
    if status != 0 {
        let error = unsafe { CStr::from_ptr(HAL_GetErrorMessage(status)) };
        panic!("{}", error.to_string_lossy());
    }
    out
}
