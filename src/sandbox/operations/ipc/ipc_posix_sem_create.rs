use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct IpcPosixSemCreate {
    filter: IpcPosixFilter,
}

impl IpcPosixSemCreate {
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

impl ToSbdl for IpcPosixSemCreate {
    fn to_sbdl(&self) -> String {
        format!("ipc-posix-sem-create {}", self.filter.to_sbdl())
    }
}
