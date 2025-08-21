#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/pam.rs"));

use std::ffi::{c_char, c_int};

type PamFunction = unsafe extern "C" fn(
    pamh: *mut pam_handle_t,
    flags: c_int,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int;

#[repr(C)]
pub struct pam_module {
    pub name: *const c_char,
    pub pam_sm_authenticate: Option<PamFunction>,
    pub pam_sm_setcred: Option<PamFunction>,
    pub pam_sm_acct_mgmt: Option<PamFunction>,
    pub pam_sm_open_session: Option<PamFunction>,
    pub pam_sm_close_session: Option<PamFunction>,
    pub pam_sm_chauthtok: Option<PamFunction>,
}
