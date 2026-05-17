use std::path::PathBuf;

use super::escape;
use crate::traits::ToSbpl;

#[derive(Debug)]
pub struct ReadDirectory(PathBuf);

impl ReadDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl ToSbpl for ReadDirectory {
    fn to_sbpl(&self) -> String {
        format!(
            "(allow file-read* (subpath \"{}\"))",
            escape(&self.0.to_string_lossy())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_directory_renders() {
        assert_eq!(
            ReadDirectory::new("/etc").to_sbpl(),
            r#"(allow file-read* (subpath "/etc"))"#
        );
    }

    #[test]
    fn read_directory_escapes_quotes_and_backslashes() {
        assert_eq!(
            ReadDirectory::new(r#"/odd"path\here"#).to_sbpl(),
            r#"(allow file-read* (subpath "/odd\"path\\here"))"#
        );
    }
}
