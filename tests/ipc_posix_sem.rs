mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::ipc::ipc_posix_sem::IpcPosixSem;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_name() {
    assert_profile_initializes(Profile::deny_by_default().allow(IpcPosixSem::name("/painless-pb")));
}

#[test]
fn init_regex() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(IpcPosixSem::regex(Regex::new(r"^/painless-.*$").unwrap())),
    );
}
