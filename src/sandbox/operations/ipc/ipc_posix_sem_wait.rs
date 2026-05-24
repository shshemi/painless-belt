use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct IpcPosixSemWait {
    filter: IpcPosixFilter,
}

impl IpcPosixSemWait {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixSemWait {
    fn render(&self) -> String {
        format!("ipc-posix-sem-wait {}", self.filter.to_sbdl())
    }
}
