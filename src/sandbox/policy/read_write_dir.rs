use std::path::PathBuf;

use super::read_dir::ReadDirectory;
use super::write_dir::WriteDirectory;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct ReadWriteDirectory(PathBuf);

impl ReadWriteDirectory {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

impl ToSbdl for ReadWriteDirectory {
    fn to_sbdl(&self) -> String {
        let p = self.0.as_path();
        format!(
            "{}\n{}",
            ReadDirectory::new(p).to_sbdl(),
            WriteDirectory::new(p).to_sbdl(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_directory_renders() {
        assert_eq!(
            ReadWriteDirectory::new("/tmp").to_sbdl(),
            "(allow file-read* (subpath \"/tmp\"))\n(allow file-write* (subpath \"/tmp\"))"
        );
    }
}
