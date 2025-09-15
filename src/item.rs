use crate::c_utils;
use crate::ffi::pam;
use crate::ffi::syslog;
use crate::log;

use c_utils::parse_c_string;
use std::ffi::{c_char, c_int, c_void};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Connection {
    pub service: String,
    pub user: String,
    pub ruser: String,
    pub rhost: String,
}

fn pam_get_err_msg(val: c_int) -> String {
    return match val {
        pam::PAM_BAD_ITEM => "undefined or inaccessible item".to_owned(),
        pam::PAM_PERM_DENIED => "third argument of pam_get_item is NULL".to_owned(),
        pam::PAM_SYSTEM_ERR => "wrong PAM handle".to_owned(),
        _ => "unknown error".to_owned(),
    };
}

fn pam_item_type_to_string(item_type: c_int) -> String {
    match item_type {
        pam::PAM_SERVICE => "service".to_owned(),
        pam::PAM_USER => "user".to_owned(),
        pam::PAM_TTY => "tty".to_owned(),
        pam::PAM_RHOST => "rhost".to_owned(),
        pam::PAM_CONV => "conv".to_owned(),
        pam::PAM_AUTHTOK => "authtok".to_owned(),
        pam::PAM_OLDAUTHTOK => "oldauthtok".to_owned(),
        pam::PAM_RUSER => "ruser".to_owned(),
        pam::PAM_USER_PROMPT => "user prompt".to_owned(),
        pam::PAM_FAIL_DELAY => "fail delay".to_owned(),
        pam::PAM_XDISPLAY => "xdisplay".to_owned(),
        pam::PAM_XAUTHDATA => "xauthdata".to_owned(),
        pam::PAM_AUTHTOK_TYPE => "authtok type".to_owned(),
        _ => "unknown".to_owned(),
    }
}

fn pam_item_log_err_and_throw(
    pamh: *const pam::pam_handle_t,
    val: c_int,
    item_type: c_int,
) -> Result<(), String> {
    let msg = format!(
        "item type: '{}', {}",
        pam_item_type_to_string(item_type),
        pam_get_err_msg(val)
    );

    match val {
        pam::PAM_SUCCESS => return Ok(()),
        pam::PAM_SYSTEM_ERR => log::syslog(syslog::LOG_ERR, format!("PAM {}", msg).as_str()),
        _ => log::pam_syslog(pamh, syslog::LOG_ERR, &msg),
    }

    Err(msg)
}

pub fn get_pam_connection(pamh: *const pam::pam_handle_t) -> Result<Connection, String> {
    if pamh.is_null() {
        return Err("null pamh passed".to_string());
    }

    let mut item: *const c_void = std::ptr::null();

    let ret = unsafe { pam::pam_get_item(pamh, pam::PAM_SERVICE, &mut item) };
    pam_item_log_err_and_throw(pamh, ret, pam::PAM_SERVICE)?;
    let service = parse_c_string(item as *const c_char);

    let ret = unsafe { pam::pam_get_item(pamh, pam::PAM_USER, &mut item) };
    pam_item_log_err_and_throw(pamh, ret, pam::PAM_USER)?;
    let user = parse_c_string(item as *const c_char);

    let ret = unsafe { pam::pam_get_item(pamh, pam::PAM_RUSER, &mut item) };
    pam_item_log_err_and_throw(pamh, ret, pam::PAM_RUSER)?;

    let ruser = if item.is_null() {
        String::new()
    } else {
        parse_c_string(item as *const c_char)
    };

    let ret = unsafe { pam::pam_get_item(pamh, pam::PAM_RHOST, &mut item) };
    pam_item_log_err_and_throw(pamh, ret, pam::PAM_RHOST)?;

    let rhost = if item.is_null() {
        String::new()
    } else {
        parse_c_string(item as *const c_char)
    };

    Ok(Connection {
        service,
        user,
        ruser,
        rhost,
    })
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    #[test]
    fn test_get_pam_connection_tp() {
        let conv: pam::pam_conv = pam::pam_conv::default();
        let mut pamh: *mut pam::pam_handle_t = std::ptr::null_mut();

        assert_eq!(pamh, std::ptr::null_mut());

        let ret = unsafe {
            pam::pam_start(
                config::PAM_MODULE_NAME.as_ptr(),
                c"doe".as_ptr(),
                &conv,
                &mut pamh,
            )
        };

        assert_eq!(ret, pam::PAM_SUCCESS);
        assert_ne!(pamh, std::ptr::null_mut());

        let ret = get_pam_connection(pamh);
        assert!(ret.is_ok());

        let connection = ret.unwrap();

        assert_eq!(
            connection.service,
            config::PAM_MODULE_NAME.to_string_lossy().into_owned()
        );
        assert_eq!(connection.user, "doe");
        assert_eq!(connection.ruser, "");
        assert_eq!(connection.rhost, "");

        let ret = unsafe { pam::pam_end(pamh, pam::PAM_SUCCESS) };

        assert_eq!(ret, pam::PAM_SUCCESS);
    }

    #[test]
    fn test_get_pam_connection_tn_null() {
        let pamh: *mut pam::pam_handle_t = std::ptr::null_mut();
        let ret = get_pam_connection(pamh);
        assert!(ret.is_err());
    }
}
