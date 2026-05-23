use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;
use crate::sandbox::operations::Filter;

#[derive(Debug)]
pub struct IpcPosixSemCreate {
    filter: IpcPosixFilter,
}

impl IpcPosixSemCreate {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixSemCreate {
    fn render(&self) -> String {
        format!("ipc-posix-sem-create {}", self.filter.render())
    }
}
