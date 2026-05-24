use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct IpcPosixShmWrite {
    filter: IpcPosixFilter,
}

impl IpcPosixShmWrite {
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

impl ToSbdl for IpcPosixShmWrite {
    fn to_sbdl(&self) -> String {
        format!("ipc-posix-shm-write* {}", self.filter.to_sbdl())
    }
}
