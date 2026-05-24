use regex::Regex;

use super::SysctlFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct SysctlRead {
    filter: SysctlFilter,
}

impl SysctlRead {
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

impl Operation for SysctlRead {
    fn render(&self) -> String {
        format!("sysctl-read {}", self.filter.to_sbdl())
    }
}
