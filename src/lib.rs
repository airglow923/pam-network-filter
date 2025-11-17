#[macro_use]
mod type_traits;
#[macro_use]
mod assert;
#[macro_use]
mod error;
mod auth;
mod c_utils;
mod config;
mod ffi;
mod filter;
mod item;
mod log;
mod network;
mod parser;

use ffi::pam;

use std::ffi::{c_char, c_int};

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_authenticate(
    pamh: *mut pam::pam_handle_t,
    flags: c_int,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    auth::authenticate(pamh, flags, argc, argv)
}

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_setcred(
    _pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    pam::PAM_SUCCESS
}

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_acct_mgmt(
    pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    log::log_unimplemented_pam_function(pamh, "pam_sm_acct_mgmt");
    pam::PAM_IGNORE
}

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_open_session(
    pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    log::log_unimplemented_pam_function(pamh, "pam_sm_open_session");
    pam::PAM_IGNORE
}

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_close_session(
    pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    log::log_unimplemented_pam_function(pamh, "pam_sm_close_session");
    pam::PAM_IGNORE
}

#[unsafe(no_mangle)]
pub extern "C" fn pam_sm_chauthtok(
    pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    log::log_unimplemented_pam_function(pamh, "pam_sm_chauthtok");
    pam::PAM_IGNORE
}

// PAM no longer supports static libraries
// https://github.com/linux-pam/linux-pam/commit/a684595c0bbd88df71285f43fb27630e3829121e
#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut _pam_listfile_modstruct: pam::pam_module = pam::pam_module {
    name: config::PAM_MODULE_NAME.as_ptr(),
    pam_sm_authenticate: Some(pam_sm_authenticate),
    pam_sm_setcred: Some(pam_sm_setcred),
    pam_sm_acct_mgmt: Some(pam_sm_acct_mgmt),
    pam_sm_open_session: Some(pam_sm_open_session),
    pam_sm_close_session: Some(pam_sm_close_session),
    pam_sm_chauthtok: Some(pam_sm_chauthtok),
};
