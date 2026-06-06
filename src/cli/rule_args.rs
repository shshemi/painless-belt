use std::path::PathBuf;
use std::str::FromStr;

use clap::Args;
use regex::Regex;

use crate::sandbox::rule_set::{Proto, RuleSet};

/// A `<proto>:<address>` value for network `local`/`remote` filters,
/// e.g. `tcp:*:443` (proto=tcp, address=`*:443`).
#[derive(Debug, Clone)]
pub struct NetSpec {
    proto: Proto,
    addr: String,
}

#[derive(Debug)]
pub struct NetSpecError(String);

impl std::fmt::Display for NetSpecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for NetSpecError {}

impl FromStr for NetSpec {
    type Err = NetSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (proto, addr) = s
            .split_once(':')
            .ok_or_else(|| NetSpecError("expected <proto>:<address>".into()))?;
        let proto = match proto {
            "ip" => Proto::Ip,
            "tcp" => Proto::Tcp,
            "udp" => Proto::Udp,
            other => {
                return Err(NetSpecError(format!(
                    "unknown proto '{other}' (ip|tcp|udp)"
                )));
            }
        };
        Ok(NetSpec {
            proto,
            addr: addr.to_owned(),
        })
    }
}

// ── path-filter ops (file-*): literal/prefix/subpath + regex + bare ──
macro_rules! path_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        pastey::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                $(
                    #[arg(long)] pub [<allow_ $op _literal>]: Vec<PathBuf>,
                    #[arg(long)] pub [<allow_ $op _prefix>]: Vec<PathBuf>,
                    #[arg(long)] pub [<allow_ $op _subpath>]: Vec<PathBuf>,
                    #[arg(long)] pub [<allow_ $op _regex>]: Vec<Regex>,
                    #[arg(long)] pub [<allow_ $op>]: bool,
                    #[arg(long)] pub [<deny_ $op _literal>]: Vec<PathBuf>,
                    #[arg(long)] pub [<deny_ $op _prefix>]: Vec<PathBuf>,
                    #[arg(long)] pub [<deny_ $op _subpath>]: Vec<PathBuf>,
                    #[arg(long)] pub [<deny_ $op _regex>]: Vec<Regex>,
                    #[arg(long)] pub [<deny_ $op>]: bool,
                )+
            }

            impl $struct {
                pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
                    $(
                        for v in &self.[<allow_ $op _literal>] { rs = rs.allow().$op().literal(v); }
                        for v in &self.[<allow_ $op _prefix>] { rs = rs.allow().$op().prefix(v); }
                        for v in &self.[<allow_ $op _subpath>] { rs = rs.allow().$op().subpath(v); }
                        for v in &self.[<allow_ $op _regex>] { rs = rs.allow().$op().regex(v.clone()); }
                        if self.[<allow_ $op>] { rs = rs.allow().$op().any(); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _literal>] { rs = rs.deny().$op().literal(v); }
                        for v in &self.[<deny_ $op _prefix>] { rs = rs.deny().$op().prefix(v); }
                        for v in &self.[<deny_ $op _subpath>] { rs = rs.deny().$op().subpath(v); }
                        for v in &self.[<deny_ $op _regex>] { rs = rs.deny().$op().regex(v.clone()); }
                        if self.[<deny_ $op>] { rs = rs.deny().$op().any(); }
                    )+
                    rs
                }
            }
        }
    };
}

