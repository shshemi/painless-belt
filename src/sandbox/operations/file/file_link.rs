use std::path::PathBuf;

use regex::Regex;

use super::FileFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct FileLink {
    filter: FileFilter,
}

impl FileLink {
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

impl Operation for FileLink {
    fn render(&self) -> String {
        format!("file-link {}", self.filter.to_sbdl())
    }
}
