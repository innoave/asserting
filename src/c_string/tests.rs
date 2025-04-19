use crate::prelude::*;
use crate::std::ffi::{CStr, CString};

#[test]
fn c_str_is_empty() {
    let subject: &CStr = CStr::from_bytes_until_nul(b"\0")
        .unwrap_or_else(|err| panic!("could not create CStr: {err}"));

    assert_that(subject).is_empty();
}

#[test]
fn c_str_is_not_empty() {
    let subject: &CStr = CStr::from_bytes_until_nul(b"facilisi dolores nostrud aliquyam\0")
        .unwrap_or_else(|err| panic!("could not create CStr: {err}"));

    assert_that(subject).is_not_empty();
}

#[test]
fn c_string_is_empty() {
    let subject: CString =
        CString::new(b"").unwrap_or_else(|err| panic!("could not create a CString: {err}"));

    assert_that(subject).is_empty();
}

#[test]
fn c_string_is_not_empty() {
    let subject: CString = CString::new(b"anim ea aute aliqua")
        .unwrap_or_else(|err| panic!("could not create a CString: {err}"));

    assert_that(subject).is_not_empty();
}
