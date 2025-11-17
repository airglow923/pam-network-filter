use libc;

pub struct AddrinfoSmartPointer {
    pub addrinfo: *mut libc::addrinfo,
}

#[allow(dead_code)]
impl AddrinfoSmartPointer {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            addrinfo: std::ptr::null_mut(),
        }
    }
}

impl Drop for AddrinfoSmartPointer {
    fn drop(&mut self) {
        unsafe {
            libc::freeaddrinfo(self.addrinfo);
        }
    }
}
