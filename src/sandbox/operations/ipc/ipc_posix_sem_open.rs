use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct IpcPosixSemOpen {
    filter: IpcPosixFilter,
}

impl IpcPosixSemOpen {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixSemOpen {
    fn render(&self) -> String {
        format!("ipc-posix-sem-open {}", self.filter.to_sbdl())
    }
}
