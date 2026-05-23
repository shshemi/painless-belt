mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::network::network_bind::NetworkBind;
use painless_belt::sandbox::operations::network::{Host, Port, Proto};
use painless_belt::sandbox::profile::Profile;

#[test]
fn init_local_ip_any() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(NetworkBind::local(Proto::Ip, Host::Any, Port::Any)),
    );
}

#[test]
fn init_local_tcp_port() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(NetworkBind::local(Proto::Tcp, Host::Any, Port::Number(8080))),
    );
}

#[test]
fn init_local_udp_localhost() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(NetworkBind::local(Proto::Udp, Host::Localhost, Port::Number(53))),
    );
}

#[test]
fn init_remote_ip_any() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(NetworkBind::remote(Proto::Ip, Host::Any, Port::Any)),
    );
}

#[test]
fn init_remote_tcp_port() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(NetworkBind::remote(Proto::Tcp, Host::Any, Port::Number(443))),
    );
}

#[test]
fn init_remote_udp_localhost() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(NetworkBind::remote(Proto::Udp, Host::Localhost, Port::Number(53))),
    );
}
