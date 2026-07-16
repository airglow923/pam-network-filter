use fancy_regex::Regex;

const PAT_IPV4_STR: &str = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";

pub fn pat_ipv4() -> fancy_regex::Result<Regex> {
    Regex::new(format!(r"^{}$", PAT_IPV4_STR).as_str())
}

pub fn pat_ipv4_range() -> fancy_regex::Result<Regex> {
    Regex::new(format!(r"^{p}-{p}$", p = PAT_IPV4_STR).as_str())
}

pub fn pat_ipv4_subnet() -> fancy_regex::Result<Regex> {
    // IPv4 followed by CIDR notation /0-32
    // \d       => 0-9
    // [1-2]\d  => 10-29
    // 3[0-2]   => 30-32
    Regex::new(format!(r"^{}/(\d|[1-2]\d|3[0-2])$", PAT_IPV4_STR).as_str())
}

pub fn pat_fqdn() -> fancy_regex::Result<Regex> {
    // from RegExr FQDN: https://regexr.com/3g5j0
    Regex::new(r"^(?!:\/\/)(?=.{1,255}$)((.{1,63}\.){1,127}(?![0-9]*$)[a-z0-9-]+\.?)$")
}

pub fn pat_username() -> fancy_regex::Result<Regex> {
    Regex::new(r"^[a-z_]([a-z0-9_-]{0,31}|[a-z0-9_-]{0,30}\$)$")
}
