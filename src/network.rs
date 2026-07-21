use std::net::Ipv4Addr;

use anyhow::{Result, bail};
use ipnet::Ipv4Net;
use roaring::RoaringBitmap;

use crate::pattern;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Pattern {
    Ipv4Addr,
    Ipv6Addr,
    Ipv4Net,
    Ipv6Net,
    Ipv4Range,
    Ipv6Range,
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

fn find_ip_match(ip: &str) -> Result<Pattern> {
    let pat_ipv4_range = pattern::pat_ipv4_range();

    if pat_ipv4_range.is_match(ip)? {
        return Ok(Pattern::Ipv4Range);
    }

    let pat_ipv4_subnet = pattern::pat_ipv4_subnet();

    if pat_ipv4_subnet.is_match(ip)? {
        return Ok(Pattern::Ipv4Net);
    }

    let pat_ipv4 = pattern::pat_ipv4();

    if pat_ipv4.is_match(ip)? {
        return Ok(Pattern::Ipv4Addr);
    }

    bail!("'{}' no matching pattern", ip)
}

#[allow(dead_code)]
fn is_domain(domain: &str) -> Result<()> {
    let pat_fqdn = pattern::pat_fqdn();

    if pat_fqdn.is_match(domain)? {
        return Ok(());
    }

    bail!("'{}' wrong domain syntax", domain)
}

pub fn create_list_ipv4(ip_list: Vec<String>) -> Result<Ipv4List> {
    let mut ips = RoaringBitmap::new();
    let mut subnets = Vec::new();
    let mut ranges = Vec::new();

    for ip in ip_list {
        let pat = find_ip_match(&ip)?;

        let ret: Result<(), String> = match pat {
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

                if let [lower, upper] = &range[0..2] {
                    if lower >= upper {
                        bail!(
                            "'{}' IP on left side should be lower than the right one",
                            ip
                        );
                    }
                    ranges.push((lower.parse::<Ipv4Addr>()?, upper.parse::<Ipv4Addr>()?));
                } else {
                    bail!("'{}' wrong input for IPv4 range syntax", ip);
                }

                Ok(())
            }
            _ => Err("no matching pattern".to_owned()),
        };

        if let Err(e) = ret {
            bail!(e);
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
    use crate::error;

    use super::*;
    use anyhow::{Ok, Result};
    use std::net::AddrParseError;

    #[test]
    fn test_find_ip_match_tp_ipv4range() -> Result<()> {
        let ret = find_ip_match("127.0.0.1-127.0.0.1");

        assert_eq!(ret?, Pattern::Ipv4Range);

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tn_ipv4range_preceded_by_invalid() -> Result<()> {
        let ret = find_ip_match("1127.0.0.1-127.0.0.1").expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("no matching pattern"));

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tn_ipv4range_superceded_by_invalid() -> Result<()> {
        let ret = find_ip_match("127.0.0.1-127.0.0.1a").expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("no matching pattern"));

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tp_ipv4net() -> Result<()> {
        let ret = find_ip_match("127.0.0.1/32");

        assert_eq!(ret?, Pattern::Ipv4Net);

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tp_ipv4net_subnet_0to32() -> Result<()> {
        for i in 0..=32 {
            let ret = find_ip_match(format!("127.0.0.1/{}", i.to_string()).as_str());

            assert_eq!(ret?, Pattern::Ipv4Net);
        }

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tn_ipv4net_invalid_subnet() -> Result<()> {
        let ret = find_ip_match("127.0.0.1/33").expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("no matching pattern"));

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tn_ipv4net_invalid_subnet_3digits() -> Result<()> {
        let ret = find_ip_match("127.0.0.1/100").expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("no matching pattern"));

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tp_ipv4addr() -> Result<()> {
        let ret = find_ip_match("127.0.0.1");

        assert_eq!(ret?, Pattern::Ipv4Addr);

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tn_ipv4addr_preceded_by_invalid() -> Result<()> {
        let ret = find_ip_match("1127.0.0.1").expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("no matching pattern"));

        Ok(())
    }

    #[test]
    fn test_find_ip_match_tn_ipv4addr_superceded_by_invalid() -> Result<()> {
        let ret = find_ip_match("127.0.0.1111").expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("no matching pattern"));

        Ok(())
    }

    // currently regex pattern does not check whether an IP octet is in 0-255
    // however, this is handled when parsed as Ipv4Addr
    // doing such check twice is unnecessary, so this behavior is expected
    #[test]
    fn test_find_ip_match_fp_ipv4addr_out_of_range() -> Result<()> {
        let ret = find_ip_match("127.0.0.256");

        assert_eq!(ret?, Pattern::Ipv4Addr);

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4addr_single() -> Result<()> {
        let ret = create_list_ipv4(vec!["127.0.0.255".to_owned()]);

        let list = ret?;
        assert_eq!(list.ips.len(), 1);
        assert_eq!(list.subnets.len(), 0);
        assert_eq!(list.ranges.len(), 0);
        assert!(list.ips.contains(Ipv4Addr::new(127, 0, 0, 255).to_bits()));

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4addr_multiple() -> Result<()> {
        let ret = create_list_ipv4(vec!["127.0.0.255".to_owned(), "0.0.0.0".to_owned()]);

        let list = ret?;
        assert_eq!(list.ips.len(), 2);
        assert_eq!(list.subnets.len(), 0);
        assert_eq!(list.ranges.len(), 0);
        assert!(list.ips.contains(Ipv4Addr::new(127, 0, 0, 255).to_bits()));
        assert!(list.ips.contains(Ipv4Addr::new(0, 0, 0, 0).to_bits()));

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4addr_out_of_range() -> Result<()> {
        let ret = create_list_ipv4(vec!["127.0.0.256".to_owned()]).expect_err("must fail");

        assert!(error::is_underlying::<AddrParseError>(&ret));
        assert!(ret.to_string().contains("IPv4 address"));

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4net_single() -> Result<()> {
        let ret = create_list_ipv4(vec!["127.0.0.255/32".to_owned()]);

        let list = ret?;
        assert_eq!(list.ips.len(), 0);
        assert_eq!(list.subnets.len(), 1);
        assert_eq!(list.ranges.len(), 0);
        assert_eq!(list.subnets[0], "127.0.0.255/32".parse().unwrap());

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4net_multiple() -> Result<()> {
        let ret = create_list_ipv4(vec!["127.0.0.255/32".to_owned(), "0.0.0.0/32".to_owned()]);

        let list = ret?;
        assert_eq!(list.ips.len(), 0);
        assert_eq!(list.subnets.len(), 2);
        assert_eq!(list.ranges.len(), 0);
        assert_eq!(list.subnets[0], "127.0.0.255/32".parse().unwrap());
        assert_eq!(list.subnets[1], "0.0.0.0/32".parse().unwrap());

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4net_invalid_subnet() -> Result<()> {
        let ret = create_list_ipv4(vec!["127.0.0.255/33".to_owned()]).expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("no matching pattern"));

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tp_ipv4range_single() -> Result<()> {
        let ret = create_list_ipv4(vec!["127.0.0.254-127.0.0.255".to_owned()]);

        let list = ret?;
        assert_eq!(list.ips.len(), 0);
        assert_eq!(list.subnets.len(), 0);
        assert_eq!(list.ranges.len(), 1);
        assert_eq!(list.ranges[0].0, Ipv4Addr::new(127, 0, 0, 254));
        assert_eq!(list.ranges[0].1, Ipv4Addr::new(127, 0, 0, 255));

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4range_equal() -> Result<()> {
        let ret =
            create_list_ipv4(vec!["127.0.0.254-127.0.0.254".to_owned()]).expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("IP on left side"));

        Ok(())
    }

    #[test]
    fn test_create_list_ipv4_tn_ipv4range_invalid() -> Result<()> {
        let ret =
            create_list_ipv4(vec!["127.0.0.255-127.0.0.254".to_owned()]).expect_err("must fail");

        assert!(error::is_underlying::<String>(&ret));
        assert!(ret.to_string().contains("IP on left side"));

        Ok(())
    }
}
