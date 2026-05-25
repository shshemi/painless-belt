use std::{fmt::Write, marker::PhantomData, path::Path};

use regex::Regex;

use crate::misc::ext::str_ext::StrExt;
use crate::sandbox::ToSbdl;

#[derive(Debug, Default)]
pub struct RuleSet<A = (), O = ()> {
    inner: String,
    phantom: PhantomData<(A, O)>,
}

impl RuleSet<(), ()> {
    pub fn allow(self) -> RuleSet<Allow, ()> {
        RuleSet {
            inner: self.inner,
            phantom: PhantomData,
        }
    }

    pub fn deny(self) -> RuleSet<Deny, ()> {
        RuleSet {
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}

impl ToSbdl for RuleSet<(), ()> {
    fn to_sbdl(&self) -> &str {
        &self.inner
    }
}

pub trait Action {
    fn sbdl() -> &'static str;
}

pub struct Allow;

impl Action for Allow {
    fn sbdl() -> &'static str {
        "allow"
    }
}

pub struct Deny;

impl Action for Deny {
    fn sbdl() -> &'static str {
        "deny"
    }
}

macro_rules! file_op {
    ($method:ident, $marker:ident, $sbpl:literal) => {
        pub struct $marker;

        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(self) -> RuleSet<A, $marker> {
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }

        impl<A: Action> RuleSet<A, $marker> {
            pub fn literal(mut self, path: impl AsRef<Path>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (literal \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    path.as_ref().to_string_lossy().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn prefix(mut self, path: impl AsRef<Path>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (prefix \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    path.as_ref().to_string_lossy().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn subpath(mut self, path: impl AsRef<Path>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (subpath \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    path.as_ref().to_string_lossy().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn regex(mut self, regex: Regex) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (regex #\"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    regex_str(&regex)
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn any(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! mach_op {
    ($method:ident, $marker:ident, $sbpl:literal) => {
        pub struct $marker;

        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(self) -> RuleSet<A, $marker> {
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }

        impl<A: Action> RuleSet<A, $marker> {
            pub fn global_name(mut self, name: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (global-name \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    name.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn local_name(mut self, name: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (local-name \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    name.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn global_name_regex(mut self, regex: Regex) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (global-name-regex #\"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    regex_str(&regex)
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn local_name_regex(mut self, regex: Regex) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (local-name-regex #\"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    regex_str(&regex)
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn any(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! ipc_op {
    ($method:ident, $marker:ident, $sbpl:literal) => {
        pub struct $marker;

        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(self) -> RuleSet<A, $marker> {
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }

        impl<A: Action> RuleSet<A, $marker> {
            pub fn name(mut self, name: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (ipc-posix-name \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    name.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn regex(mut self, regex: Regex) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (ipc-posix-name-regex #\"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    regex_str(&regex)
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn any(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! sysctl_op {
    ($method:ident, $marker:ident, $sbpl:literal) => {
        pub struct $marker;

        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(self) -> RuleSet<A, $marker> {
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }

        impl<A: Action> RuleSet<A, $marker> {
            pub fn name(mut self, name: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (sysctl-name \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    name.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn regex(mut self, regex: Regex) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (sysctl-name-regex #\"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    regex_str(&regex)
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn any(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! iokit_op {
    ($method:ident, $marker:ident, $sbpl:literal) => {
        pub struct $marker;

        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(self) -> RuleSet<A, $marker> {
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }

        impl<A: Action> RuleSet<A, $marker> {
            pub fn user_client_class(mut self, class: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (iokit-user-client-class \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    class.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn user_client_class_regex(mut self, regex: Regex) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (iokit-user-client-class-regex #\"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    regex_str(&regex)
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn property(mut self, prop: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (iokit-property \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    prop.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn property_regex(mut self, regex: Regex) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (iokit-property-regex #\"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    regex_str(&regex)
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn any(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! network_op {
    ($method:ident, $marker:ident, $sbpl:literal) => {
        pub struct $marker;

        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(self) -> RuleSet<A, $marker> {
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }

        impl<A: Action> RuleSet<A, $marker> {
            pub fn local(mut self, proto: Proto, address: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (local {} \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    proto.sbdl(),
                    address.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn remote(mut self, proto: Proto, address: impl AsRef<str>) -> RuleSet<(), ()> {
                writeln!(
                    self.inner,
                    "({} {} (remote {} \"{}\"))",
                    A::sbdl(),
                    $sbpl,
                    proto.sbdl(),
                    address.as_ref().escape()
                )
                .unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn any(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! signal_op {
    ($method:ident, $marker:ident, $sbpl:literal) => {
        pub struct $marker;

        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(self) -> RuleSet<A, $marker> {
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }

        impl<A: Action> RuleSet<A, $marker> {
            pub fn self_target(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {} (target self))", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn others(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {} (target others))", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }

            pub fn any(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

macro_rules! bare_op {
    ($method:ident, $sbpl:literal) => {
        impl<A: Action> RuleSet<A, ()> {
            pub fn $method(mut self) -> RuleSet<(), ()> {
                writeln!(self.inner, "({} {})", A::sbdl(), $sbpl).unwrap();
                RuleSet {
                    inner: self.inner,
                    phantom: PhantomData,
                }
            }
        }
    };
}

// ── file ──
file_op!(file_chroot, FileChroot, "file-chroot");
file_op!(file_fsctl, FileFsctl, "file-fsctl");
file_op!(file_ioctl, FileIoctl, "file-ioctl");
file_op!(
    file_issue_extension,
    FileIssueExtension,
    "file-issue-extension"
);
file_op!(file_link, FileLink, "file-link");
file_op!(
    file_map_executable,
    FileMapExecutable,
    "file-map-executable"
);
file_op!(file_mknod, FileMknod, "file-mknod");
file_op!(file_read, FileRead, "file-read*");
file_op!(file_read_data, FileReadData, "file-read-data");
file_op!(file_read_metadata, FileReadMetadata, "file-read-metadata");
file_op!(file_read_xattr, FileReadXattr, "file-read-xattr");
file_op!(file_revoke, FileRevoke, "file-revoke");
file_op!(file_search, FileSearch, "file-search");
file_op!(file_write, FileWrite, "file-write*");
file_op!(file_write_create, FileWriteCreate, "file-write-create");
file_op!(file_write_data, FileWriteData, "file-write-data");
file_op!(file_write_flags, FileWriteFlags, "file-write-flags");
file_op!(file_write_mode, FileWriteMode, "file-write-mode");
file_op!(file_write_mount, FileWriteMount, "file-write-mount");
file_op!(file_write_owner, FileWriteOwner, "file-write-owner");
file_op!(file_write_setugid, FileWriteSetugid, "file-write-setugid");
file_op!(file_write_times, FileWriteTimes, "file-write-times");
file_op!(file_write_umount, FileWriteUmount, "file-write-umount");
file_op!(file_write_unlink, FileWriteUnlink, "file-write-unlink");
file_op!(file_write_xattr, FileWriteXattr, "file-write-xattr");

// ── mach (filtered) ──
mach_op!(mach_lookup, MachLookup, "mach-lookup");
mach_op!(mach_register, MachRegister, "mach-register");

// ── mach (bare) ──
bare_op!(mach_bootstrap, "mach-bootstrap");
bare_op!(mach_cross_domain_lookup, "mach-cross-domain-lookup");
bare_op!(mach_derive_port, "mach-derive-port");
bare_op!(mach_host_special_port_set, "mach-host-special-port-set");
bare_op!(mach_issue_extension, "mach-issue-extension");
bare_op!(mach_per_user_lookup, "mach-per-user-lookup");
bare_op!(mach_priv_host_port, "mach-priv-host-port");
bare_op!(mach_priv_task_port, "mach-priv-task-port");
bare_op!(mach_task_name, "mach-task-name");

// ── ipc ──
ipc_op!(ipc_posix_sem, IpcPosixSem, "ipc-posix-sem*");
ipc_op!(
    ipc_posix_sem_create,
    IpcPosixSemCreate,
    "ipc-posix-sem-create"
);
ipc_op!(ipc_posix_sem_open, IpcPosixSemOpen, "ipc-posix-sem-open");
ipc_op!(ipc_posix_sem_post, IpcPosixSemPost, "ipc-posix-sem-post");
ipc_op!(
    ipc_posix_sem_unlink,
    IpcPosixSemUnlink,
    "ipc-posix-sem-unlink"
);
ipc_op!(ipc_posix_sem_wait, IpcPosixSemWait, "ipc-posix-sem-wait");
ipc_op!(ipc_posix_shm, IpcPosixShm, "ipc-posix-shm*");
ipc_op!(ipc_posix_shm_read, IpcPosixShmRead, "ipc-posix-shm-read*");
ipc_op!(
    ipc_posix_shm_read_data,
    IpcPosixShmReadData,
    "ipc-posix-shm-read-data"
);
ipc_op!(
    ipc_posix_shm_read_metadata,
    IpcPosixShmReadMetadata,
    "ipc-posix-shm-read-metadata"
);
ipc_op!(
    ipc_posix_shm_write,
    IpcPosixShmWrite,
    "ipc-posix-shm-write*"
);
ipc_op!(
    ipc_posix_shm_write_create,
    IpcPosixShmWriteCreate,
    "ipc-posix-shm-write-create"
);
ipc_op!(
    ipc_posix_shm_write_data,
    IpcPosixShmWriteData,
    "ipc-posix-shm-write-data"
);
ipc_op!(
    ipc_posix_shm_write_unlink,
    IpcPosixShmWriteUnlink,
    "ipc-posix-shm-write-unlink"
);

// ── sysctl ──
sysctl_op!(sysctl_read, SysctlRead, "sysctl-read");
sysctl_op!(sysctl_write, SysctlWrite, "sysctl-write");

// ── iokit ──
iokit_op!(iokit_open, IokitOpen, "iokit-open");
iokit_op!(
    iokit_set_properties,
    IokitSetProperties,
    "iokit-set-properties"
);

// ── network ──
network_op!(network, Network, "network*");
network_op!(network_bind, NetworkBind, "network-bind");
network_op!(network_inbound, NetworkInbound, "network-inbound");
network_op!(network_outbound, NetworkOutbound, "network-outbound");

// ── signal ──
signal_op!(signal, Signal, "signal");

fn regex_str(r: &Regex) -> String {
    r.as_str().replace('"', "\\\"")
}

#[derive(Debug)]
pub enum Proto {
    Ip,
    Tcp,
    Udp,
}

impl Proto {
    fn sbdl(&self) -> &'static str {
        match self {
            Proto::Ip => "ip",
            Proto::Tcp => "tcp",
            Proto::Udp => "udp",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rs() -> RuleSet<(), ()> {
        RuleSet::default()
    }

    #[test]
    fn allow_file_read_subpath() {
        assert_eq!(
            rs().allow().file_read().subpath("/").to_sbdl(),
            "(allow file-read* (subpath \"/\"))\n"
        );
    }

    #[test]
    fn deny_file_write_prefix() {
        assert_eq!(
            rs().deny().file_write().prefix("/etc/").to_sbdl(),
            "(deny file-write* (prefix \"/etc/\"))\n"
        );
    }

    #[test]
    fn file_literal_escapes_quotes_and_backslashes() {
        assert_eq!(
            rs().allow()
                .file_read()
                .literal(r#"/odd"path\here"#)
                .to_sbdl(),
            "(allow file-read* (literal \"/odd\\\"path\\\\here\"))\n"
        );
    }

    #[test]
    fn file_search_regex() {
        let re = Regex::new(r"^/tmp/[0-9]+$").unwrap();
        assert_eq!(
            rs().allow().file_search().regex(re).to_sbdl(),
            "(allow file-search (regex #\"^/tmp/[0-9]+$\"))\n"
        );
    }

    #[test]
    fn mach_lookup_global_name() {
        assert_eq!(
            rs().allow()
                .mach_lookup()
                .global_name("com.apple.example")
                .to_sbdl(),
            "(allow mach-lookup (global-name \"com.apple.example\"))\n"
        );
    }

    #[test]
    fn mach_register_local_name_regex() {
        let re = Regex::new("^foo$").unwrap();
        assert_eq!(
            rs().deny().mach_register().local_name_regex(re).to_sbdl(),
            "(deny mach-register (local-name-regex #\"^foo$\"))\n"
        );
    }

    #[test]
    fn mach_bare_bootstrap() {
        assert_eq!(
            rs().allow().mach_bootstrap().to_sbdl(),
            "(allow mach-bootstrap)\n"
        );
    }

    #[test]
    fn ipc_posix_sem_name() {
        assert_eq!(
            rs().allow().ipc_posix_sem().name("mysem").to_sbdl(),
            "(allow ipc-posix-sem* (ipc-posix-name \"mysem\"))\n"
        );
    }

    #[test]
    fn ipc_posix_shm_regex() {
        let re = Regex::new("^/com\\.app\\.").unwrap();
        assert_eq!(
            rs().allow().ipc_posix_shm().regex(re).to_sbdl(),
            "(allow ipc-posix-shm* (ipc-posix-name-regex #\"^/com\\.app\\.\"))\n"
        );
    }

    #[test]
    fn sysctl_read_name() {
        assert_eq!(
            rs().allow().sysctl_read().name("kern.ostype").to_sbdl(),
            "(allow sysctl-read (sysctl-name \"kern.ostype\"))\n"
        );
    }

    #[test]
    fn iokit_open_user_client_class() {
        assert_eq!(
            rs().allow()
                .iokit_open()
                .user_client_class("IOSurfaceRootUserClient")
                .to_sbdl(),
            "(allow iokit-open (iokit-user-client-class \"IOSurfaceRootUserClient\"))\n"
        );
    }

    #[test]
    fn iokit_property_regex() {
        let re = Regex::new("^Built").unwrap();
        assert_eq!(
            rs().allow()
                .iokit_set_properties()
                .property_regex(re)
                .to_sbdl(),
            "(allow iokit-set-properties (iokit-property-regex #\"^Built\"))\n"
        );
    }

    #[test]
    fn network_outbound_remote() {
        assert_eq!(
            rs().allow()
                .network_outbound()
                .remote(Proto::Tcp, "*:443")
                .to_sbdl(),
            "(allow network-outbound (remote tcp \"*:443\"))\n"
        );
    }

    #[test]
    fn network_outbound_any() {
        assert_eq!(
            rs().allow().network_outbound().any().to_sbdl(),
            "(allow network-outbound)\n"
        );
    }

    #[test]
    fn network_inbound_local() {
        assert_eq!(
            rs().deny()
                .network_inbound()
                .local(Proto::Udp, "localhost:5353")
                .to_sbdl(),
            "(deny network-inbound (local udp \"localhost:5353\"))\n"
        );
    }

    #[test]
    fn signal_self_target() {
        assert_eq!(
            rs().allow().signal().self_target().to_sbdl(),
            "(allow signal (target self))\n"
        );
    }

    #[test]
    fn signal_others() {
        assert_eq!(
            rs().deny().signal().others().to_sbdl(),
            "(deny signal (target others))\n"
        );
    }

    #[test]
    fn file_any() {
        assert_eq!(
            rs().allow().file_read().any().to_sbdl(),
            "(allow file-read*)\n"
        );
    }

    #[test]
    fn mach_any() {
        assert_eq!(
            rs().allow().mach_lookup().any().to_sbdl(),
            "(allow mach-lookup)\n"
        );
    }

    #[test]
    fn ipc_any() {
        assert_eq!(
            rs().allow().ipc_posix_shm().any().to_sbdl(),
            "(allow ipc-posix-shm*)\n"
        );
    }

    #[test]
    fn sysctl_any() {
        assert_eq!(
            rs().allow().sysctl_read().any().to_sbdl(),
            "(allow sysctl-read)\n"
        );
    }

    #[test]
    fn iokit_any() {
        assert_eq!(
            rs().allow().iokit_open().any().to_sbdl(),
            "(allow iokit-open)\n"
        );
    }

    #[test]
    fn signal_any() {
        assert_eq!(rs().allow().signal().any().to_sbdl(), "(allow signal)\n");
    }

    #[test]
    fn chained_rules() {
        assert_eq!(
            rs().allow()
                .file_read()
                .subpath("/")
                .deny()
                .file_write()
                .subpath("/etc")
                .allow()
                .mach_bootstrap()
                .to_sbdl(),
            "(allow file-read* (subpath \"/\"))\n\
             (deny file-write* (subpath \"/etc\"))\n\
             (allow mach-bootstrap)\n"
        );
    }
}
