use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct IpcPosixShmWriteCreate {
    filter: IpcPosixFilter,
}

impl IpcPosixShmWriteCreate {
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

impl ToSbdl for IpcPosixShmWriteCreate {
    fn to_sbdl(&self) -> String {
        format!("ipc-posix-shm-write-create {}", self.filter.to_sbdl())
    }
}
