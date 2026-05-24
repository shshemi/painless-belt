use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct IpcPosixShmReadData {
    filter: IpcPosixFilter,
}

impl IpcPosixShmReadData {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixShmReadData {
    fn render(&self) -> String {
        format!("ipc-posix-shm-read-data {}", self.filter.to_sbdl())
    }
}
