mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::mach::mach_lookup::MachLookup;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_global_name() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(MachLookup::global_name("com.apple.test")),
    );
}

#[test]
fn init_local_name() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(MachLookup::local_name("com.apple.test")),
    );
}

#[test]
fn init_global_name_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(MachLookup::global_name_regex(Regex::new(r"^com\.apple\..*$").unwrap())),
    );
}

#[test]
fn init_local_name_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(MachLookup::local_name_regex(Regex::new(r"^com\.apple\..*$").unwrap())),
    );
}
