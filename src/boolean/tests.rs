use crate::prelude::*;

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
