mod common;

use common::assert_rules_compile;
use painless_belt::sandbox::rule_set::{Proto, RuleSet};

macro_rules! init_file {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default().allow().$method().subpath("/tmp");
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_mach {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default()
                .allow()
                .$method()
                .global_name("com.apple.example");
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_bare {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default().allow().$method();
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_ipc {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default()
                .allow()
                .$method()
                .name("painless-belt-test");
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_sysctl {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default().allow().$method().name("kern.ostype");
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_iokit {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default()
                .allow()
                .$method()
                .user_client_class("IOSurfaceRootUserClient");
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_network_local {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default()
                .allow()
                .$method()
                .local(Proto::Tcp, "*:*");
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_network_any {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default().allow().$method().any();
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_any {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default().allow().$method().any();
            assert_rules_compile(&rules);
        }
    };
}

macro_rules! init_network_remote {
    ($test:ident, $method:ident) => {
        #[test]
        fn $test() {
            let rules = RuleSet::default()
                .allow()
                .$method()
                .remote(Proto::Tcp, "*:*");
            assert_rules_compile(&rules);
        }
    };
}

// ── file ──
init_file!(init_file_chroot, file_chroot);
init_file!(init_file_fsctl, file_fsctl);
init_file!(init_file_ioctl, file_ioctl);
init_file!(init_file_issue_extension, file_issue_extension);
init_file!(init_file_link, file_link);
init_file!(init_file_map_executable, file_map_executable);
init_file!(init_file_mknod, file_mknod);
init_file!(init_file_read, file_read);
init_file!(init_file_read_data, file_read_data);
init_file!(init_file_read_metadata, file_read_metadata);
init_file!(init_file_read_xattr, file_read_xattr);
init_file!(init_file_revoke, file_revoke);
init_file!(init_file_search, file_search);
init_file!(init_file_write, file_write);
init_file!(init_file_write_create, file_write_create);
init_file!(init_file_write_data, file_write_data);
init_file!(init_file_write_flags, file_write_flags);
init_file!(init_file_write_mode, file_write_mode);
init_file!(init_file_write_mount, file_write_mount);
init_file!(init_file_write_owner, file_write_owner);
init_file!(init_file_write_setugid, file_write_setugid);
init_file!(init_file_write_times, file_write_times);
init_file!(init_file_write_umount, file_write_umount);
init_file!(init_file_write_unlink, file_write_unlink);
init_file!(init_file_write_xattr, file_write_xattr);

// ── mach (filtered) ──
init_mach!(init_mach_lookup, mach_lookup);
init_mach!(init_mach_register, mach_register);

// ── mach (bare) ──
init_bare!(init_mach_bootstrap, mach_bootstrap);
init_bare!(init_mach_cross_domain_lookup, mach_cross_domain_lookup);
init_bare!(init_mach_derive_port, mach_derive_port);
init_bare!(init_mach_host_special_port_set, mach_host_special_port_set);
init_bare!(init_mach_issue_extension, mach_issue_extension);
init_bare!(init_mach_per_user_lookup, mach_per_user_lookup);
init_bare!(init_mach_priv_host_port, mach_priv_host_port);
init_bare!(init_mach_priv_task_port, mach_priv_task_port);
init_bare!(init_mach_task_name, mach_task_name);

// ── ipc ──
init_ipc!(init_ipc_posix_sem, ipc_posix_sem);
init_ipc!(init_ipc_posix_sem_create, ipc_posix_sem_create);
init_ipc!(init_ipc_posix_sem_open, ipc_posix_sem_open);
init_ipc!(init_ipc_posix_sem_post, ipc_posix_sem_post);
init_ipc!(init_ipc_posix_sem_unlink, ipc_posix_sem_unlink);
init_ipc!(init_ipc_posix_sem_wait, ipc_posix_sem_wait);
init_ipc!(init_ipc_posix_shm, ipc_posix_shm);
init_ipc!(init_ipc_posix_shm_read, ipc_posix_shm_read);
init_ipc!(init_ipc_posix_shm_read_data, ipc_posix_shm_read_data);
init_ipc!(
    init_ipc_posix_shm_read_metadata,
    ipc_posix_shm_read_metadata
);
init_ipc!(init_ipc_posix_shm_write, ipc_posix_shm_write);
init_ipc!(init_ipc_posix_shm_write_create, ipc_posix_shm_write_create);
init_ipc!(init_ipc_posix_shm_write_data, ipc_posix_shm_write_data);
init_ipc!(init_ipc_posix_shm_write_unlink, ipc_posix_shm_write_unlink);

// ── sysctl ──
init_sysctl!(init_sysctl_read, sysctl_read);
init_sysctl!(init_sysctl_write, sysctl_write);

// ── iokit ──
init_iokit!(init_iokit_open, iokit_open);
init_iokit!(init_iokit_set_properties, iokit_set_properties);

// ── network ──
init_network_remote!(init_network, network);
init_network_remote!(init_network_outbound, network_outbound);
init_network_local!(init_network_bind, network_bind);
init_network_local!(init_network_inbound, network_inbound);
init_network_any!(init_network_any, network);
init_network_any!(init_network_bind_any, network_bind);
init_network_any!(init_network_inbound_any, network_inbound);
init_network_any!(init_network_outbound_any, network_outbound);

// ── signal ──
#[test]
fn init_signal_self_target() {
    let rules = RuleSet::default().allow().signal().self_target();
    assert_rules_compile(&rules);
}

#[test]
fn init_signal_others() {
    let rules = RuleSet::default().allow().signal().others();
    assert_rules_compile(&rules);
}

// ── .any() on all filtered ops ──
init_any!(init_file_chroot_any, file_chroot);
init_any!(init_file_fsctl_any, file_fsctl);
init_any!(init_file_ioctl_any, file_ioctl);
init_any!(init_file_issue_extension_any, file_issue_extension);
init_any!(init_file_link_any, file_link);
init_any!(init_file_map_executable_any, file_map_executable);
init_any!(init_file_mknod_any, file_mknod);
init_any!(init_file_read_any, file_read);
init_any!(init_file_read_data_any, file_read_data);
init_any!(init_file_read_metadata_any, file_read_metadata);
init_any!(init_file_read_xattr_any, file_read_xattr);
init_any!(init_file_revoke_any, file_revoke);
init_any!(init_file_search_any, file_search);
init_any!(init_file_write_any, file_write);
init_any!(init_file_write_create_any, file_write_create);
init_any!(init_file_write_data_any, file_write_data);
init_any!(init_file_write_flags_any, file_write_flags);
init_any!(init_file_write_mode_any, file_write_mode);
init_any!(init_file_write_mount_any, file_write_mount);
init_any!(init_file_write_owner_any, file_write_owner);
init_any!(init_file_write_setugid_any, file_write_setugid);
init_any!(init_file_write_times_any, file_write_times);
init_any!(init_file_write_umount_any, file_write_umount);
init_any!(init_file_write_unlink_any, file_write_unlink);
init_any!(init_file_write_xattr_any, file_write_xattr);

init_any!(init_mach_lookup_any, mach_lookup);
init_any!(init_mach_register_any, mach_register);

init_any!(init_ipc_posix_sem_any, ipc_posix_sem);
init_any!(init_ipc_posix_sem_create_any, ipc_posix_sem_create);
init_any!(init_ipc_posix_sem_open_any, ipc_posix_sem_open);
init_any!(init_ipc_posix_sem_post_any, ipc_posix_sem_post);
init_any!(init_ipc_posix_sem_unlink_any, ipc_posix_sem_unlink);
init_any!(init_ipc_posix_sem_wait_any, ipc_posix_sem_wait);
init_any!(init_ipc_posix_shm_any, ipc_posix_shm);
init_any!(init_ipc_posix_shm_read_any, ipc_posix_shm_read);
init_any!(init_ipc_posix_shm_read_data_any, ipc_posix_shm_read_data);
init_any!(
    init_ipc_posix_shm_read_metadata_any,
    ipc_posix_shm_read_metadata
);
init_any!(init_ipc_posix_shm_write_any, ipc_posix_shm_write);
init_any!(
    init_ipc_posix_shm_write_create_any,
    ipc_posix_shm_write_create
);
init_any!(init_ipc_posix_shm_write_data_any, ipc_posix_shm_write_data);
init_any!(
    init_ipc_posix_shm_write_unlink_any,
    ipc_posix_shm_write_unlink
);

init_any!(init_sysctl_read_any, sysctl_read);
init_any!(init_sysctl_write_any, sysctl_write);

init_any!(init_iokit_open_any, iokit_open);
init_any!(init_iokit_set_properties_any, iokit_set_properties);

init_any!(init_signal_any, signal);

// process / system-socket
init_file!(init_process_exec, process_exec);
init_any!(init_process_exec_any, process_exec);
init_bare!(init_process_fork, process_fork);
init_bare!(init_system_socket, system_socket);
