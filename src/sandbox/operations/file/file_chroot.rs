use std::path::PathBuf;

use regex::Regex;

use super::FileFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct FileChroot {
    filter: FileFilter,
}

impl FileChroot {
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

impl ToSbdl for FileChroot {
    fn to_sbdl(&self) -> String {
        format!("file-chroot {}", self.filter.to_sbdl())
    }
}
