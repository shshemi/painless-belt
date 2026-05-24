use std::path::PathBuf;

use regex::Regex;

use super::FileFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct FileWriteUmount {
    filter: FileFilter,
}

impl FileWriteUmount {
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

impl Operation for FileWriteUmount {
    fn render(&self) -> String {
        format!("file-write-umount {}", self.filter.to_sbdl())
    }
}
