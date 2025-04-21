use crate::prelude::*;

//
// Is in range for `i32`
//

#[test]
fn i32_is_in_range() {
    let subject = 42;

    assert_that(subject).is_in_range(41..43);
}

#[test]
fn verify_i32_is_in_range_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range(43..51)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of 43..51
   but was: 42
  expected: 43 <= x < 51
"
        ]
    );
}

#[test]
fn i32_is_in_inclusive_range() {
    let subject = 42;

    assert_that(subject).is_in_range(41..=43);
}

#[test]
fn verify_i32_is_in_inclusive_range_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range(43..=51)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of 43..=51
   but was: 42
  expected: 43 <= x <= 51
"
        ]
    );
}

#[test]
fn i32_is_in_range_from() {
    let subject = 42;

    assert_that(subject).is_in_range(41..);
}

#[test]
fn verify_i32_is_in_range_from_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range(43..)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of 43..
   but was: 42
  expected: 43 <= x < ..
"
        ]
    );
}

#[test]
fn i32_is_in_range_to() {
    let subject = 42;

    assert_that(subject).is_in_range(..43);
}

#[test]
fn verify_i32_is_in_range_to_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range(..42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of ..42
   but was: 42
  expected: .. < x < 42
"
        ]
    );
}

#[test]
fn i32_is_in_range_to_inclusive() {
    let subject = 42;

    assert_that(subject).is_in_range(..=42);
}

#[test]
fn verify_i32_is_in_range_to_inclusive_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range(..=41)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of ..=41
   but was: 42
  expected: .. < x <= 41
"
        ]
    );
}

#[test]
fn i32_is_not_in_range() {
    let subject = 42;

    assert_that(subject).is_not_in_range(39..=41);
}

#[test]
fn verify_i32_is_not_in_range_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range(41..43)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of 41..43
   but was: 42
  expected: x < 41 || x >= 43
"
        ]
    );
}

#[test]
fn i32_is_not_in_inclusive_range() {
    let subject = 42;

    assert_that(subject).is_not_in_range(39..=41);
}

#[test]
fn verify_i32_is_not_in_inclusive_range_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range(41..=42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of 41..=42
   but was: 42
  expected: x < 41 || x > 42
"
        ]
    );
}

#[test]
fn i32_is_not_in_range_from() {
    let subject = 38;

    assert_that(subject).is_not_in_range(39..);
}

#[test]
fn verify_i32_is_not_in_range_from_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range(42..)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of 42..
   but was: 42
  expected: x < 42 || x > ..
"
        ]
    );
}

#[test]
fn i32_is_not_in_range_to() {
    let subject = 42;

    assert_that(subject).is_not_in_range(..42);
}

#[test]
fn verify_i32_is_not_in_range_to_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range(..43)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of ..43
   but was: 42
  expected: x < .. || x >= 43
"
        ]
    );
}

#[test]
fn i32_is_not_in_range_to_inclusive() {
    let subject = 42;

    assert_that(subject).is_not_in_range(..=41);
}

#[test]
fn verify_i32_is_not_in_range_to_inclusive_fails() {
    let subject = 42;

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range(..=42)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of ..=42
   but was: 42
  expected: x < .. || x > 42
"
        ]
    );
}

//
// Is in range for `char`
//

#[test]
fn char_is_in_range() {
    let subject = 'K';

    assert_that(subject).is_in_range('J'..'L');
}

#[test]
fn verify_char_is_in_range_fails() {
    let subject = 'K';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range('L'..'Z')
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of 'L'..'Z'
   but was: 'K'
  expected: 'L' <= x < 'Z'
"
        ]
    );
}

#[test]
fn char_is_in_inclusive_range() {
    let subject = 'K';

    assert_that(subject).is_in_range('J'..='L');
}

#[test]
fn verify_char_is_in_inclusive_range_fails() {
    let subject = 'K';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_in_range('L'..='Z')
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is within range of 'L'..='Z'
   but was: 'K'
  expected: 'L' <= x <= 'Z'
"
        ]
    );
}

#[test]
fn char_is_not_in_range() {
    let subject = 'K';

    assert_that(subject).is_not_in_range('A'..='J');
}

#[test]
fn verify_char_is_not_in_range_fails() {
    let subject = 'K';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range('J'..'L')
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of 'J'..'L'
   but was: 'K'
  expected: x < 'J' || x >= 'L'
"
        ]
    );
}

#[test]
fn char_is_not_in_inclusive_range() {
    let subject = 'K';

    assert_that(subject).is_not_in_range('A'..='J');
}

#[test]
fn verify_char_is_not_in_inclusive_range_fails() {
    let subject = 'K';

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_in_range('J'..='K')
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing is not within range of 'J'..='K'
   but was: 'K'
  expected: x < 'J' || x > 'K'
"
        ]
    );
}

#[test]
fn range_is_empty() {
    let range = 1..1;

    assert_that!(range.clone().count()).is_equal_to(0);
    assert_that(range).is_empty();
}

#[test]
#[allow(clippy::reversed_empty_ranges)]
fn inclusive_range_is_empty() {
    let range = 1..=0;

    assert_that(range.clone().count()).is_equal_to(0);
    assert_that(range).is_empty();
}

#[test]
fn range_is_not_empty() {
    let range = 1..2;

    assert_that(range.clone().count()).is_equal_to(1);
    assert_that(range).is_not_empty();
}

#[test]
fn inclusive_range_is_not_empty() {
    let range = 1..=1;

    assert_that(range.clone().count()).is_equal_to(1);
    assert_that(range).is_not_empty();
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;

    #[test]
    fn highlight_diffs_i64_is_in_range_above_upper_bound() {
        let subject = 29_834;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_in_range(-4321..4322)
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is within range of -4321..4322\n   \
                     but was: \u{1b}[31m29834\u{1b}[0m\n  \
                    expected: -4321 <= x < \u{1b}[34m4322\u{1b}[0m\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_i64_is_in_range_below_lower_bound() {
        let subject = -29_834;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_in_range(-4321..4322)
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is within range of -4321..4322\n   \
                     but was: \u{1b}[31m-29834\u{1b}[0m\n  \
                    expected: \u{1b}[34m-4321\u{1b}[0m <= x < 4322\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_i64_is_in_inclusive_range_above_upper_bound() {
        let subject = 29_834;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_in_range(-4321..=4321)
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is within range of -4321..=4321\n   \
                     but was: \u{1b}[31m29834\u{1b}[0m\n  \
                    expected: -4321 <= x <= \u{1b}[34m4321\u{1b}[0m\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_i64_is_in_inclusive_range_below_lower_bound() {
        let subject = -29_834;

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_in_range(-4321..=4321)
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is within range of -4321..=4321\n   \
                     but was: \u{1b}[31m-29834\u{1b}[0m\n  \
                    expected: \u{1b}[34m-4321\u{1b}[0m <= x <= 4321\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_char_is_not_in_range() {
        let subject = 'm';

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_not_in_range('a'..'p')
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is not within range of 'a'..'p'\n   \
                     but was: \u{1b}[31m'm'\u{1b}[0m\n  \
                    expected: x < \u{1b}[32m'a'\u{1b}[0m || x >= \u{1b}[32m'p'\u{1b}[0m\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_char_is_not_in_inclusive_range() {
        let subject = 'm';

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_not_in_range('a'..='z')
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is not within range of 'a'..='z'\n   \
                     but was: \u{1b}[31m'm'\u{1b}[0m\n  \
                    expected: x < \u{1b}[32m'a'\u{1b}[0m || x > \u{1b}[32m'z'\u{1b}[0m\n\
                "
            ]
        );
    }
}
