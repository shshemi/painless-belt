use regex::Regex;

use super::SysctlFilter;
use crate::sandbox::Operation;
use crate::sandbox::operations::Filter;

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

impl Operation for SysctlWrite {
    fn render(&self) -> String {
        format!("sysctl-write {}", self.filter.render())
    }
}
