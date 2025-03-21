use crate::prelude::*;
#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec,
};

#[test]
fn option_of_i32_is_none() {
    let subject: Option<i32> = None;

    assert_that(subject).is_none();
}

#[test]
fn option_of_i32_is_some() {
    let subject: Option<i32> = Some(42);

    assert_that(subject).is_some();
}

#[test]
fn option_of_string_is_none() {
    let subject: Option<String> = None;

    assert_that(subject).is_none();
}

#[test]
fn option_of_string_is_some() {
    let subject: Option<String> = Some("te veniam dolore ut".to_string());

    assert_that(subject).is_some();
}

#[test]
fn option_of_custom_struct_is_none() {
    #[derive(Debug)]
    struct MyStruct;

    let subject: Option<MyStruct> = None;

    assert_that(subject).is_none();
}

#[test]
fn option_of_string_has_some_str_value() {
    let subject = Some("non tempor ea delenit".to_string());

    assert_that(subject).has_value("non tempor ea delenit");
}

#[test]
fn option_of_str_has_some_str_value() {
    let subject = Some("facilisi cupiditat elitr facilisis");

    assert_that(subject).has_value("facilisi cupiditat elitr facilisis");
}

#[test]
fn option_of_str_has_some_string_value() {
    let subject = Some("invidunt commodi eros vel");

    assert_that(subject).has_value("invidunt commodi eros vel".to_string());
}

#[test]
fn option_of_custom_struct_has_value() {
    #[derive(Debug, PartialEq)]
    struct MyStruct;

    let subject = Some(MyStruct);

    assert_that(subject).has_value(MyStruct);
}

#[test]
fn verify_option_of_custom_struct_is_some_fails() {
    #[derive(Debug)]
    struct MyStruct;

    let subject: Option<MyStruct> = None;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_some()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is Some(_)
   but was: None
  expected: Some(_)
"]
    );
}

#[test]
fn verify_option_of_custom_struct_is_none_fails() {
    #[derive(Debug)]
    struct MyStruct;

    let subject: Option<MyStruct> = Some(MyStruct);

    let failures = verify_that(subject)
        .named("my_thing")
        .is_none()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is None
   but was: Some(MyStruct)
  expected: None
"]
    );
}

#[test]
fn verify_option_of_string_has_some_value_fails() {
    let subject = Some("labore dolore voluptate culpa".to_string());

    let failures = verify_that(subject)
        .named("my_thing")
        .has_value("labore dolores voluptate culpa")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing is some containing "labore dolores voluptate culpa"
   but was: Some("labore dolore voluptate culpa")
  expected: Some("labore dolores voluptate culpa")
"#
        ]
    );
}

#[test]
fn map_option_with_some_value_to_its_value() {
    let subject = Some(vec![1, 2, 3]);

    assert_that(subject).some().is_not_empty();
}

#[cfg(feature = "panic")]
#[test]
fn map_option_with_none_to_its_value() {
    let subject: Option<Vec<usize>> = None;

    assert_that_code(|| {
        assert_that(subject).some().is_empty();
    })
    .panics_with_message("assertion failed: expected the subject to be `Some(_)`, but was `None`");
}
