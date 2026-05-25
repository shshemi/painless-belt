use std::path::PathBuf;

use crate::misc::ext::str_ext::StrExt;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct ReadDirectory(PathBuf);

impl ReadDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl ToSbdl for ReadDirectory {
    fn to_sbdl(&self) -> String {
        format!(
            "(allow file-read* (subpath \"{}\"))",
            self.0.to_string_lossy().escape()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_directory_renders() {
        assert_eq!(
            ReadDirectory::new("/etc").to_sbdl(),
            r#"(allow file-read* (subpath "/etc"))"#
        );
    }

    #[test]
    fn read_directory_escapes_quotes_and_backslashes() {
        assert_eq!(
            ReadDirectory::new(r#"/odd"path\here"#).to_sbdl(),
            r#"(allow file-read* (subpath "/odd\"path\\here"))"#
        );
    }
}
