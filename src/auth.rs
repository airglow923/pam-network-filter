use crate::ffi::{pam, syslog};
use crate::item;
use crate::log;
use crate::network;
use crate::parser;

use std::ffi::{c_char, c_int};
use std::net::Ipv4Addr;

pub fn authenticate(
    pamh: *mut pam::pam_handle_t,
    _flags: c_int,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let parsed = match parser::process_pam_args(argc, argv) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, syslog::LOG_ERR, &e.to_string());
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    let connection = match item::get_pam_connection(pamh) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, syslog::LOG_ERR, &e);
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    let ip_allowlist = match network::create_list_ipv4(parsed.ip_allow) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, syslog::LOG_ERR, &e.to_string());
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    let ip_denylist = match network::create_list_ipv4(parsed.ip_deny) {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, syslog::LOG_ERR, &e.to_string());
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    let rhost = match connection.rhost.parse::<Ipv4Addr>() {
        Ok(x) => x,
        Err(e) => {
            log::pam_syslog(pamh, syslog::LOG_ERR, &e.to_string());
            return pam::PAM_AUTHINFO_UNAVAIL;
        }
    };

    let network::IpList::V4(ipv4_allowlist) = ip_allowlist;
    let network::IpList::V4(ipv4_denylist) = ip_denylist;

    if ipv4_denylist.ips.contains(rhost.to_bits()) {
        return pam::PAM_AUTH_ERR;
    }

    if !ipv4_allowlist.ips.contains(rhost.to_bits()) {
        return pam::PAM_AUTH_ERR;
    }

    pam::PAM_SUCCESS
}
