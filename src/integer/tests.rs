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
    let failures = verify_that(42)
        .named("my_thing")
        .is_equal_to(-42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is equal to -42
   but was: 42
  expected: -42
"]
    );
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;

    #[test]
    fn highlight_diffs_is_equal_to_for_integers() {
        let failures = verify_that(37)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_equal_to(42)
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is equal to 42\n   \
               but was: \u{1b}[31m37\u{1b}[0m\n  \
              expected: \u{1b}[34m42\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_is_not_equal_to_for_integers() {
        let failures = verify_that(42)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_not_equal_to(42)
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is not equal to 42\n   \
               but was: 42\n  \
              expected: 42\n\
            "]
        );
    }
}
