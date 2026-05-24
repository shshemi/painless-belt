use regex::Regex;

use crate::misc::ext::str_ext::StrExt;

pub mod mach_bootstrap;
pub mod mach_cross_domain_lookup;
pub mod mach_derive_port;
pub mod mach_host_special_port_set;
pub mod mach_issue_extension;
pub mod mach_lookup;
pub mod mach_per_user_lookup;
pub mod mach_priv_host_port;
pub mod mach_priv_task_port;
pub mod mach_register;
pub mod mach_task_name;

#[derive(Debug)]
enum MachFilter {
    GlobalName(String),
    LocalName(String),
    GlobalNameRegex(Regex),
    LocalNameRegex(Regex),
}

impl MachFilter {
    fn to_sbdl(&self) -> String {
        match self {
            MachFilter::GlobalName(n) => format!("(global-name \"{}\")", n.escape()),
            MachFilter::LocalName(n) => format!("(local-name \"{}\")", n.escape()),
            MachFilter::GlobalNameRegex(r) => {
                format!("(global-name-regex #\"{}\")", r.as_str().replace('"', "\\\""))
            }
            MachFilter::LocalNameRegex(r) => {
                format!("(local-name-regex #\"{}\")", r.as_str().replace('"', "\\\""))
            }
        }
    }
}
