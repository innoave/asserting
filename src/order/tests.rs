use crate::prelude::*;

//
// comparing `i32`
//

#[test]
fn i32_is_less_than_other_i32() {
    let subject = 42;

    assert_that(subject).is_less_than(43);
}

#[test]
fn verify_i32_is_less_than_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_less_than(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is less than 42
   but was: 42
  expected: < 42
"]
    );
}

#[test]
fn i32_is_greater_than_other_i32() {
    let subject = 42;

    assert_that(subject).is_greater_than(41);
}

#[test]
fn verify_i32_is_greater_than_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_greater_than(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is greater than 42
   but was: 42
  expected: > 42
"]
    );
}

#[test]
fn i32_is_at_most_other_i32() {
    let subject = 42;

    assert_that(subject).is_at_most(42);
}

#[test]
fn verify_i32_is_at_most_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_most(41)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at most 41
   but was: 42
  expected: <= 41
"]
    );
}

#[test]
fn i32_is_at_least_other_i32() {
    let subject = 42;

    assert_that(subject).is_at_least(42);
}

#[test]
fn verify_i32_is_at_least_other_i32_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_least(43)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at least 43
   but was: 42
  expected: >= 43
"]
    );
}

//
// comparing `char`
//

#[test]
fn char_is_less_than_other_char() {
    let subject = 'C';

    assert_that(subject).is_less_than('D');
}

#[test]
fn verify_char_is_less_than_other_char_fails() {
    let subject = 'C';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_less_than('C')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is less than 'C'
   but was: 'C'
  expected: < 'C'
"]
    );
}

#[test]
fn char_is_greater_than_other_char() {
    let subject = 'D';

    assert_that(subject).is_greater_than('C');
}

#[test]
fn verify_char_is_greater_than_other_char_fails() {
    let subject = 'D';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_greater_than('D')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is greater than 'D'
   but was: 'D'
  expected: > 'D'
"]
    );
}

#[test]
fn char_is_at_most_other_char() {
    let subject = 'C';

    assert_that(subject).is_at_most('C');
}

#[test]
fn verify_char_is_at_most_other_char_fails() {
    let subject = 'C';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_most('B')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at most 'B'
   but was: 'C'
  expected: <= 'B'
"]
    );
}

#[test]
fn char_is_at_least_other_char() {
    let subject = 'D';

    assert_that(subject).is_at_least('D');
}

#[test]
fn verify_char_is_at_least_other_char_fails() {
    let subject = 'D';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_at_least('E')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is at least 'E'
   but was: 'D'
  expected: >= 'E'
"]
    );
}

#[test]
fn char_is_before_other_char() {
    let subject = 'L';

    assert_that(subject).is_before('M');
}

#[test]
fn verify_char_is_before_other_char_fails() {
    let subject = 'L';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_before('L')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is before 'L'
   but was: 'L'
  expected: < 'L'
"]
    );
}

#[test]
fn char_is_after_other_char() {
    let subject = 'L';

    assert_that(subject).is_after('K');
}

#[test]
fn verify_char_is_after_other_char_fails() {
    let subject = 'L';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_after('L')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is after 'L'
   but was: 'L'
  expected: > 'L'
"]
    );
}

#[test]
fn char_is_between_a_min_char_and_a_max_char() {
    let subject = 'L';

    assert_that(subject).is_between('K', 'M');
    assert_that(subject).is_between('L', 'P');
    assert_that(subject).is_between('H', 'L');
}

#[test]
fn verify_char_is_between_a_min_char_and_a_max_char_fails() {
    let subject = 'L';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_between('M', 'P')
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing is between 'M' and 'P'
   but was: 'L'
  expected: 'M' <= x <= 'P'
"]
    );
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;

    #[test]
    fn highlight_diffs_is_less_than() {
        let subject = 3.781;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_less_than(3.779)
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is less than 3.779\n   \
                  but was: \u{1b}[31m3.781\u{1b}[0m\n  \
                 expected: < \u{1b}[32m3.779\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_is_at_most() {
        let subject = 3.781;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_at_most(3.779)
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is at most 3.779\n   \
                  but was: \u{1b}[31m3.781\u{1b}[0m\n  \
                 expected: <= \u{1b}[34m3.779\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_is_greater_than() {
        let subject = 3.781;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .is_greater_than(3.782)
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is greater than 3.782\n   \
                  but was: \u{1b}[31m3.781\u{1b}[0m\n  \
                 expected: > \u{1b}[33m3.782\u{1b}[0m\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_is_at_least() {
        let subject = 3.781;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_at_least(3.782)
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is at least 3.782\n   \
                  but was: \u{1b}[31m3.781\u{1b}[0m\n  \
                 expected: >= \u{1b}[34m3.782\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_is_between_but_is_below_min() {
        let subject = 'L';

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .is_between('M', 'P')
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is between 'M' and 'P'\n   \
                    but was: \u{1b}[31m'L'\u{1b}[0m\n  \
                   expected: \u{1b}[33m'M'\u{1b}[0m <= x <= 'P'\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_is_between_but_is_above_max() {
        let subject = 'L';

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .is_between('H', 'K')
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is between 'H' and 'K'\n   \
                    but was: \u{1b}[31m'L'\u{1b}[0m\n  \
                   expected: 'H' <= x <= \u{1b}[33m'K'\u{1b}[0m\n\
                "
            ]
        );
    }
}
