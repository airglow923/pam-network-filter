#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/pam.rs"));

use std::ffi::{c_char, c_int};

use crate::ffi::types::argv_t;

pub type pamh_t = *mut pam_handle_t;

type PamFunction =
    unsafe extern "C" fn(pamh: pamh_t, flags: c_int, argc: c_int, argv: argv_t) -> c_int;

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
