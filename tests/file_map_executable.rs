mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::file::file_map_executable::FileMapExecutable;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_literal() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileMapExecutable::literal("/tmp/painless-pb")),
    );
}

#[test]
fn init_prefix() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileMapExecutable::prefix("/tmp/painless-pb")),
    );
}

#[test]
fn init_subpath() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileMapExecutable::subpath("/tmp/painless-pb")),
    );
}

#[test]
fn init_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(FileMapExecutable::regex(Regex::new(r"^/tmp/painless-pb/.*$").unwrap())),
    );
}
