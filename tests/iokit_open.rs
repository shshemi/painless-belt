mod common;

use common::assert_profile_initializes;
use painless_belt::sandbox::operations::iokit::iokit_open::IokitOpen;
use painless_belt::sandbox::profile::Profile;
use regex::Regex;

#[test]
fn init_user_client_class() {
    assert_profile_initializes(
        Profile::deny_by_default().allow(IokitOpen::user_client_class("AppleSMC")),
    );
}

#[test]
fn init_user_client_class_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(IokitOpen::user_client_class_regex(Regex::new(r"^Apple.*$").unwrap())),
    );
}

#[test]
fn init_property() {
    assert_profile_initializes(Profile::deny_by_default().allow(IokitOpen::property("IOClass")));
}

#[test]
fn init_property_regex() {
    assert_profile_initializes(
        Profile::deny_by_default()
            .allow(IokitOpen::property_regex(Regex::new(r"^IO.*$").unwrap())),
    );
}
