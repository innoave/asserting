use crate::prelude::*;

#[test]
fn bool_is_equal_to_bool() {
    let subject: bool = true;

    assert_that(subject).is_equal_to(true);
}

#[test]
fn bool_is_not_equal_to_bool() {
    let subject: bool = true;

    assert_that(subject).is_not_equal_to(false);
}

#[test]
fn verify_bool_is_not_equal_to_false_fails() {
    let failures = verify_that(false)
        .named("my_thing")
        .is_not_equal_to(false)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is not equal to false
   but was: false
  expected: false
"]
    );
}

#[test]
fn expression_is_true() {
    assert_that(42 == 42).is_true();
}

#[test]
fn bool_is_false() {
    assert_that(false).is_false();
}

#[test]
fn verify_bool_is_true_fails() {
    assert_eq!(
        verify_that(false)
            .named("my_thing")
            .is_true()
            .display_failures(),
        &[r"assertion failed: expected my_thing is true
   but was: false
  expected: true
"]
    );
}

#[test]
fn verify_bool_is_false_fails() {
    let failures = verify_that(true)
        .named("my_thing")
        .is_false()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is false
   but was: true
  expected: false
"]
    );
}
