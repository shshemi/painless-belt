use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct IpcPosixSemOpen {
    filter: IpcPosixFilter,
}

impl IpcPosixSemOpen {
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            filter: IpcPosixFilter::Name(name.into()),
        }
    }

    pub fn regex(regex: Regex) -> Self {
        Self {
            filter: IpcPosixFilter::Regex(regex),
        }
    }
}

impl ToSbdl for IpcPosixSemOpen {
    fn to_sbdl(&self) -> String {
        format!("ipc-posix-sem-open {}", self.filter.to_sbdl())
    }
}
