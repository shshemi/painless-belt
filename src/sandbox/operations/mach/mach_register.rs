use regex::Regex;

use super::MachFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachRegister {
    filter: MachFilter,
}

impl MachRegister {
    pub fn global_name(name: impl Into<String>) -> Self {
        Self {
            filter: MachFilter::GlobalName(name.into()),
        }
    }

    pub fn local_name(name: impl Into<String>) -> Self {
        Self {
            filter: MachFilter::LocalName(name.into()),
        }
    }

    pub fn global_name_regex(regex: Regex) -> Self {
        Self {
            filter: MachFilter::GlobalNameRegex(regex),
        }
    }

    pub fn local_name_regex(regex: Regex) -> Self {
        Self {
            filter: MachFilter::LocalNameRegex(regex),
        }
    }
}

impl ToSbdl for MachRegister {
    fn to_sbdl(&self) -> String {
        format!("mach-register {}", self.filter.to_sbdl())
    }
}