// ── mach filtered ops (mach-lookup, mach-register) ──
macro_rules! mach_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        pastey::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                $(
                    #[arg(long)] pub [<allow_ $op _global_name>]: Vec<String>,
                    #[arg(long)] pub [<allow_ $op _local_name>]: Vec<String>,
                    #[arg(long)] pub [<allow_ $op _global_name_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<allow_ $op _local_name_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<allow_ $op>]: bool,
                    #[arg(long)] pub [<deny_ $op _global_name>]: Vec<String>,
                    #[arg(long)] pub [<deny_ $op _local_name>]: Vec<String>,
                    #[arg(long)] pub [<deny_ $op _global_name_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<deny_ $op _local_name_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<deny_ $op>]: bool,
                )+
            }

            impl $struct {
                pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
                    $(
                        for v in &self.[<allow_ $op _global_name>] { rs = rs.allow().$op().global_name(v); }
                        for v in &self.[<allow_ $op _local_name>] { rs = rs.allow().$op().local_name(v); }
                        for v in &self.[<allow_ $op _global_name_regex>] { rs = rs.allow().$op().global_name_regex(v.clone()); }
                        for v in &self.[<allow_ $op _local_name_regex>] { rs = rs.allow().$op().local_name_regex(v.clone()); }
                        if self.[<allow_ $op>] { rs = rs.allow().$op().any(); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _global_name>] { rs = rs.deny().$op().global_name(v); }
                        for v in &self.[<deny_ $op _local_name>] { rs = rs.deny().$op().local_name(v); }
                        for v in &self.[<deny_ $op _global_name_regex>] { rs = rs.deny().$op().global_name_regex(v.clone()); }
                        for v in &self.[<deny_ $op _local_name_regex>] { rs = rs.deny().$op().local_name_regex(v.clone()); }
                        if self.[<deny_ $op>] { rs = rs.deny().$op().any(); }
                    )+
                    rs
                }
            }
        }
    };
}

// ── name-filter ops (ipc, sysctl): name + regex + bare ──
macro_rules! name_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        pastey::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                $(
                    #[arg(long)] pub [<allow_ $op _name>]: Vec<String>,
                    #[arg(long)] pub [<allow_ $op _regex>]: Vec<Regex>,
                    #[arg(long)] pub [<allow_ $op>]: bool,
                    #[arg(long)] pub [<deny_ $op _name>]: Vec<String>,
                    #[arg(long)] pub [<deny_ $op _regex>]: Vec<Regex>,
                    #[arg(long)] pub [<deny_ $op>]: bool,
                )+
            }

            impl $struct {
                pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
                    $(
                        for v in &self.[<allow_ $op _name>] { rs = rs.allow().$op().name(v); }
                        for v in &self.[<allow_ $op _regex>] { rs = rs.allow().$op().regex(v.clone()); }
                        if self.[<allow_ $op>] { rs = rs.allow().$op().any(); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _name>] { rs = rs.deny().$op().name(v); }
                        for v in &self.[<deny_ $op _regex>] { rs = rs.deny().$op().regex(v.clone()); }
                        if self.[<deny_ $op>] { rs = rs.deny().$op().any(); }
                    )+
                    rs
                }
            }
        }
    };
}

// ── iokit ops: user-client-class/property (+regex) + bare ──
macro_rules! iokit_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        pastey::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                $(
                    #[arg(long)] pub [<allow_ $op _user_client_class>]: Vec<String>,
                    #[arg(long)] pub [<allow_ $op _user_client_class_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<allow_ $op _property>]: Vec<String>,
                    #[arg(long)] pub [<allow_ $op _property_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<allow_ $op>]: bool,
                    #[arg(long)] pub [<deny_ $op _user_client_class>]: Vec<String>,
                    #[arg(long)] pub [<deny_ $op _user_client_class_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<deny_ $op _property>]: Vec<String>,
                    #[arg(long)] pub [<deny_ $op _property_regex>]: Vec<Regex>,
                    #[arg(long)] pub [<deny_ $op>]: bool,
                )+
            }

            impl $struct {
                pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
                    $(
                        for v in &self.[<allow_ $op _user_client_class>] { rs = rs.allow().$op().user_client_class(v); }
                        for v in &self.[<allow_ $op _user_client_class_regex>] { rs = rs.allow().$op().user_client_class_regex(v.clone()); }
                        for v in &self.[<allow_ $op _property>] { rs = rs.allow().$op().property(v); }
                        for v in &self.[<allow_ $op _property_regex>] { rs = rs.allow().$op().property_regex(v.clone()); }
                        if self.[<allow_ $op>] { rs = rs.allow().$op().any(); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _user_client_class>] { rs = rs.deny().$op().user_client_class(v); }
                        for v in &self.[<deny_ $op _user_client_class_regex>] { rs = rs.deny().$op().user_client_class_regex(v.clone()); }
                        for v in &self.[<deny_ $op _property>] { rs = rs.deny().$op().property(v); }
                        for v in &self.[<deny_ $op _property_regex>] { rs = rs.deny().$op().property_regex(v.clone()); }
                        if self.[<deny_ $op>] { rs = rs.deny().$op().any(); }
                    )+
                    rs
                }
            }
        }
    };
}

