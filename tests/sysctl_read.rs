mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::sysctl::sysctl_read::SysctlRead;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_name() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(SysctlRead::name("kern.osversion")),
    );
}

#[test]
fn init_regex() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(SysctlRead::regex(Regex::new(r"^kern\..*$").unwrap())),
    );
}
