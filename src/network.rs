// ip pattern
// 0.0.0.0
// 0.0.0.0/0
// 0.0.0.0-0.0.00
//
// 1. remove duplicate checks
// 2. merge individual ip addresses into subnets and ranges if possible
//      - check ip is in subnet (easy with library)
//      - check ip is in range (not directly supported by library)
//          - iterate through range and collect all in bitset then match
//              - costly -- iteration is costly
//          - cut this bs and just compare directly (lower <= ip <= upper)
//
//  - store allowlist in three ways:
//      1. create bitset for individual ip
//      2. vector of subnets
//      3. vector of ranges

extern crate ipnet;
extern crate roaring;

use ipnet::Ipv4Net;
use regex::Regex;
use roaring::RoaringBitmap;
use std::error::Error;
use std::net::Ipv4Addr;

enum Pattern {
    Ipv4Addr,
    Ipv6Addr,
    Ipv4Net,
    Ipv6Net,
    Ipv4Range,
    Ipv6Range,
}

enum IpAllowlist {
    V4(Ipv4Allowlist),
}

#[derive(Debug, Clone)]
struct Ipv4Allowlist {
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

    let pattern_ipv4net = Regex::new(format!(r"^{}/\d{{1,2}}$", pattern_ipv4addr_str).as_str())?;

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

        let _: Result<(), Box<dyn Error>> = match pat {
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
            _ => Err(Box::<dyn Error>::from("")),
        };
    }

    Ok(IpAllowlist::V4(Ipv4Allowlist {
        ips,
        subnets,
        ranges,
    }))
}
