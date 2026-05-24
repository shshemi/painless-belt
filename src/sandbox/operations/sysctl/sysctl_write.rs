use regex::Regex;

use super::SysctlFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct SysctlWrite {
    filter: SysctlFilter,
}

impl SysctlWrite {
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            filter: SysctlFilter::Name(name.into()),
        }
    }

    pub fn regex(regex: Regex) -> Self {
        Self {
            filter: SysctlFilter::Regex(regex),
        }
    }
}

impl ToSbdl for SysctlWrite {
    fn to_sbdl(&self) -> String {
        format!("sysctl-write {}", self.filter.to_sbdl())
    }
}
