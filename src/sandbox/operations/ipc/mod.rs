use regex::Regex;

use crate::misc::ext::str_ext::StrExt;

pub mod ipc_posix_sem;
pub mod ipc_posix_sem_create;
pub mod ipc_posix_sem_open;
pub mod ipc_posix_sem_post;
pub mod ipc_posix_sem_unlink;
pub mod ipc_posix_sem_wait;
pub mod ipc_posix_shm;
pub mod ipc_posix_shm_read;
pub mod ipc_posix_shm_read_data;
pub mod ipc_posix_shm_read_metadata;
pub mod ipc_posix_shm_write;
pub mod ipc_posix_shm_write_create;
pub mod ipc_posix_shm_write_data;
pub mod ipc_posix_shm_write_unlink;

#[derive(Debug)]
enum IpcPosixFilter {
    Name(String),
    Regex(Regex),
}

impl IpcPosixFilter {
    fn to_sbdl(&self) -> String {
        match self {
            IpcPosixFilter::Name(n) => format!("(ipc-posix-name \"{}\")", n.escape()),
            IpcPosixFilter::Regex(r) => {
                format!("(ipc-posix-name-regex #\"{}\")", r.as_str().replace('"', "\\\""))
            }
        }
    }
}