// ── network ops: local/remote (proto:addr) + bare ──
macro_rules! network_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        pastey::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                $(
                    #[arg(long)] pub [<allow_ $op _local>]: Vec<NetSpec>,
                    #[arg(long)] pub [<allow_ $op _remote>]: Vec<NetSpec>,
                    #[arg(long)] pub [<allow_ $op>]: bool,
                    #[arg(long)] pub [<deny_ $op _local>]: Vec<NetSpec>,
                    #[arg(long)] pub [<deny_ $op _remote>]: Vec<NetSpec>,
                    #[arg(long)] pub [<deny_ $op>]: bool,
                )+
            }

            impl $struct {
                pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
                    $(
                        for s in &self.[<allow_ $op _local>] { rs = rs.allow().$op().local(s.proto.clone(), &s.addr); }
                        for s in &self.[<allow_ $op _remote>] { rs = rs.allow().$op().remote(s.proto.clone(), &s.addr); }
                        if self.[<allow_ $op>] { rs = rs.allow().$op().any(); }
                    )+
                    $(
                        for s in &self.[<deny_ $op _local>] { rs = rs.deny().$op().local(s.proto.clone(), &s.addr); }
                        for s in &self.[<deny_ $op _remote>] { rs = rs.deny().$op().remote(s.proto.clone(), &s.addr); }
                        if self.[<deny_ $op>] { rs = rs.deny().$op().any(); }
                    )+
                    rs
                }
            }
        }
    };
}

// ── signal: self-target/others + bare ──
macro_rules! signal_rule_args {
    ($struct:ident, $heading:literal, $op:ident) => {
        pastey::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                #[arg(long)] pub [<allow_ $op _self_target>]: bool,
                #[arg(long)] pub [<allow_ $op _others>]: bool,
                #[arg(long)] pub [<allow_ $op>]: bool,
                #[arg(long)] pub [<deny_ $op _self_target>]: bool,
                #[arg(long)] pub [<deny_ $op _others>]: bool,
                #[arg(long)] pub [<deny_ $op>]: bool,
            }

            impl $struct {
                pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
                    if self.[<allow_ $op _self_target>] { rs = rs.allow().$op().self_target(); }
                    if self.[<allow_ $op _others>] { rs = rs.allow().$op().others(); }
                    if self.[<allow_ $op>] { rs = rs.allow().$op().any(); }
                    if self.[<deny_ $op _self_target>] { rs = rs.deny().$op().self_target(); }
                    if self.[<deny_ $op _others>] { rs = rs.deny().$op().others(); }
                    if self.[<deny_ $op>] { rs = rs.deny().$op().any(); }
                    rs
                }
            }
        }
    };
}

// ── bare mach ops (no filter) ──
macro_rules! bare_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        pastey::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                $(
                    #[arg(long)] pub [<allow_ $op>]: bool,
                    #[arg(long)] pub [<deny_ $op>]: bool,
                )+
            }

            impl $struct {
                pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
                    $( if self.[<allow_ $op>] { rs = rs.allow().$op(); } )+
                    $( if self.[<deny_ $op>] { rs = rs.deny().$op(); } )+
                    rs
                }
            }
        }
    };
}

path_rule_args!(
    FileRuleArgs,
    "File rules",
    file_chroot,
    file_fsctl,
    file_ioctl,
    file_issue_extension,
    file_link,
    file_map_executable,
    file_mknod,
    file_read,
    file_read_data,
    file_read_metadata,
    file_read_xattr,
    file_revoke,
    file_search,
    file_write,
    file_write_create,
    file_write_data,
    file_write_flags,
    file_write_mode,
    file_write_mount,
    file_write_owner,
    file_write_setugid,
    file_write_times,
    file_write_umount,
    file_write_unlink,
    file_write_xattr,
);

mach_rule_args!(MachRuleArgs, "Mach rules", mach_lookup, mach_register);

bare_rule_args!(
    MachBareRuleArgs,
    "Mach (bare) rules",
    mach_bootstrap,
    mach_cross_domain_lookup,
    mach_derive_port,
    mach_host_special_port_set,
    mach_issue_extension,
    mach_per_user_lookup,
    mach_priv_host_port,
    mach_priv_task_port,
    mach_task_name,
);

