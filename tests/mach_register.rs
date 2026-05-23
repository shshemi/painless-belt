mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::mach::mach_register::MachRegister;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_global_name() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(MachRegister::global_name("com.apple.test")),
    );
}

#[test]
fn init_local_name() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(MachRegister::local_name("com.apple.test")),
    );
}

#[test]
fn init_global_name_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(MachRegister::global_name_regex(Regex::new(r"^com\.apple\..*$").unwrap())),
    );
}

#[test]
fn init_local_name_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(MachRegister::local_name_regex(Regex::new(r"^com\.apple\..*$").unwrap())),
    );
}
