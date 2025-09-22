use crate::ffi::pam;
use crate::filter;
use crate::item;
use crate::log;
use crate::parser;

use libc;

use std::ffi::{c_char, c_int};

macro_rules! pam_syslog_on_err {
    ($e: expr, $pamh: expr $(,)?) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                log::pam_syslog($pamh, libc::LOG_ERR, &e.to_string());
                return pam::PAM_AUTHINFO_UNAVAIL;
            }
        }
    };
}

macro_rules! err_if_not_contains {
    ($haystack: expr, $needle: expr $(,)?) => {
        if !$haystack.contains($needle) {
            return pam::PAM_AUTH_ERR;
        }
    };
}

pub fn authenticate(
    pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let parsed = pam_syslog_on_err!(parser::process_pam_args(argc, argv), pamh);
    let connection = pam_syslog_on_err!(item::get_pam_connection(pamh), pamh);
    let filter_user_allow = pam_syslog_on_err!(filter::filter_from_users(parsed.user_allow), pamh);
    let filter_ipv4_allow = pam_syslog_on_err!(filter::filter_from_ips(parsed.ip_allow), pamh);

    err_if_not_contains!(filter_user_allow, &connection.user);
    err_if_not_contains!(filter_ipv4_allow, &connection.rhost);

    pam::PAM_SUCCESS
}