name_rule_args!(
    IpcSysctlRuleArgs,
    "IPC / sysctl rules",
    ipc_posix_sem,
    ipc_posix_sem_create,
    ipc_posix_sem_open,
    ipc_posix_sem_post,
    ipc_posix_sem_unlink,
    ipc_posix_sem_wait,
    ipc_posix_shm,
    ipc_posix_shm_read,
    ipc_posix_shm_read_data,
    ipc_posix_shm_read_metadata,
    ipc_posix_shm_write,
    ipc_posix_shm_write_create,
    ipc_posix_shm_write_data,
    ipc_posix_shm_write_unlink,
    sysctl_read,
    sysctl_write,
);

iokit_rule_args!(
    IokitRuleArgs,
    "IOKit rules",
    iokit_open,
    iokit_set_properties
);

network_rule_args!(
    NetworkRuleArgs,
    "Network rules",
    network,
    network_bind,
    network_inbound,
    network_outbound,
);

signal_rule_args!(SignalRuleArgs, "Signal rules", signal);

/// Convenience shortcuts that open up running system binaries and loading
/// system shared libraries.
#[derive(Args, Debug, Default)]
#[command(next_help_heading = "System shortcuts")]
pub struct SystemRuleArgs {
    /// Allow executing binaries found in $PATH (reads PATH dirs + system
    /// library paths, plus process-exec / process-fork / file-map-executable).
    #[arg(long)]
    pub allow_system_binaries: bool,
    /// Allow loading system shared libraries (reads /usr/lib, /System,
    /// /Library, /opt, /usr/share, /, plus file-map-executable).
    #[arg(long)]
    pub allow_system_libraries: bool,
}

impl SystemRuleArgs {
    pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
        // System binaries imply system libraries (dyld needs the libs to run
        // the binary).
        let want_libs = self.allow_system_binaries || self.allow_system_libraries;
        if want_libs {
            rs = rs.allow().file_read().literal("/");
            for dir in ["/usr/lib", "/usr/share", "/System", "/Library", "/opt"] {
                rs = rs.allow().file_read().subpath(dir);
            }
            rs = rs.allow().file_map_executable().any();
        }
        if self.allow_system_binaries {
            for dir in std::env::var("PATH")
                .unwrap_or_default()
                .split(':')
                .filter(|s| !s.is_empty())
            {
                rs = rs.allow().file_read().subpath(dir);
            }
            rs = rs.allow().process_exec().any();
            rs = rs.allow().process_fork();
        }
        rs
    }
}

/// Convenience shortcuts that target the user's home directory.
#[derive(Args, Debug, Default)]
#[command(next_help_heading = "Home shortcuts")]
pub struct HomeRuleArgs {
    /// Allow reading the home directory.
    #[arg(long)]
    pub allow_read_home: bool,
    /// Allow writing the home directory.
    #[arg(long)]
    pub allow_write_home: bool,
    /// Allow reading and writing the home directory.
    #[arg(long)]
    pub allow_readwrite_home: bool,
    /// Deny reading the home directory.
    #[arg(long)]
    pub deny_read_home: bool,
    /// Deny writing the home directory.
    #[arg(long)]
    pub deny_write_home: bool,
    /// Deny reading and writing the home directory.
    #[arg(long)]
    pub deny_readwrite_home: bool,
}

impl HomeRuleArgs {
    pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
        let Some(home) = dirs::home_dir() else {
            return rs;
        };
        if self.allow_read_home || self.allow_readwrite_home {
            rs = rs.allow().file_read().subpath(&home);
        }
        if self.allow_write_home || self.allow_readwrite_home {
            rs = rs.allow().file_write().subpath(&home);
        }
        if self.deny_read_home || self.deny_readwrite_home {
            rs = rs.deny().file_read().subpath(&home);
        }
        if self.deny_write_home || self.deny_readwrite_home {
            rs = rs.deny().file_write().subpath(&home);
        }
        rs
    }
}

