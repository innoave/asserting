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

#[test]
fn check_option_of_custom_struct_is_some_fails() {
    #[derive(Debug)]
    struct MyStruct;

    let subject: Option<MyStruct> = None;

    assert_eq!(
        check_that(subject).named("my_thing").is_some().to_string(),
        r"assertion failed: expected my_thing is some
   but was: None
  expected: Some(_)
"
    );
}

#[test]
fn check_option_of_custom_struct_is_none_fails() {
    #[derive(Debug)]
    struct MyStruct;

    let subject: Option<MyStruct> = Some(MyStruct);

    assert_eq!(
        check_that(subject).named("my_thing").is_none().to_string(),
        r"assertion failed: expected my_thing is none
   but was: Some(MyStruct)
  expected: None
"
    );
}

#[test]
fn check_option_of_string_has_some_value_fails() {
    let subject = Some("labore dolore voluptate culpa".to_string());

    assert_eq!(
        check_that(subject)
            .named("my_thing")
            .has_value("labore dolores voluptate culpa")
            .to_string(),
        r#"assertion failed: expected my_thing has some value "labore dolores voluptate culpa"
   but was: Some("labore dolore voluptate culpa")
  expected: Some("labore dolores voluptate culpa")
"#
    );
}
