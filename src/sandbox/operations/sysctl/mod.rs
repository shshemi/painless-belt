use regex::Regex;

use crate::misc::ext::str_ext::StrExt;

pub mod sysctl_read;
pub mod sysctl_write;

#[derive(Debug)]
enum SysctlFilter {
    Name(String),
    Regex(Regex),
}

impl SysctlFilter {
    fn to_sbdl(&self) -> String {
        match self {
            SysctlFilter::Name(n) => format!("(sysctl-name \"{}\")", n.escape()),
            SysctlFilter::Regex(r) => {
                format!("(sysctl-name-regex #\"{}\")", r.as_str().replace('"', "\\\""))
            }
        }
    }
}
