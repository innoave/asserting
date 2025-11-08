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
        &[r"expected my_thing to be not equal to false
   but was: false
  expected: not false
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
        &[r"expected my_thing to be true
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
        &[r"expected my_thing to be false
   but was: true
  expected: false
"]
    );
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;

    #[test]
    fn highlight_diffs_bool_is_true() {
        let failures = verify_that(37 == 42)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_true()
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to be true\n   \
               but was: \u{1b}[31mfalse\u{1b}[0m\n  \
              expected: \u{1b}[32mtrue\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_bool_is_false() {
        let failures = verify_that(42 == 42)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .is_false()
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to be false\n   \
               but was: \u{1b}[31mtrue\u{1b}[0m\n  \
              expected: \u{1b}[33mfalse\u{1b}[0m\n\
            "]
        );
    }
}
