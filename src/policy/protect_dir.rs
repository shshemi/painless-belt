use std::path::PathBuf;

use super::escape;
use crate::traits::ToSbpl;

#[derive(Debug)]
pub struct ProtectDirectory(PathBuf);

impl ProtectDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl ToSbpl for ProtectDirectory {
    fn to_sbpl(&self) -> String {
        let p = escape(&self.0.to_string_lossy());
        format!("(deny file-read* (subpath \"{p}\"))\n(deny file-write* (subpath \"{p}\"))")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protect_directory_renders() {
        assert_eq!(
            ProtectDirectory::new("/Users/me/.ssh").to_sbpl(),
            "(deny file-read* (subpath \"/Users/me/.ssh\"))\n\
             (deny file-write* (subpath \"/Users/me/.ssh\"))"
        );
    }

    #[test]
    fn protect_directory_escapes_quotes_and_backslashes() {
        assert_eq!(
            ProtectDirectory::new(r#"/odd"path\here"#).to_sbpl(),
            "(deny file-read* (subpath \"/odd\\\"path\\\\here\"))\n\
             (deny file-write* (subpath \"/odd\\\"path\\\\here\"))"
        );
    }
}
