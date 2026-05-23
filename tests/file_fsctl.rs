mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::file::file_fsctl::FileFsctl;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_literal() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileFsctl::literal("/tmp/painless-pb")),
    );
}

#[test]
fn init_prefix() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileFsctl::prefix("/tmp/painless-pb")),
    );
}

#[test]
fn init_subpath() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileFsctl::subpath("/tmp/painless-pb")),
    );
}

#[test]
fn init_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(FileFsctl::regex(Regex::new(r"^/tmp/painless-pb/.*$").unwrap())),
    );
}
