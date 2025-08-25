use crate::c_utils;
use crate::ffi::pam;
use crate::ffi::syslog;
use crate::log;

use std::ffi::{c_char, c_int, c_void};

pub struct Connection {
    service: String,
    user: String,
    ruser: String,
    rhost: String,
}

fn pam_get_err_msg(val: c_int) -> String {
    return match val {
        pam::PAM_BAD_ITEM => "undefined or inaccessible item".to_owned(),
        pam::PAM_PERM_DENIED => "third argument of pam_get_item is NULL".to_owned(),
        pam::PAM_SYSTEM_ERR => "wrong PAM handle".to_owned(),
        _ => "".to_owned(),
    };
}

fn pam_get_item_log_err_and_throw(pamh: *mut pam::pam_handle_t, val: c_int) -> Result<(), String> {
    let msg = pam_get_err_msg(val);

    match val {
        pam::PAM_SUCCESS => return Ok(()),
        pam::PAM_SYSTEM_ERR => log::syslog(syslog::LOG_ERR, format!("PAM {}", msg).as_str()),
        _ => log::pam_syslog(pamh, syslog::LOG_ERR, &msg),
    }

    Err(msg)
}

pub fn get_pam_connection(pamh: *mut pam::pam_handle_t) -> Result<Connection, String> {
    let mut item: *const c_void = std::ptr::null();
    let mut ret;

    unsafe {
        ret = pam::pam_get_item(pamh, pam::PAM_SERVICE, &mut item);
    }

    pam_get_item_log_err_and_throw(pamh, ret)?;
    let service = item as *const c_char;

    unsafe {
        ret = pam::pam_get_item(pamh, pam::PAM_USER, &mut item);
    }

    pam_get_item_log_err_and_throw(pamh, ret)?;
    let user = item as *const c_char;

    unsafe {
        ret = pam::pam_get_item(pamh, pam::PAM_RUSER, &mut item);
    }

    pam_get_item_log_err_and_throw(pamh, ret)?;
    let ruser = item as *const c_char;

    unsafe {
        ret = pam::pam_get_item(pamh, pam::PAM_RHOST, &mut item);
    }

    pam_get_item_log_err_and_throw(pamh, ret)?;
    let rhost = item as *const c_char;

    Ok(Connection {
        service: c_utils::parse_c_string(service),
        user: c_utils::parse_c_string(user),
        ruser: c_utils::parse_c_string(ruser),
        rhost: c_utils::parse_c_string(rhost),
    })
}
