use crate::prelude::*;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

#[test]
fn result_of_i32_is_ok() {
    let subject: Result<i32, String> = Ok(42);

    assert_that(subject).is_ok();
}

#[test]
fn result_of_custom_types_is_err() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Err(MyError);

    assert_that(subject).is_err();
}

#[test]
fn verify_result_of_custom_types_is_ok_fails() {
    #[derive(Debug)]
    struct MyValue;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("aute nam ad amet".to_string()));

    let failures = verify_that(subject)
        .named("my_thing")
        .is_ok()
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing is Ok(_)
   but was: Err(MyError("aute nam ad amet"))
  expected: Ok(_)
"#]
    );
}

#[test]
fn verify_result_of_custom_types_is_err_fails() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyValue(i32);

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    let failures = verify_that(subject)
        .named("my_thing")
        .is_err()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is Err(_)
   but was: Ok(MyValue(42))
  expected: Err(_)
"]
    );
}

#[test]
fn result_of_custom_types_has_value() {
    #[derive(Debug, PartialEq)]
    struct MyValue(i32);

    #[derive(Debug)]
    struct MyError;

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    assert_that(subject).has_value(MyValue(42));
}

#[test]
fn result_of_custom_types_has_error() {
    #[derive(Debug)]
    struct MyValue;

    #[derive(Debug, PartialEq)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("to complicated!".to_string()));

    assert_that(subject).has_error(MyError("to complicated!".to_string()));
}

#[test]
fn verify_result_of_custom_types_has_value_fails() {
    #[derive(Debug, PartialEq)]
    struct MyValue(String);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyError(String);

    let subject: Result<MyValue, MyError> = Err(MyError("amet esse rebum feugait".to_string()));

    let failures = verify_that(subject)
        .named("my_thing")
        .has_value(MyValue("sea non obcaecat nostrud".to_string()))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing is ok containing MyValue("sea non obcaecat nostrud")
   but was: Err(MyError("amet esse rebum feugait"))
  expected: Ok(MyValue("sea non obcaecat nostrud"))
"#
        ]
    );
}

#[test]
fn verify_result_of_custom_types_has_error_fails() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyValue(u32);

    #[derive(Debug, PartialEq)]
    struct MyError(i32);

    let subject: Result<MyValue, MyError> = Ok(MyValue(42));

    let failures = verify_that(subject)
        .named("my_thing")
        .has_error(MyError(-1))
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is error containing MyError(-1)
   but was: Ok(MyValue(42))
  expected: Err(MyError(-1))
"
        ]
    );
}
