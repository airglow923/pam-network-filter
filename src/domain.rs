use crate::addrinfo_builder;
use crate::c_utils;

use libc;

use std::ffi::{c_char, c_int};
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use addrinfo_builder::AddrinfoBuilder;

const NI_MAXHOST_USIZE: usize = libc::NI_MAXHOST as usize;

#[allow(non_camel_case_types)]
pub enum AiFamily {
    AF_INET,
    AF_INET6,
    AF_UNSPEC,
}

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

pub fn get_domain_from_ip(ip: IpAddr) -> Result<Vec<String>, String> {
    // let ip = match ip {
    //     IpAddr::V4 => Ipv4Addr(ip),
    //     IpAddr::V6 => Ipv6Addr(ip),
    // };

    // int getnameinfo(socklen_t hostlen, socklen_t servlen;
    //                   const struct sockaddr *restrict addr, socklen_t addrlen,
    //                   char host[_Nullable restrict hostlen],
    //                   socklen_t hostlen,
    //                   char serv[_Nullable restrict servlen],
    //                   socklen_t servlen,
    //                   int flags);

    // libc::getnameinfo(sa, salen, host, hostlen, serv, servlen, flags)

    Ok(Vec::new())
}

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

    let mut res: *mut libc::addrinfo = std::ptr::null_mut();

    let ret = unsafe { libc::getaddrinfo(node, std::ptr::null(), &hints, &mut res) };

    if ret != 0 {
        return Err(eai_get_err_msg(ret));
    }

    let mut p = res;
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

    unsafe {
        libc::freeaddrinfo(res);
    }

    Ok(lookup)
}
