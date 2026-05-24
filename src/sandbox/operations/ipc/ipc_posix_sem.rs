use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct IpcPosixSem {
    filter: IpcPosixFilter,
}

impl IpcPosixSem {
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

impl ToSbdl for IpcPosixSem {
    fn to_sbdl(&self) -> String {
        format!("ipc-posix-sem* {}", self.filter.to_sbdl())
    }
}
