mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::mach::mach_priv_task_port::MachPrivTaskPort;
use painless_belt::sandbox::profile::Profile;

#[test]
fn init() {
    assert_profile_initializes(Profile::deny_by_default().allow(MachPrivTaskPort));
}
