use std::ffi::{CStr, c_char};

pub fn parse_c_string(s: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(s) };
    cstr.to_string_lossy().into_owned()
}
