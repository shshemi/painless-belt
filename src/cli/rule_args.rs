use std::path::PathBuf;
use std::str::FromStr;

use clap::Args;
use regex::Regex;

use crate::sandbox::ToSbdl;
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
        paste::paste! {
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
                pub fn apply_to(&self, out: &mut String) {
                    $(
                        for v in &self.[<allow_ $op _literal>] { out.push_str(RuleSet::default().allow().$op().literal(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _prefix>] { out.push_str(RuleSet::default().allow().$op().prefix(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _subpath>] { out.push_str(RuleSet::default().allow().$op().subpath(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _regex>] { out.push_str(RuleSet::default().allow().$op().regex(v.clone()).to_sbdl()); }
                        if self.[<allow_ $op>] { out.push_str(RuleSet::default().allow().$op().any().to_sbdl()); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _literal>] { out.push_str(RuleSet::default().deny().$op().literal(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _prefix>] { out.push_str(RuleSet::default().deny().$op().prefix(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _subpath>] { out.push_str(RuleSet::default().deny().$op().subpath(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _regex>] { out.push_str(RuleSet::default().deny().$op().regex(v.clone()).to_sbdl()); }
                        if self.[<deny_ $op>] { out.push_str(RuleSet::default().deny().$op().any().to_sbdl()); }
                    )+
                }
            }
        }
    };
}

// ── mach filtered ops (mach-lookup, mach-register) ──
macro_rules! mach_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        paste::paste! {
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
                pub fn apply_to(&self, out: &mut String) {
                    $(
                        for v in &self.[<allow_ $op _global_name>] { out.push_str(RuleSet::default().allow().$op().global_name(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _local_name>] { out.push_str(RuleSet::default().allow().$op().local_name(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _global_name_regex>] { out.push_str(RuleSet::default().allow().$op().global_name_regex(v.clone()).to_sbdl()); }
                        for v in &self.[<allow_ $op _local_name_regex>] { out.push_str(RuleSet::default().allow().$op().local_name_regex(v.clone()).to_sbdl()); }
                        if self.[<allow_ $op>] { out.push_str(RuleSet::default().allow().$op().any().to_sbdl()); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _global_name>] { out.push_str(RuleSet::default().deny().$op().global_name(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _local_name>] { out.push_str(RuleSet::default().deny().$op().local_name(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _global_name_regex>] { out.push_str(RuleSet::default().deny().$op().global_name_regex(v.clone()).to_sbdl()); }
                        for v in &self.[<deny_ $op _local_name_regex>] { out.push_str(RuleSet::default().deny().$op().local_name_regex(v.clone()).to_sbdl()); }
                        if self.[<deny_ $op>] { out.push_str(RuleSet::default().deny().$op().any().to_sbdl()); }
                    )+
                }
            }
        }
    };
}

// ── name-filter ops (ipc, sysctl): name + regex + bare ──
macro_rules! name_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        paste::paste! {
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
                pub fn apply_to(&self, out: &mut String) {
                    $(
                        for v in &self.[<allow_ $op _name>] { out.push_str(RuleSet::default().allow().$op().name(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _regex>] { out.push_str(RuleSet::default().allow().$op().regex(v.clone()).to_sbdl()); }
                        if self.[<allow_ $op>] { out.push_str(RuleSet::default().allow().$op().any().to_sbdl()); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _name>] { out.push_str(RuleSet::default().deny().$op().name(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _regex>] { out.push_str(RuleSet::default().deny().$op().regex(v.clone()).to_sbdl()); }
                        if self.[<deny_ $op>] { out.push_str(RuleSet::default().deny().$op().any().to_sbdl()); }
                    )+
                }
            }
        }
    };
}

// ── iokit ops: user-client-class/property (+regex) + bare ──
macro_rules! iokit_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        paste::paste! {
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
                pub fn apply_to(&self, out: &mut String) {
                    $(
                        for v in &self.[<allow_ $op _user_client_class>] { out.push_str(RuleSet::default().allow().$op().user_client_class(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _user_client_class_regex>] { out.push_str(RuleSet::default().allow().$op().user_client_class_regex(v.clone()).to_sbdl()); }
                        for v in &self.[<allow_ $op _property>] { out.push_str(RuleSet::default().allow().$op().property(v).to_sbdl()); }
                        for v in &self.[<allow_ $op _property_regex>] { out.push_str(RuleSet::default().allow().$op().property_regex(v.clone()).to_sbdl()); }
                        if self.[<allow_ $op>] { out.push_str(RuleSet::default().allow().$op().any().to_sbdl()); }
                    )+
                    $(
                        for v in &self.[<deny_ $op _user_client_class>] { out.push_str(RuleSet::default().deny().$op().user_client_class(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _user_client_class_regex>] { out.push_str(RuleSet::default().deny().$op().user_client_class_regex(v.clone()).to_sbdl()); }
                        for v in &self.[<deny_ $op _property>] { out.push_str(RuleSet::default().deny().$op().property(v).to_sbdl()); }
                        for v in &self.[<deny_ $op _property_regex>] { out.push_str(RuleSet::default().deny().$op().property_regex(v.clone()).to_sbdl()); }
                        if self.[<deny_ $op>] { out.push_str(RuleSet::default().deny().$op().any().to_sbdl()); }
                    )+
                }
            }
        }
    };
}

// ── network ops: local/remote (proto:addr) + bare ──
macro_rules! network_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        paste::paste! {
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
                pub fn apply_to(&self, out: &mut String) {
                    $(
                        for s in &self.[<allow_ $op _local>] { out.push_str(RuleSet::default().allow().$op().local(s.proto.clone(), &s.addr).to_sbdl()); }
                        for s in &self.[<allow_ $op _remote>] { out.push_str(RuleSet::default().allow().$op().remote(s.proto.clone(), &s.addr).to_sbdl()); }
                        if self.[<allow_ $op>] { out.push_str(RuleSet::default().allow().$op().any().to_sbdl()); }
                    )+
                    $(
                        for s in &self.[<deny_ $op _local>] { out.push_str(RuleSet::default().deny().$op().local(s.proto.clone(), &s.addr).to_sbdl()); }
                        for s in &self.[<deny_ $op _remote>] { out.push_str(RuleSet::default().deny().$op().remote(s.proto.clone(), &s.addr).to_sbdl()); }
                        if self.[<deny_ $op>] { out.push_str(RuleSet::default().deny().$op().any().to_sbdl()); }
                    )+
                }
            }
        }
    };
}

// ── signal: self-target/others + bare ──
macro_rules! signal_rule_args {
    ($struct:ident, $heading:literal, $op:ident) => {
        paste::paste! {
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
                pub fn apply_to(&self, out: &mut String) {
                    if self.[<allow_ $op _self_target>] { out.push_str(RuleSet::default().allow().$op().self_target().to_sbdl()); }
                    if self.[<allow_ $op _others>] { out.push_str(RuleSet::default().allow().$op().others().to_sbdl()); }
                    if self.[<allow_ $op>] { out.push_str(RuleSet::default().allow().$op().any().to_sbdl()); }
                    if self.[<deny_ $op _self_target>] { out.push_str(RuleSet::default().deny().$op().self_target().to_sbdl()); }
                    if self.[<deny_ $op _others>] { out.push_str(RuleSet::default().deny().$op().others().to_sbdl()); }
                    if self.[<deny_ $op>] { out.push_str(RuleSet::default().deny().$op().any().to_sbdl()); }
                }
            }
        }
    };
}

// ── bare mach ops (no filter) ──
macro_rules! bare_rule_args {
    ($struct:ident, $heading:literal, $($op:ident),+ $(,)?) => {
        paste::paste! {
            #[derive(Args, Debug, Default)]
            #[command(next_help_heading = $heading)]
            pub struct $struct {
                $(
                    #[arg(long)] pub [<allow_ $op>]: bool,
                    #[arg(long)] pub [<deny_ $op>]: bool,
                )+
            }

            impl $struct {
                pub fn apply_to(&self, out: &mut String) {
                    $( if self.[<allow_ $op>] { out.push_str(RuleSet::default().allow().$op().to_sbdl()); } )+
                    $( if self.[<deny_ $op>] { out.push_str(RuleSet::default().deny().$op().to_sbdl()); } )+
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
}

impl RuleArgs {
    /// Render all set flags into SBPL rule lines (allow rules then deny rules
    /// per group, so denies win under SBPL's last-match-wins evaluation).
    pub fn to_sbpl(&self) -> String {
        let mut out = String::new();
        self.file.apply_to(&mut out);
        self.mach.apply_to(&mut out);
        self.mach_bare.apply_to(&mut out);
        self.ipc_sysctl.apply_to(&mut out);
        self.iokit.apply_to(&mut out);
        self.network.apply_to(&mut out);
        self.signal.apply_to(&mut out);
        out
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
    fn file_rules_emit_allow_before_deny() {
        let args = FileRuleArgs {
            allow_file_read_subpath: vec![PathBuf::from("/tmp")],
            deny_file_read_subpath: vec![PathBuf::from("/tmp/secret")],
            ..Default::default()
        };
        let mut out = String::new();
        args.apply_to(&mut out);
        assert_eq!(
            out,
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
        let mut out = String::new();
        args.apply_to(&mut out);
        assert_eq!(out, "(allow file-read*)\n");
    }

    #[test]
    fn network_remote_emits_proto_and_addr() {
        let args = NetworkRuleArgs {
            allow_network_outbound_remote: vec!["tcp:*:443".parse().unwrap()],
            ..Default::default()
        };
        let mut out = String::new();
        args.apply_to(&mut out);
        assert_eq!(out, "(allow network-outbound (remote tcp \"*:443\"))\n");
    }
}
