use crate::prelude::*;

#[test]
fn usize_is_equal_to_usize() {
    let subject: usize = 42;

    assert_that(subject).is_equal_to(42);
}

#[test]
fn usize_is_not_equal_to_usize() {
    let subject: usize = 42;

    assert_that(subject).is_not_equal_to(51);
}

#[test]
fn i32_is_equal_to_i32() {
    let subject: i32 = -42;

    assert_that(subject).is_equal_to(-42);
}

#[test]
fn i32_is_not_equal_to_i32() {
    let subject: i32 = 42;

    assert_that(subject).is_not_equal_to(-42);
}

#[test]
fn verify_i32_is_equal_to_i32_fails() {
    assert_eq!(
        verify_that(42)
            .named("my_thing")
            .is_equal_to(-42)
            .display_failures(),
        &[r"assertion failed: expected my_thing is equal to -42
   but was: 42
  expected: -42
"]
    );
}
