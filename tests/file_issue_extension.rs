mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::file::file_issue_extension::FileIssueExtension;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_literal() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileIssueExtension::literal("/tmp/painless-pb")),
    );
}

#[test]
fn init_prefix() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileIssueExtension::prefix("/tmp/painless-pb")),
    );
}

#[test]
fn init_subpath() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(FileIssueExtension::subpath("/tmp/painless-pb")),
    );
}

#[test]
fn init_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(FileIssueExtension::regex(Regex::new(r"^/tmp/painless-pb/.*$").unwrap())),
    );
}
