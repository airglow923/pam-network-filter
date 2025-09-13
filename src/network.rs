extern crate ipnet;
extern crate roaring;

use ipnet::Ipv4Net;
use regex::Regex;
use roaring::RoaringBitmap;
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Pattern {
    Ipv4Addr,
    Ipv6Addr,
    Ipv4Net,
    Ipv6Net,
    Ipv4Range,
    Ipv6Range,
}

#[derive(Debug, Clone)]
pub enum IpAllowlist {
    V4(Ipv4Allowlist),
}

#[derive(Debug, Clone)]
pub struct Ipv4Allowlist {
    ips: RoaringBitmap,
    subnets: Vec<Ipv4Net>,
    ranges: Vec<(Ipv4Addr, Ipv4Addr)>,
}

fn find_ip_match(ip: &str) -> Result<Pattern, regex::Error> {
    let pattern_ipv4addr_str = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";
    let pattern_ranges = Regex::new(format!(r"^{p}-{p}$", p = pattern_ipv4addr_str).as_str())?;

    if pattern_ranges.is_match(ip) {
        return Ok(Pattern::Ipv4Range);
    }

    // \d       => 0-9
    // [1-2]?\d => 10-29
    // 3[0-2]   => 30-32
    let pattern_ipv4net =
        Regex::new(format!(r"^{}/(\d|[1-2]?\d|3[0-2])$", pattern_ipv4addr_str).as_str())?;

    if pattern_ipv4net.is_match(ip) {
        return Ok(Pattern::Ipv4Net);
    }

    let pattern_ipv4addr = Regex::new(format!("^{}$", pattern_ipv4addr_str).as_str())?;

    if pattern_ipv4addr.is_match(ip) {
        return Ok(Pattern::Ipv4Addr);
    }

    Err(regex::Error::Syntax("no matching pattern".to_owned()))
}

pub fn create_ip_allowlist(ip_list: Vec<String>) -> Result<IpAllowlist, Box<dyn Error>> {
    let mut ips = RoaringBitmap::new();
    let mut subnets = Vec::new();
    let mut ranges = Vec::new();

    for ip in ip_list {
        let pat = find_ip_match(&ip)?;

        let ret: Result<(), Box<dyn Error>> = match pat {
            Pattern::Ipv4Addr => {
                ips.insert(ip.parse::<Ipv4Addr>()?.to_bits());
                Ok(())
            }
            Pattern::Ipv4Net => {
                subnets.push(ip.parse::<Ipv4Net>()?);
                Ok(())
            }
            Pattern::Ipv4Range => {
                let range: Vec<_> = ip.split('-').collect();

                if let [lower, upper] = &range[0..1] {
                    ranges.push((lower.parse::<Ipv4Addr>()?, upper.parse::<Ipv4Addr>()?));
                } else {
                    return Err(Box::<dyn Error>::from("wrong input for IPv4 range syntax"));
                }

                Ok(())
            }
            _ => Err(Box::<dyn Error>::from("no matching pattern")),
        };

        if let Err(e) = ret {
            println!("asdfasdfasdfasdfasdfasdfasdfasdfasdfasdfok");
            return Err(e);
        }
    }

    Ok(IpAllowlist::V4(Ipv4Allowlist {
        ips,
        subnets,
        ranges,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_ip_match_tp_ipv4range() {
        let ret = find_ip_match("127.0.0.1-127.0.0.1");

        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), Pattern::Ipv4Range);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4range_preceded_by_invalid() {
        let ret = find_ip_match("1127.0.0.1-127.0.0.1");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), regex::Error);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4range_superceded_by_invalid() {
        let ret = find_ip_match("127.0.0.1-127.0.0.1a");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), regex::Error);
    }

    #[test]
    fn test_find_ip_match_tp_ipv4net() {
        let ret = find_ip_match("127.0.0.1/32");

        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), Pattern::Ipv4Net);
    }

    #[test]
    fn test_find_ip_match_tp_ipv4net_subnet_0to32() {
        for i in 0..=32 {
            let ret = find_ip_match(format!("127.0.0.1/{}", i.to_string()).as_str());

            assert!(ret.is_ok());
            assert_eq!(ret.unwrap(), Pattern::Ipv4Net);
        }
    }

    #[test]
    fn test_find_ip_match_tn_ipv4net_invalid_subnet() {
        let ret = find_ip_match("127.0.0.1/33");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), regex::Error);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4net_invalid_subnet_3digits() {
        let ret = find_ip_match("127.0.0.1/100");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), regex::Error);
    }

    #[test]
    fn test_find_ip_match_tp_ipv4addr() {
        let ret = find_ip_match("127.0.0.1");

        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), Pattern::Ipv4Addr);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4addr_preceded_by_invalid() {
        let ret = find_ip_match("1127.0.0.1");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), regex::Error);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4addr_superceded_by_invalid() {
        let ret = find_ip_match("127.0.0.1111");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), regex::Error);
    }

    // currently regex pattern does not check whether an IP octet is in 0-255
    // however, this is handled when parsed as Ipv4Addr
    // doing such check twice is unnecessary, so this behavior is expected
    #[test]
    fn test_find_ip_match_fp_ipv4addr_out_of_range() {
        let ret = find_ip_match("127.0.0.256");

        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), Pattern::Ipv4Addr);
    }

    #[test]
    fn test_create_ip_allowlist_tn_ipv4addr_out_of_range() {
        let ret = create_ip_allowlist(vec!["127.0.0.256".to_owned()]);

        assert!(ret.is_err());
    }
}
