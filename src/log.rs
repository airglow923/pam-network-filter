use crate::ffi::pam;
use crate::ffi::syslog;

use std::ffi::{c_char, c_int};

pub fn syslog(priority: c_int, msg: &str) {
    unsafe {
        syslog::syslog(priority, msg.as_ptr() as *const c_char);
    }
}

pub fn pam_syslog(pamh: *const pam::pam_handle_t, priority: c_int, msg: &str) {
    unsafe {
        pam::pam_syslog(pamh, priority, msg.as_ptr() as *const c_char);
    }
}

pub fn log_unimplemented_pam_function(pamh: *mut pam::pam_handle_t, name: &str) {
    pam_syslog(
        pamh,
        syslog::LOG_INFO,
        format!("feature '{}' not implemented", name).as_str(),
    );
}
