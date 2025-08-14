mod pam;

use std::ffi::c_char;
use std::ffi::c_int;

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_authenticate(
    _pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_setcred(
    _pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    0
}
