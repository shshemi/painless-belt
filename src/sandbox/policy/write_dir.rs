use std::path::PathBuf;

use crate::misc::ext::str_ext::StrExt;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct WriteDirectory(PathBuf);

impl WriteDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl ToSbdl for WriteDirectory {
    fn to_sbdl(&self) -> String {
        format!(
            "(allow file-write* (subpath \"{}\"))",
            self.0.to_string_lossy().escape()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_directory_renders() {
        assert_eq!(
            WriteDirectory::new("/tmp").to_sbdl(),
            r#"(allow file-write* (subpath "/tmp"))"#
        );
    }

    #[test]
    fn write_directory_escapes_quotes_and_backslashes() {
        assert_eq!(
            WriteDirectory::new(r#"/odd"path\here"#).to_sbdl(),
            r#"(allow file-write* (subpath "/odd\"path\\here"))"#
        );
    }
}
