use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct IpcPosixSemPost {
    filter: IpcPosixFilter,
}

impl IpcPosixSemPost {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixSemPost {
    fn render(&self) -> String {
        format!("ipc-posix-sem-post {}", self.filter.to_sbdl())
    }
}
