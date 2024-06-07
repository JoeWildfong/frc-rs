use std::{
    error::Error,
    ffi::CStr,
    fmt::{Debug, Display},
};

/// An unknown wpihal error, formattable using `wpihal_sys::HAL_GetErrorMessage`.
/// Call sites should prefer to check for HAL errors and wrap them into more specific enums.
/// This type is provided for cases where enumerating all possible HAL errors is not feasible,
/// and can serve as a "something else" type.
pub struct HalError(i32);

impl HalError {
    pub fn new(status: i32) -> Self {
        Self(status)
    }
}

impl Debug for HalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HalError({}), message: {}", self.0, self)
    }
}

impl Display for HalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c_str = unsafe { CStr::from_ptr(wpihal_sys::HAL_GetErrorMessage(self.0)) };
        write!(f, "{}", c_str.to_str().unwrap())
    }
}

impl Error for HalError {}
