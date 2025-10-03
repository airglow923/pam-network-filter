extern crate ipnet;
extern crate roaring;

use fancy_regex::Regex;
use ipnet::Ipv4Net;
use roaring::RoaringBitmap;

use std::net::Ipv4Addr;

macro_rules! err_if_fail {
    ($e: expr $(,)?) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        }
    };
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Pattern {
    Ipv4Addr,
    Ipv6Addr,
    Ipv4Net,
    Ipv6Net,
    Ipv4Range,
    Ipv6Range,
    Domain,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum IpList {
    V4(Ipv4List),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ipv4List {
    pub ips: RoaringBitmap,
    pub subnets: Vec<Ipv4Net>,
    pub ranges: Vec<(Ipv4Addr, Ipv4Addr)>,
}

fn find_ip_match(ip: &str) -> Result<Pattern, String> {
    let pattern_ipv4addr_str = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";
    let pattern_ranges = err_if_fail!(Regex::new(
        format!(r"^{p}-{p}$", p = pattern_ipv4addr_str).as_str()
    ));

    if err_if_fail!(pattern_ranges.is_match(ip)) {
        return Ok(Pattern::Ipv4Range);
    }

    // \d       => 0-9
    // [1-2]?\d => 10-29
    // 3[0-2]   => 30-32
    let pattern_ipv4net = err_if_fail!(Regex::new(
        format!(r"^{}/(\d|[1-2]?\d|3[0-2])$", pattern_ipv4addr_str).as_str()
    ));

    if err_if_fail!(pattern_ipv4net.is_match(ip)) {
        return Ok(Pattern::Ipv4Net);
    }

    let pattern_ipv4addr = err_if_fail!(Regex::new(format!("^{}$", pattern_ipv4addr_str).as_str()));

    if err_if_fail!(pattern_ipv4addr.is_match(ip)) {
        return Ok(Pattern::Ipv4Addr);
    }

    Err(format!("'{}' no matching pattern", ip))
}

#[allow(dead_code)]
fn is_domain(domain: &str) -> Result<(), String> {
    let pattern_domain = err_if_fail!(Regex::new(
        r"^(?!:\/\/)(?=.{1,255}$)((.{1,63}\.){1,127}(?![0-9]*$)[\w-]+\.?)$"
    ));

    if !err_if_fail!(pattern_domain.is_match(domain)) {
        return Err(format!("'{}' wrong domain syntax", domain));
    }

    Ok(())
}

pub fn create_list_ipv4(ip_list: Vec<String>) -> Result<Ipv4List, String> {
    let mut ips = RoaringBitmap::new();
    let mut subnets = Vec::new();
    let mut ranges = Vec::new();

    for ip in ip_list {
        let pat = find_ip_match(&ip)?;

        let ret: Result<(), String> = match pat {
            Pattern::Ipv4Addr => {
                ips.insert(err_if_fail!(ip.parse::<Ipv4Addr>()).to_bits());
                Ok(())
            }
            Pattern::Ipv4Net => {
                subnets.push(err_if_fail!(ip.parse::<Ipv4Net>()));
                Ok(())
            }
            Pattern::Ipv4Range => {
                let range: Vec<_> = ip.split('-').collect();

                if let [lower, upper] = &range[0..2] {
                    if lower >= upper {
                        return Err(format!(
                            "'{}' IP on left side should be lower than the right one",
                            ip
                        ));
                    }
                    ranges.push((
                        err_if_fail!(lower.parse::<Ipv4Addr>()),
                        err_if_fail!(upper.parse::<Ipv4Addr>()),
                    ));
                } else {
                    return Err(format!("'{}' wrong input for IPv4 range syntax", ip));
                }

                Ok(())
            }
            _ => Err("no matching pattern".to_owned()),
        };

        if let Err(e) = ret {
            return Err(e);
        }
    }

    Ok(Ipv4List {
        ips,
        subnets,
        ranges,
    })
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
        assert_eq_type!(ret.unwrap_err(), String);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4range_superceded_by_invalid() {
        let ret = find_ip_match("127.0.0.1-127.0.0.1a");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), String);
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
        assert_eq_type!(ret.unwrap_err(), String);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4net_invalid_subnet_3digits() {
        let ret = find_ip_match("127.0.0.1/100");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), String);
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
        assert_eq_type!(ret.unwrap_err(), String);
    }

    #[test]
    fn test_find_ip_match_tn_ipv4addr_superceded_by_invalid() {
        let ret = find_ip_match("127.0.0.1111");

        assert!(ret.is_err());
        assert_eq_type!(ret.unwrap_err(), String);
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
    fn test_create_list_ipv4_tp_ipv4addr_single() {
        let ret = create_list_ipv4(vec!["127.0.0.255".to_owned()]);
        assert!(ret.is_ok());

        let list = ret.unwrap();
        assert_eq!(list.ips.len(), 1);
        assert_eq!(list.subnets.len(), 0);
        assert_eq!(list.ranges.len(), 0);
        assert!(list.ips.contains(Ipv4Addr::new(127, 0, 0, 255).to_bits()));
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4addr_multiple() {
        let ret = create_list_ipv4(vec!["127.0.0.255".to_owned(), "0.0.0.0".to_owned()]);
        assert!(ret.is_ok());

        let list = ret.unwrap();
        assert_eq!(list.ips.len(), 2);
        assert_eq!(list.subnets.len(), 0);
        assert_eq!(list.ranges.len(), 0);
        assert!(list.ips.contains(Ipv4Addr::new(127, 0, 0, 255).to_bits()));
        assert!(list.ips.contains(Ipv4Addr::new(0, 0, 0, 0).to_bits()));
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4addr_out_of_range() {
        let ret = create_list_ipv4(vec!["127.0.0.256".to_owned()]);

        assert!(ret.is_err());
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4net_single() {
        let ret = create_list_ipv4(vec!["127.0.0.255/32".to_owned()]);
        assert!(ret.is_ok());

        let list = ret.unwrap();
        assert_eq!(list.ips.len(), 0);
        assert_eq!(list.subnets.len(), 1);
        assert_eq!(list.ranges.len(), 0);
        assert_eq!(list.subnets[0], "127.0.0.255/32".parse().unwrap());
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4net_multiple() {
        let ret = create_list_ipv4(vec!["127.0.0.255/32".to_owned(), "0.0.0.0/32".to_owned()]);
        assert!(ret.is_ok());

        let list = ret.unwrap();
        assert_eq!(list.ips.len(), 0);
        assert_eq!(list.subnets.len(), 2);
        assert_eq!(list.ranges.len(), 0);
        assert_eq!(list.subnets[0], "127.0.0.255/32".parse().unwrap());
        assert_eq!(list.subnets[1], "0.0.0.0/32".parse().unwrap());
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4net_invalid_subnet() {
        let ret = create_list_ipv4(vec!["127.0.0.255/33".to_owned()]);
        assert!(ret.is_err());
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4range_single() {
        let ret = create_list_ipv4(vec!["127.0.0.254-127.0.0.255".to_owned()]);
        assert!(ret.is_ok());

        let list = ret.unwrap();
        assert_eq!(list.ips.len(), 0);
        assert_eq!(list.subnets.len(), 0);
        assert_eq!(list.ranges.len(), 1);
        assert_eq!(list.ranges[0].0, Ipv4Addr::new(127, 0, 0, 254));
        assert_eq!(list.ranges[0].1, Ipv4Addr::new(127, 0, 0, 255));
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4range_equal() {
        let ret = create_list_ipv4(vec!["127.0.0.254-127.0.0.254".to_owned()]);
        assert!(ret.is_err());
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4range_invalid() {
        let ret = create_list_ipv4(vec!["127.0.0.255-127.0.0.254".to_owned()]);
        assert!(ret.is_err());
    }
}
