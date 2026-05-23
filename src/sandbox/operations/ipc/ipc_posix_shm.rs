use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;
use crate::sandbox::operations::Filter;

#[derive(Debug)]
pub struct IpcPosixShm {
    filter: IpcPosixFilter,
}

impl IpcPosixShm {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixShm {
    fn render(&self) -> String {
        format!("ipc-posix-shm* {}", self.filter.render())
    }
}