/// Convenience shortcuts that target ~/Library/Keychains.
#[derive(Args, Debug, Default)]
#[command(next_help_heading = "Keychain shortcuts")]
pub struct KeychainRuleArgs {
    /// Allow reading ~/Library/Keychains.
    #[arg(long)]
    pub allow_read_keychain: bool,
    /// Allow writing ~/Library/Keychains.
    #[arg(long)]
    pub allow_write_keychain: bool,
    /// Allow reading and writing ~/Library/Keychains.
    #[arg(long)]
    pub allow_readwrite_keychain: bool,
    /// Deny reading ~/Library/Keychains.
    #[arg(long)]
    pub deny_read_keychain: bool,
    /// Deny writing ~/Library/Keychains.
    #[arg(long)]
    pub deny_write_keychain: bool,
    /// Deny reading and writing ~/Library/Keychains.
    #[arg(long)]
    pub deny_readwrite_keychain: bool,
}

impl KeychainRuleArgs {
    pub fn apply_to(&self, mut rs: RuleSet) -> RuleSet {
        let Some(home) = dirs::home_dir() else {
            return rs;
        };
        let path = home.join("Library/Keychains");
        if self.allow_read_keychain || self.allow_readwrite_keychain {
            rs = rs.allow().file_read().subpath(&path);
        }
        if self.allow_write_keychain || self.allow_readwrite_keychain {
            rs = rs.allow().file_write().subpath(&path);
        }
        if self.deny_read_keychain || self.deny_readwrite_keychain {
            rs = rs.deny().file_read().subpath(&path);
        }
        if self.deny_write_keychain || self.deny_readwrite_keychain {
            rs = rs.deny().file_write().subpath(&path);
        }
        rs
    }
}

/// All sandbox-rule flag groups, flattened into the run command.
#[derive(Args, Debug, Default)]
pub struct RuleArgs {
    #[command(flatten)]
    pub file: FileRuleArgs,
    #[command(flatten)]
    pub mach: MachRuleArgs,
    #[command(flatten)]
    pub mach_bare: MachBareRuleArgs,
    #[command(flatten)]
    pub ipc_sysctl: IpcSysctlRuleArgs,
    #[command(flatten)]
    pub iokit: IokitRuleArgs,
    #[command(flatten)]
    pub network: NetworkRuleArgs,
    #[command(flatten)]
    pub signal: SignalRuleArgs,
    #[command(flatten)]
    pub system: SystemRuleArgs,
    #[command(flatten)]
    pub home: HomeRuleArgs,
    #[command(flatten)]
    pub keychain: KeychainRuleArgs,
}

