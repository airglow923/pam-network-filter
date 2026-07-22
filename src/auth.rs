use std::ffi::c_int;

use libc;

use crate::ffi::{pam, types};
use crate::filter;
use crate::item;
use crate::log;
use crate::parser;
use crate::pattern;

use libc::{LOG_ERR, LOG_INFO};

use filter::{Filter, FilterDomain, FilterIp, FilterUser};
use log::pam_syslog;
use pam::pamh_t;
use pam::{PAM_AUTH_ERR, PAM_AUTHINFO_UNAVAIL, PAM_SUCCESS};
use pattern::pat_ipv4;
use types::argv_t;

macro_rules! pam_syslog_on_err {
    ($e: expr, $pamh: expr $(,)?) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                pam_syslog($pamh, LOG_ERR, &e.to_string());
                return PAM_AUTHINFO_UNAVAIL;
            }
        }
    };
}

fn auth_user(allowed_users: &FilterUser, user: &str, pamh: pamh_t) -> c_int {
    // allow all users if rules not set
    if allowed_users.is_empty() || allowed_users.contains(user) {
        let msg = format!("user '{}' allowed", user);
        pam_syslog(pamh, LOG_INFO, &msg);
        return PAM_SUCCESS;
    }

    let msg = format!("user '{}' not allowed", user);
    pam_syslog(pamh, LOG_ERR, &msg);
    PAM_AUTH_ERR
}

fn auth_rhost(
    allowed_ips: &FilterIp,
    allowed_domains: &FilterDomain,
    rhost: &str,
    pamh: pamh_t,
) -> c_int {
    let msg_allow = format!("host '{}' allowed", rhost);
    let msg_deny = format!("host '{}' not allowed", rhost);

    let is_ip_set = !allowed_ips.is_empty();
    let is_domain_set = !allowed_domains.is_empty();

    if !is_ip_set && !is_domain_set {
        pam_syslog(pamh, LOG_INFO, &msg_allow);
        return PAM_SUCCESS;
    }

    // not sure why fancy_regex returns Result while std doesn't
    match pat_ipv4().is_match(rhost).unwrap_or(false) {
        // I wish Rust has goto
        true => {
            if !is_ip_set {
                // if IP is provided but only domain rules set
                if is_domain_set {
                    pam_syslog(pamh, LOG_ERR, &msg_deny);
                    return PAM_AUTH_ERR;
                }
            } else if !allowed_ips.contains(rhost) {
                pam_syslog(pamh, LOG_ERR, &msg_deny);
                return PAM_AUTH_ERR;
            }
        }
        false => {
            if !is_domain_set {
                // if domain is provided but only IP rules set
                // do not perform reverse DNS lookup and deny immediately
                if is_ip_set {
                    pam_syslog(pamh, LOG_ERR, &msg_deny);
                    return PAM_AUTH_ERR;
                }
            } else if !allowed_domains.contains(rhost) {
                pam_syslog(pamh, LOG_ERR, &msg_deny);
                return PAM_AUTH_ERR;
            }
        }
    }

    pam_syslog(pamh, LOG_INFO, &msg_allow);
    PAM_SUCCESS
}

pub fn authenticate(pamh: pamh_t, _flags: c_int, argc: c_int, argv: argv_t) -> c_int {
    let parsed = pam_syslog_on_err!(parser::process_pam_args(argc, argv), pamh);
    let conn = pam_syslog_on_err!(item::get_pam_connection(pamh), pamh);

    let filter_user_allow = pam_syslog_on_err!(filter::filter_from_users(parsed.user_allow), pamh);
    let filter_ipv4_allow = pam_syslog_on_err!(filter::filter_from_ips(parsed.ip_allow), pamh);
    let filter_domain_allow =
        pam_syslog_on_err!(filter::filter_from_domains(parsed.domain_allow), pamh);

    #[allow(unused_variables)]
    let item::Connection {
        user,
        service,
        ruser,
        rhost,
    } = &conn;

    if auth_user(&filter_user_allow, user, pamh) != PAM_SUCCESS {
        return PAM_AUTH_ERR;
    }

    if auth_rhost(&filter_ipv4_allow, &filter_domain_allow, rhost, pamh) != PAM_SUCCESS {
        return PAM_AUTH_ERR;
    }

    let msg = format!("'{}@{}' authentication succeeded", user, rhost);
    pam_syslog(pamh, LOG_INFO, &msg);
    PAM_SUCCESS
}
