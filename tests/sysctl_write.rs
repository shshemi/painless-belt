mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::sysctl::sysctl_write::SysctlWrite;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_name() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(SysctlWrite::name("kern.osversion")),
    );
}

#[test]
fn init_regex() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(SysctlWrite::regex(Regex::new(r"^kern\..*$").unwrap())),
    );
}