impl RuleArgs {
    /// Render all set flags into a `RuleSet` (allow rules then deny rules
    /// per group, so denies win under SBPL's last-match-wins evaluation).
    pub fn rule_set(&self) -> RuleSet {
        let mut rs = RuleSet::default();
        rs = self.file.apply_to(rs);
        rs = self.mach.apply_to(rs);
        rs = self.mach_bare.apply_to(rs);
        rs = self.ipc_sysctl.apply_to(rs);
        rs = self.iokit.apply_to(rs);
        rs = self.network.apply_to(rs);
        rs = self.signal.apply_to(rs);
        rs = self.system.apply_to(rs);
        rs = self.home.apply_to(rs);
        rs = self.keychain.apply_to(rs);
        rs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netspec_parses_proto_and_keeps_address() {
        let s: NetSpec = "tcp:*:443".parse().unwrap();
        assert!(matches!(s.proto, Proto::Tcp));
        assert_eq!(s.addr, "*:443");
    }

    #[test]
    fn netspec_rejects_unknown_proto() {
        assert!("bogus:host".parse::<NetSpec>().is_err());
    }

    #[test]
    fn netspec_rejects_missing_colon() {
        assert!("tcp".parse::<NetSpec>().is_err());
    }

    #[test]
    fn keychain_readwrite_emits_allow_read_and_write() {
        let args = KeychainRuleArgs {
            allow_readwrite_keychain: true,
            ..Default::default()
        };
        let rs = args.apply_to(RuleSet::default());
        let home = dirs::home_dir().unwrap();
        let kc = home.join("Library/Keychains").display().to_string();
        assert_eq!(
            rs.to_sbpl(),
            format!(
                "(allow file-read* (subpath \"{kc}\"))\n\
                 (allow file-write* (subpath \"{kc}\"))\n"
            )
        );
    }

    #[test]
    fn system_libraries_emits_lib_paths_and_map_executable() {
        let args = SystemRuleArgs {
            allow_system_libraries: true,
            ..Default::default()
        };
        let rs = args.apply_to(RuleSet::default());
        assert_eq!(
            rs.to_sbpl(),
            "(allow file-read* (literal \"/\"))\n\
             (allow file-read* (subpath \"/usr/lib\"))\n\
             (allow file-read* (subpath \"/usr/share\"))\n\
             (allow file-read* (subpath \"/System\"))\n\
             (allow file-read* (subpath \"/Library\"))\n\
             (allow file-read* (subpath \"/opt\"))\n\
             (allow file-map-executable)\n"
        );
    }

    #[test]
    fn system_binaries_implies_libraries_and_adds_exec_fork() {
        let args = SystemRuleArgs {
            allow_system_binaries: true,
            ..Default::default()
        };
        let sbpl = args.apply_to(RuleSet::default()).to_sbpl().to_string();
        assert!(sbpl.contains("(allow file-read* (subpath \"/usr/lib\"))"));
        assert!(sbpl.contains("(allow file-map-executable)"));
        assert!(sbpl.contains("(allow process-exec)"));
        assert!(sbpl.contains("(allow process-fork)"));
    }

    #[test]
    fn home_readwrite_emits_allow_read_and_write() {
        let args = HomeRuleArgs {
            allow_readwrite_home: true,
            ..Default::default()
        };
        let rs = args.apply_to(RuleSet::default());
        let home = dirs::home_dir().unwrap().display().to_string();
        assert_eq!(
            rs.to_sbpl(),
            format!(
                "(allow file-read* (subpath \"{home}\"))\n\
                 (allow file-write* (subpath \"{home}\"))\n"
            )
        );
    }

    #[test]
    fn home_allow_then_keychain_deny_ordering() {
        // Home should emit before keychain so the narrower keychain deny
        // wins over a broad home allow.
        let args = RuleArgs {
            home: HomeRuleArgs {
                allow_read_home: true,
                ..Default::default()
            },
            keychain: KeychainRuleArgs {
                deny_read_keychain: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let rs = args.rule_set();
        let home = dirs::home_dir().unwrap();
        let h = home.display().to_string();
        let kc = home.join("Library/Keychains").display().to_string();
        assert_eq!(
            rs.to_sbpl(),
            format!(
                "(allow file-read* (subpath \"{h}\"))\n\
                 (deny file-read* (subpath \"{kc}\"))\n"
            )
        );
    }

    #[test]
    fn keychain_deny_read_emits_deny_only() {
        let args = KeychainRuleArgs {
            deny_read_keychain: true,
            ..Default::default()
        };
        let rs = args.apply_to(RuleSet::default());
        let home = dirs::home_dir().unwrap();
        let kc = home.join("Library/Keychains").display().to_string();
        assert_eq!(
            rs.to_sbpl(),
            format!("(deny file-read* (subpath \"{kc}\"))\n")
        );
    }

    use crate::sandbox::ToSbpl;

    #[test]
    fn file_rules_emit_allow_before_deny() {
        let args = FileRuleArgs {
            allow_file_read_subpath: vec![PathBuf::from("/tmp")],
            deny_file_read_subpath: vec![PathBuf::from("/tmp/secret")],
            ..Default::default()
        };
        let rs = args.apply_to(RuleSet::default());
        assert_eq!(
            rs.to_sbpl(),
            "(allow file-read* (subpath \"/tmp\"))\n\
             (deny file-read* (subpath \"/tmp/secret\"))\n"
        );
    }

    #[test]
    fn bare_any_emits_unfiltered_rule() {
        let args = FileRuleArgs {
            allow_file_read: true,
            ..Default::default()
        };
        let rs = args.apply_to(RuleSet::default());
        assert_eq!(rs.to_sbpl(), "(allow file-read*)\n");
    }

    #[test]
    fn network_remote_emits_proto_and_addr() {
        let args = NetworkRuleArgs {
            allow_network_outbound_remote: vec!["tcp:*:443".parse().unwrap()],
            ..Default::default()
        };
        let rs = args.apply_to(RuleSet::default());
        assert_eq!(
            rs.to_sbpl(),
            "(allow network-outbound (remote tcp \"*:443\"))\n"
        );
    }
}
