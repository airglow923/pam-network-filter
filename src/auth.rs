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

    let iplist = match network::create_ip_list(parsed.ip_allow) {
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

    let network::IpList::V4(ipv4list) = iplist;

    if !ipv4list.ips.contains(rhost.to_bits()) {
        return pam::PAM_AUTH_ERR;
    }

    pam::PAM_SUCCESS
}
