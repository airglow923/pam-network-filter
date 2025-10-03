use crate::network;

use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct FilterIp {
    list_ipv4: network::Ipv4List,
}

impl FilterIp {
    pub fn contains(&self, rhost: &str) -> bool {
        let ip = match rhost.parse::<Ipv4Addr>() {
            Ok(x) => x,
            Err(_) => {
                return false;
            }
        };

        if self.list_ipv4.ips.contains(ip.to_bits()) {
            return true;
        }

        for subnet in &self.list_ipv4.subnets {
            if subnet.contains(&ip) {
                return true;
            }
        }

        for range in &self.list_ipv4.ranges {
            let (lower, upper) = &range;

            if lower <= &ip && &ip <= upper {
                return true;
            }
        }

        false
    }

    pub fn is_empty(&self) -> bool {
        let network::Ipv4List {
            ips,
            subnets,
            ranges,
        } = &self.list_ipv4;

        ips.is_empty() && subnets.is_empty() && ranges.is_empty()
    }
}

pub fn filter_from_ips(ips: Vec<String>) -> Result<FilterIp, String> {
    let list_ipv4 = network::create_list_ipv4(ips)?;

    Ok(FilterIp { list_ipv4 })
}

pub fn filter_from_users(users: Vec<String>) -> Result<HashSet<String>, regex::Error> {
    let mut filter = HashSet::new();
    let pattern_username = Regex::new(r"^[a-z_]([a-z0-9_-]{0,31}|[a-z0-9_-]{0,30}\$)$")?;

    for user in users {
        if pattern_username.is_match(&user) {
            filter.insert(user);
        } else {
            return Err(regex::Error::Syntax(format!(
                "'{}' wrong username syntax",
                user
            )));
        }
    }

    Ok(filter)
}
