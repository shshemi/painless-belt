mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::file::file_read_data::FileReadData;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_literal() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileReadData::literal("/tmp/painless-pb")),
    );
}

#[test]
fn init_prefix() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileReadData::prefix("/tmp/painless-pb")),
    );
}

#[test]
fn init_subpath() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileReadData::subpath("/tmp/painless-pb")),
    );
}

#[test]
fn init_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(FileReadData::regex(Regex::new(r"^/tmp/painless-pb/.*$").unwrap())),
    );
}
