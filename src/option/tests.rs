use crate::prelude::*;

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
