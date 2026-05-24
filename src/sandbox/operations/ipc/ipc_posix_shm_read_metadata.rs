use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct IpcPosixShmReadMetadata {
    filter: IpcPosixFilter,
}

impl IpcPosixShmReadMetadata {
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

impl ToSbdl for IpcPosixShmReadMetadata {
    fn to_sbdl(&self) -> String {
        format!("ipc-posix-shm-read-metadata {}", self.filter.to_sbdl())
    }
}
