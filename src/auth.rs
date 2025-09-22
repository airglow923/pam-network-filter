use crate::ffi::pam;
use crate::filter;
use crate::item;
use crate::log;
use crate::parser;

use libc;

use std::ffi::{c_char, c_int};

pub fn authenticate(
    pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let parsed = match parser::process_pam_args(argc, argv) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, libc::LOG_ERR, &e.to_string());
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    let connection = match item::get_pam_connection(pamh) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, libc::LOG_ERR, &e);
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    let filter_user_allow = match filter::filter_from_users(parsed.user_allow) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, libc::LOG_ERR, &e.to_string());
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    if !filter_user_allow.contains(&connection.user) {
        return pam::PAM_AUTH_ERR;
    }

    let filter_ipv4_allow = match filter::filter_from_ips(parsed.ip_allow) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, libc::LOG_ERR, &e.to_string());
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    if !filter_ipv4_allow.contains(&connection.rhost) {
        return pam::PAM_AUTH_ERR;
    }

    pam::PAM_SUCCESS
}
