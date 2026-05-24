use std::path::PathBuf;

use regex::Regex;

use crate::misc::ext::str_ext::StrExt;

pub mod file_chroot;
pub mod file_fsctl;
pub mod file_ioctl;
pub mod file_issue_extension;
pub mod file_link;
pub mod file_map_executable;
pub mod file_mknod;
pub mod file_read;
pub mod file_read_data;
pub mod file_read_metadata;
pub mod file_read_xattr;
pub mod file_revoke;
pub mod file_search;
pub mod file_write;
pub mod file_write_create;
pub mod file_write_data;
pub mod file_write_flags;
pub mod file_write_mode;
pub mod file_write_mount;
pub mod file_write_owner;
pub mod file_write_setugid;
pub mod file_write_times;
pub mod file_write_umount;
pub mod file_write_unlink;
pub mod file_write_xattr;

#[derive(Debug)]
enum FileFilter {
    Literal(PathBuf),
    Prefix(PathBuf),
    Subpath(PathBuf),
    Regex(Regex),
}

impl FileFilter {
    fn to_sbdl(&self) -> String {
        match self {
            FileFilter::Literal(p) => {
                format!("(literal \"{}\")", p.to_string_lossy().escape())
            }
            FileFilter::Prefix(p) => {
                format!("(prefix \"{}\")", p.to_string_lossy().escape())
            }
            FileFilter::Subpath(p) => {
                format!("(subpath \"{}\")", p.to_string_lossy().escape())
            }
            FileFilter::Regex(r) => {
                format!("(regex #\"{}\")", r.as_str().replace('"', "\\\""))
            }
        }
    }
}
