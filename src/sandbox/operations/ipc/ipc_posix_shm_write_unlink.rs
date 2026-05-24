use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct IpcPosixShmWriteUnlink {
    filter: IpcPosixFilter,
}

impl IpcPosixShmWriteUnlink {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixShmWriteUnlink {
    fn render(&self) -> String {
        format!("ipc-posix-shm-write-unlink {}", self.filter.to_sbdl())
    }
}
