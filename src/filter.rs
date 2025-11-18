use regex::Regex;
use std::collections::HashSet;
use std::marker::Sized;
use std::net::Ipv4Addr;

use crate::network;

pub trait Filter {
    type Value: ?Sized;

    fn contains(&self, value: &Self::Value) -> bool;
    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub struct FilterIp {
    list_ipv4: network::Ipv4List,
}

#[derive(Debug, Default)]
pub struct FilterUser {
    users: HashSet<String>,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FilterHost {}

impl Filter for FilterIp {
    type Value = str;

    fn contains(&self, rhost: &str) -> bool {
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

    fn is_empty(&self) -> bool {
        let network::Ipv4List {
            ips,
            subnets,
            ranges,
        } = &self.list_ipv4;

        ips.is_empty() && subnets.is_empty() && ranges.is_empty()
    }
}

impl Filter for FilterUser {
    type Value = str;

    fn contains(&self, user: &str) -> bool {
        self.users.contains(user)
    }

    fn is_empty(&self) -> bool {
        self.users.is_empty()
    }
}

pub fn filter_from_ips(ips: Vec<String>) -> Result<impl Filter<Value = str>, String> {
    let list_ipv4 = network::create_list_ipv4(ips)?;

    Ok(FilterIp { list_ipv4 })
}

pub fn filter_from_users(users: Vec<String>) -> Result<impl Filter<Value = str>, regex::Error> {
    let mut filter = FilterUser::default();
    let pattern_username = Regex::new(r"^[a-z_]([a-z0-9_-]{0,31}|[a-z0-9_-]{0,30}\$)$")?;

    for user in users {
        if pattern_username.is_match(&user) {
            filter.users.insert(user);
        } else {
            return Err(regex::Error::Syntax(format!(
                "'{}' wrong username syntax",
                user
            )));
        }
    }

    Ok(filter)
}
