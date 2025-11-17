use libc;

use std::ffi::{c_char, c_int};

pub struct AddrinfoBuilder {
    ai_flags: c_int,
    ai_family: c_int,
    ai_socktype: c_int,
    ai_protocol: c_int,
    ai_addrlen: libc::socklen_t,
    ai_addr: *mut libc::sockaddr,
    ai_canonname: *mut c_char,
    ai_next: *mut libc::addrinfo,
}

impl AddrinfoBuilder {
    pub const fn new() -> Self {
        Self {
            ai_flags: libc::AI_V4MAPPED | libc::AI_ADDRCONFIG,
            ai_family: libc::AF_UNSPEC,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: std::ptr::null_mut(),
            ai_canonname: std::ptr::null_mut(),
            ai_next: std::ptr::null_mut(),
        }
    }

    pub fn flags(mut self, flags: c_int) -> AddrinfoBuilder {
        self.ai_flags = flags;
        self
    }

    pub fn family(mut self, family: c_int) -> AddrinfoBuilder {
        self.ai_family = family;
        self
    }

    pub fn socktype(mut self, socktype: c_int) -> AddrinfoBuilder {
        self.ai_socktype = socktype;
        self
    }

    pub fn protocol(mut self, protocol: c_int) -> AddrinfoBuilder {
        self.ai_protocol = protocol;
        self
    }

    pub fn addrlen(mut self, addrlen: libc::socklen_t) -> AddrinfoBuilder {
        self.ai_addrlen = addrlen;
        self
    }

    pub fn addr(mut self, addr: *mut libc::sockaddr) -> AddrinfoBuilder {
        self.ai_addr = addr;
        self
    }

    pub fn canonname(mut self, cannoname: *mut c_char) -> AddrinfoBuilder {
        self.ai_canonname = cannoname;
        self
    }

    pub fn next(mut self, next: *mut libc::addrinfo) -> AddrinfoBuilder {
        self.ai_next = next;
        self
    }

    pub fn build(self) -> libc::addrinfo {
        libc::addrinfo {
            ai_flags: self.ai_flags,
            ai_family: self.ai_family,
            ai_socktype: self.ai_socktype,
            ai_protocol: self.ai_protocol,
            ai_addrlen: self.ai_addrlen,
            ai_addr: self.ai_addr,
            ai_canonname: self.ai_canonname,
            ai_next: self.ai_next,
        }
    }
}
