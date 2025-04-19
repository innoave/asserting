use crate::prelude::*;
use crate::std::ffi::{OsStr, OsString};

#[test]
fn os_str_is_empty() {
    let subject: &OsStr = OsStr::new("");

    assert_that(subject).is_empty();
}

#[test]
fn os_str_is_not_empty() {
    let subject: &OsStr = OsStr::new("officia praesent minim feugait");

    assert_that(subject).is_not_empty();
}

#[test]
fn os_string_is_empty() {
    let subject: OsString = OsString::new();

    assert_that(subject).is_empty();
}

#[test]
fn os_string_is_not_empty() {
    let subject: OsString = OsString::from("anim ea aute aliqua");

    assert_that(subject).is_not_empty();
}

#[test]
fn os_str_has_length() {
    let subject: &OsStr = OsStr::new("A");

    assert_that(subject).has_length(1);
}

#[test]
fn os_string_has_length() {
    let subject: OsString = OsString::from("ABC");

    assert_that(subject).has_length(3);
}
