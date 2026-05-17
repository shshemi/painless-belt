use std::path::PathBuf;

use super::escape;
use crate::traits::ToSbpl;

#[derive(Debug)]
pub struct WriteDirectory(PathBuf);

impl WriteDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl ToSbpl for WriteDirectory {
    fn to_sbpl(&self) -> String {
        format!(
            "(allow file-write* (subpath \"{}\"))",
            escape(&self.0.to_string_lossy())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_directory_renders() {
        assert_eq!(
            WriteDirectory::new("/tmp").to_sbpl(),
            r#"(allow file-write* (subpath "/tmp"))"#
        );
    }

    #[test]
    fn write_directory_escapes_quotes_and_backslashes() {
        assert_eq!(
            WriteDirectory::new(r#"/odd"path\here"#).to_sbpl(),
            r#"(allow file-write* (subpath "/odd\"path\\here"))"#
        );
    }
}
