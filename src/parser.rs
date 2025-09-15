use crate::c_utils;
use crate::config;

use clap::{Parser, error::Error, error::ErrorKind};
use std::ffi::{c_char, c_int};

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
pub struct Cli {
    #[clap(long, value_delimiter(','))]
    pub ip_allow: Vec<String>,

    #[clap(long, value_delimiter(','))]
    pub ip_deny: Vec<String>,

    #[clap(long, value_delimiter(','))]
    pub mac_allow: Vec<String>,

    #[clap(long, value_delimiter(','))]
    pub mac_deny: Vec<String>,

    #[clap(long, value_delimiter(','))]
    pub port_allow: Vec<String>,

    #[clap(long, value_delimiter(','))]
    pub port_deny: Vec<String>,

    #[clap(long, value_delimiter(','))]
    pub name_allow: Vec<String>,

    #[clap(long, value_delimiter(','))]
    pub name_deny: Vec<String>,
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
            vec.push(c_utils::parse_c_string(*ptr));
        }
        ptr = ptr.wrapping_add(1);
    }

    vec
}

pub fn process_pam_args(argc: c_int, argv: *const *const c_char) -> Result<Cli, Error> {
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
    fn test_process_pam_args_tn_invalid() {
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
    fn test_process_pam_args_tn_empty() {
        let argv = [];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_process_pam_args_tn_flag_without_hyphens() {
        let argv = [c"ip-allow".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err().kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_process_pam_args_tn_ip_allow_value_empty() {
        let argv = [c"--ip-allow".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());

        assert!(ret.is_err());
        assert_eq!(ret.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_process_pam_args_tp_ip_allow_value_string() {
        let argv = [c"--ip-allow".as_ptr(), c"asdf".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());
        assert!(ret.is_ok());

        let cli = ret.unwrap();

        assert_eq!(cli.ip_allow.len(), 1);
        assert_eq!(cli.ip_allow[0].as_str(), "asdf");
        assert_eq!(cli.ip_deny.len(), 0);
        assert_eq!(cli.mac_allow.len(), 0);
        assert_eq!(cli.mac_deny.len(), 0);
        assert_eq!(cli.port_allow.len(), 0);
        assert_eq!(cli.port_deny.len(), 0);
        assert_eq!(cli.name_allow.len(), 0);
        assert_eq!(cli.name_deny.len(), 0);
    }

    #[test]
    fn test_process_pam_args_tp_ip_allow_value_string_with_equal() {
        let argv = [c"--ip-allow=asdf".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());
        assert!(ret.is_ok());

        let cli = ret.unwrap();

        assert_eq!(cli.ip_allow.len(), 1);
        assert_eq!(cli.ip_allow[0].as_str(), "asdf");
        assert_eq!(cli.ip_deny.len(), 0);
        assert_eq!(cli.mac_allow.len(), 0);
        assert_eq!(cli.mac_deny.len(), 0);
        assert_eq!(cli.port_allow.len(), 0);
        assert_eq!(cli.port_deny.len(), 0);
        assert_eq!(cli.name_allow.len(), 0);
        assert_eq!(cli.name_deny.len(), 0);
    }

    #[test]
    fn test_process_pam_args_tp_ip_allow_value_comma_separated_string() {
        let argv = [c"--ip-allow".as_ptr(), c"asdf,qwer".as_ptr()];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());
        assert!(ret.is_ok());

        let cli = ret.unwrap();
        let ip_allow = cli.ip_allow;

        assert_eq!(ip_allow.len(), 2);
        assert_eq!(ip_allow[0].as_str(), "asdf");
        assert_eq!(ip_allow[1].as_str(), "qwer");
        assert_eq!(cli.ip_deny.len(), 0);
        assert_eq!(cli.mac_allow.len(), 0);
        assert_eq!(cli.mac_deny.len(), 0);
        assert_eq!(cli.port_allow.len(), 0);
        assert_eq!(cli.port_deny.len(), 0);
        assert_eq!(cli.name_allow.len(), 0);
        assert_eq!(cli.name_deny.len(), 0);
        assert_eq!(ip_allow.len(), 2);
    }

    #[test]
    fn test_process_pam_args_tp_ip_allow_multiple_values() {
        let argv = [
            c"--ip-allow".as_ptr(),
            c"asdf".as_ptr(),
            c"--ip-allow".as_ptr(),
            c"qwer".as_ptr(),
        ];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());
        assert!(ret.is_ok());

        let cli = ret.unwrap();
        let ip_allow = cli.ip_allow;

        assert_eq!(ip_allow.len(), 2);
        assert_eq!(ip_allow[0].as_str(), "asdf");
        assert_eq!(ip_allow[1].as_str(), "qwer");
        assert_eq!(cli.ip_deny.len(), 0);
        assert_eq!(cli.mac_allow.len(), 0);
        assert_eq!(cli.mac_deny.len(), 0);
        assert_eq!(cli.port_allow.len(), 0);
        assert_eq!(cli.port_deny.len(), 0);
        assert_eq!(cli.name_allow.len(), 0);
        assert_eq!(cli.name_deny.len(), 0);
    }

    #[test]
    fn test_process_pam_args_tp_ip_allow_multiple_values_mixed_comma_separated() {
        let argv = [
            c"--ip-allow".as_ptr(),
            c"asdf,qwer".as_ptr(),
            c"--ip-allow".as_ptr(),
            c"qwer".as_ptr(),
        ];

        let ret = process_pam_args(argv.len() as c_int, argv.as_ptr());
        assert!(ret.is_ok());

        let cli = ret.unwrap();
        let ip_allow = cli.ip_allow;

        assert_eq!(ip_allow.len(), 3);
        assert_eq!(ip_allow[0].as_str(), "asdf");
        assert_eq!(ip_allow[1].as_str(), "qwer");
        assert_eq!(ip_allow[2].as_str(), "qwer");
        assert_eq!(cli.ip_deny.len(), 0);
        assert_eq!(cli.mac_allow.len(), 0);
        assert_eq!(cli.mac_deny.len(), 0);
        assert_eq!(cli.port_allow.len(), 0);
        assert_eq!(cli.port_deny.len(), 0);
        assert_eq!(cli.name_allow.len(), 0);
        assert_eq!(cli.name_deny.len(), 0);
    }
}
