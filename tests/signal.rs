mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::signal::signal::Signal;
use painless_belt::sandbox::profile::Profile;

#[test]
fn init_self_target() {
    assert_profile_initializes(Profile::deny_by_default().allow(Signal::self_target()));
}

#[test]
fn init_others() {
    assert_profile_initializes(Profile::deny_by_default().allow(Signal::others()));
}
