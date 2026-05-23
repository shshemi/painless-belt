use regex::Regex;

use super::IpcPosixFilter;
use crate::sandbox::Operation;
use crate::sandbox::operations::Filter;

#[derive(Debug)]
pub struct IpcPosixSemUnlink {
    filter: IpcPosixFilter,
}

impl IpcPosixSemUnlink {
    pub fn name(name: impl Into<String>) -> Self {
        Self { filter: IpcPosixFilter::Name(name.into()) }
    }

    pub fn regex(regex: Regex) -> Self {
        Self { filter: IpcPosixFilter::Regex(regex) }
    }
}

impl Operation for IpcPosixSemUnlink {
    fn render(&self) -> String {
        format!("ipc-posix-sem-unlink {}", self.filter.render())
    }
}
