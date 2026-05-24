use std::path::PathBuf;

use regex::Regex;

use super::FileFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct FileWriteMode {
    filter: FileFilter,
}

impl FileWriteMode {
    pub fn literal(path: impl Into<PathBuf>) -> Self {
        Self {
            filter: FileFilter::Literal(path.into()),
        }
    }

    pub fn prefix(path: impl Into<PathBuf>) -> Self {
        Self {
            filter: FileFilter::Prefix(path.into()),
        }
    }

    pub fn subpath(path: impl Into<PathBuf>) -> Self {
        Self {
            filter: FileFilter::Subpath(path.into()),
        }
    }

    pub fn regex(regex: Regex) -> Self {
        Self {
            filter: FileFilter::Regex(regex),
        }
    }
}

impl ToSbdl for FileWriteMode {
    fn to_sbdl(&self) -> String {
        format!("file-write-mode {}", self.filter.to_sbdl())
    }
}
