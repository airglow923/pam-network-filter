use std::ffi::{c_char, c_int};
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use libc;

use crate::c_utils;

use addrinfo_builder::AddrinfoBuilder;
use addrinfo_smart_pointer::AddrinfoSmartPointer;

mod addrinfo_builder;
mod addrinfo_smart_pointer;

#[allow(dead_code)]
const NI_MAXHOST_USIZE: usize = libc::NI_MAXHOST as usize;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum AiFamily {
    AF_INET,
    AF_INET6,
    AF_UNSPEC,
}

#[allow(dead_code)]
fn eai_get_err_msg(err: c_int) -> String {
    return match err {
        libc::EAI_BADFLAGS => "EAI_BADFLAGS: addrinfo.ai_flags contains invalid flags".to_owned(),
        libc::EAI_NONAME => "EAI_NONAME: wrong host or service provided as arguments".to_owned(),
        libc::EAI_AGAIN => "EAI_AGAIN: temporary failure in name resolution".to_owned(),
        libc::EAI_FAIL => "EAI_FAIL: permanent failure in name resolution".to_owned(),
        libc::EAI_NODATA => "EAI_NODATA: host exists but no network address".to_owned(),
        libc::EAI_FAMILY => "EAI_FAMILY: invalid address family".to_owned(),
        libc::EAI_SOCKTYPE => {
            "EAI_SOCKTYPE: invalid socket type or incompatible with protocol".to_owned()
        }
        libc::EAI_SERVICE => {
            "EAI_SERVICE: requested service incompatible with socket type".to_owned()
        }
        libc::EAI_MEMORY => "EAI_MEMORY: out of memory".to_owned(),
        libc::EAI_SYSTEM => format!("EAI_SYSTEM: other system error: {}", Error::last_os_error()),
        libc::EAI_OVERFLOW => "EAI_OVERFLOW: provided buffer too small".to_owned(),
        _ => format!("unknown error: {}", err),
    };
}

#[allow(dead_code)]
pub fn get_domain_from_ip(ip: IpAddr) -> Result<String, String> {
    let ip_nullterminated = format!("{}\0", ip.to_string());
    let node = ip_nullterminated.as_ptr() as *const c_char;
    let hints = AddrinfoBuilder::new().flags(libc::AI_NUMERICHOST).build();
    let mut res = AddrinfoSmartPointer::new();

    let ret = unsafe { libc::getaddrinfo(node, std::ptr::null(), &hints, &mut res.addrinfo) };

    if ret != 0 {
        return Err(eai_get_err_msg(ret));
    }

    let dret = unsafe { *res.addrinfo };
    let mut host: [c_char; NI_MAXHOST_USIZE] = [0; NI_MAXHOST_USIZE];

    let ret = unsafe {
        libc::getnameinfo(
            dret.ai_addr,
            dret.ai_addrlen,
            host.as_mut_ptr(),
            libc::NI_MAXHOST,
            std::ptr::null_mut(),
            0,
            libc::NI_NAMEREQD,
        )
    };

    if ret != 0 {
        return Err(eai_get_err_msg(ret));
    }

    Ok(c_utils::parse_c_string(host.as_ptr()))
}

#[allow(dead_code)]
pub fn get_ip_from_domain(domain: &str, ai_family: AiFamily) -> Result<Vec<IpAddr>, String> {
    let domain_nullterminated = format!("{}\0", domain);
    let node = domain_nullterminated.as_ptr() as *const c_char;
    let hints = AddrinfoBuilder::new()
        .flags(libc::AI_CANONNAME)
        .family(match ai_family {
            AiFamily::AF_INET => libc::AF_INET,
            AiFamily::AF_INET6 => libc::AF_INET6,
            AiFamily::AF_UNSPEC => libc::AF_UNSPEC,
        })
        .build();
    let mut res = AddrinfoSmartPointer::new();

    let ret = unsafe { libc::getaddrinfo(node, std::ptr::null(), &hints, &mut res.addrinfo) };

    if ret != 0 {
        return Err(eai_get_err_msg(ret));
    }

    let mut p = res.addrinfo;
    let mut lookup = Vec::new();

    while p != std::ptr::null_mut() {
        let dp = unsafe { *p };
        let mut host: [c_char; NI_MAXHOST_USIZE] = [0; NI_MAXHOST_USIZE];

        let ret = unsafe {
            libc::getnameinfo(
                dp.ai_addr,
                dp.ai_addrlen,
                host.as_mut_ptr(),
                libc::NI_MAXHOST,
                std::ptr::null_mut(),
                0,
                libc::NI_NUMERICHOST,
            )
        };

        if ret != 0 {
            return Err(eai_get_err_msg(ret));
        }

        let ip_str = c_utils::parse_c_string(host.as_ptr());
        let ip = match dp.ai_family {
            libc::AF_INET => IpAddr::V4(err_if_fail!(ip_str.parse::<Ipv4Addr>())),
            libc::AF_INET6 => IpAddr::V6(err_if_fail!(ip_str.parse::<Ipv6Addr>())),
            _ => return Err(format!("Unsupported address family: {}", dp.ai_family)),
        };

        lookup.push(ip);

        p = dp.ai_next;
    }

    Ok(lookup)
}
