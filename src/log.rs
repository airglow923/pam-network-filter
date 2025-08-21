use crate::ffi::pam;
use crate::ffi::syslog;

pub fn log_unimplemented_pam_function(pamh: *mut pam::pam_handle_t, name: &str) {
    unsafe {
        pam::pam_syslog(
            pamh,
            syslog::LOG_INFO,
            format!("feature '{}' not implemented", name).as_ptr() as *const i8,
            name,
        );
    }
}
