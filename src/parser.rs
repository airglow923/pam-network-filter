use crate::config;
use clap::{Parser, error::Error, error::ErrorKind};
use std::ffi::{CStr, c_char, c_int};

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
struct Cli {
    #[arg(long)]
    ip_allow: Vec<String>,

    #[arg(long)]
    ip_deny: Vec<String>,

    #[arg(long)]
    mac_allow: Vec<String>,

    #[arg(long)]
    mac_deny: Vec<String>,

    #[arg(long)]
    port_allow: Vec<String>,

    #[arg(long)]
    port_deny: Vec<String>,

    #[arg(long)]
    name_allow: Vec<String>,

    #[arg(long)]
    name_deny: Vec<String>,
}

fn parse_c_string(s: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(s) };
    cstr.to_string_lossy().into_owned()
}

fn parse_c_args(argc: c_int, argv: *const *const c_char) -> Vec<String> {
    let len = argc as usize;
    let mut ptr = argv;
    let end = ptr.wrapping_add(len);
    // extra one space for module name
    let mut vec = Vec::with_capacity(len + 1);

    vec.push(config::PAM_MODULE_LIB.to_string());

    while ptr != end {
        unsafe {
            vec.push(parse_c_string(*ptr));
        }
        ptr = ptr.wrapping_add(1);
    }

    vec
}

fn process_pam_args(argc: c_int, argv: *const *const c_char) -> Result<Cli, Error> {
    if argc == 0 {
        return Err(Error::raw(
            ErrorKind::MissingRequiredArgument,
            "no arguments provided",
        ));
    }

    let args = parse_c_args(argc, argv);
    let cli = Cli::try_parse_from(args)?;

    Ok(cli)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pam_args_invalid() {
        let argv = [
            c"asdf".as_ptr(),
            c"qwer".as_ptr(),
            c"the".as_ptr(),
            c"qerqwe".as_ptr(),
        ];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err().kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_process_pam_args_empty() {
        let argv = [];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_process_pam_args_flag_without_hyphens() {
        let argv = [c"ip-allow".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err().kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_process_pam_args_ip_allow_value_empty() {
        let argv = [c"--ip-allow".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_process_pam_args_ip_allow_value_string() {
        let argv = [c"--ip-allow".as_ptr(), c"asdf".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().ip_allow[0].as_str(), "asdf");
    }
}
