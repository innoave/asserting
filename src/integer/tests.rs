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
        &[r"assertion failed: expected my_thing to be equal to -42
   but was: 42
  expected: -42
"]
    );
}

#[test]
fn i8_is_negative() {
    assert_that(-42_i8).is_negative();
    assert_that(-1_i8).is_negative();
}

#[test]
fn i16_is_negative() {
    assert_that(-42_i16).is_negative();
    assert_that(-1_i16).is_negative();
}

#[test]
fn i32_is_negative() {
    assert_that(-42_i32).is_negative();
    assert_that(-1_i32).is_negative();
}

#[test]
fn i64_is_negative() {
    assert_that(-42_i64).is_negative();
    assert_that(-1_i64).is_negative();
}

#[test]
fn i128_is_negative() {
    assert_that(-42_i128).is_negative();
    assert_that(-1_i128).is_negative();
}

#[test]
fn isize_is_negative() {
    assert_that(-42_isize).is_negative();
    assert_that(-1_isize).is_negative();
}

#[test]
fn borrowed_isize_is_negative() {
    assert_that(&-42_isize).is_negative();
    assert_that(&-1_isize).is_negative();
}

#[test]
fn mutable_borrowed_isize_is_negative() {
    assert_that(&mut -42_isize).is_negative();
    assert_that(&mut -1_isize).is_negative();
}

#[test]
fn verify_i32_is_negative_fails() {
    let failures = verify_that(0_i32)
        .named("some_number")
        .is_negative()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be negative
   but was: 0
  expected: < 0
"]
    );
}

#[test]
fn i8_is_not_negative() {
    assert_that(42_i8).is_not_negative();
    assert_that(1_i8).is_not_negative();
    assert_that(0_i8).is_not_negative();
}

#[test]
fn i16_is_not_negative() {
    assert_that(42_i16).is_not_negative();
    assert_that(1_i16).is_not_negative();
    assert_that(0_i16).is_not_negative();
}

#[test]
fn i32_is_not_negative() {
    assert_that(42_i32).is_not_negative();
    assert_that(1_i32).is_not_negative();
    assert_that(0_i32).is_not_negative();
}

#[test]
fn i64_is_not_negative() {
    assert_that(42_i64).is_not_negative();
    assert_that(1_i64).is_not_negative();
    assert_that(0_i64).is_not_negative();
}

#[test]
fn i128_is_not_negative() {
    assert_that(42_i128).is_not_negative();
    assert_that(1_i128).is_not_negative();
    assert_that(0_i128).is_not_negative();
}

#[test]
fn isize_is_not_negative() {
    assert_that(42_isize).is_not_negative();
    assert_that(1_isize).is_not_negative();
    assert_that(0_isize).is_not_negative();
}

#[test]
fn borrowed_isize_is_not_negative() {
    assert_that(&42_isize).is_not_negative();
    assert_that(&1_isize).is_not_negative();
    assert_that(&0_isize).is_not_negative();
}

#[test]
fn mutable_borrowed_isize_is_not_negative() {
    assert_that(&mut 42_isize).is_not_negative();
    assert_that(&mut 1_isize).is_not_negative();
    assert_that(&mut 0_isize).is_not_negative();
}

#[test]
fn verify_i32_is_not_negative_fails() {
    let failures = verify_that(-1_i32)
        .named("some_number")
        .is_not_negative()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be not negative
   but was: -1
  expected: >= 0
"]
    );
}

#[test]
fn i8_is_positive() {
    assert_that(42_i8).is_positive();
    assert_that(1_i8).is_positive();
}

#[test]
fn i16_is_positive() {
    assert_that(42_i16).is_positive();
    assert_that(1_i16).is_positive();
}

#[test]
fn i32_is_positive() {
    assert_that(42_i32).is_positive();
    assert_that(1_i32).is_positive();
}

#[test]
fn i64_is_positive() {
    assert_that(42_i64).is_positive();
    assert_that(1_i64).is_positive();
}

#[test]
fn i128_is_positive() {
    assert_that(42_i128).is_positive();
    assert_that(1_i128).is_positive();
}

#[test]
fn isize_is_positive() {
    assert_that(42_isize).is_positive();
    assert_that(1_isize).is_positive();
}

#[test]
fn borrowed_isize_is_positive() {
    assert_that(&42_isize).is_positive();
    assert_that(&1_isize).is_positive();
}

#[test]
fn mutable_borrowed_isize_is_positive() {
    assert_that(&mut 42_isize).is_positive();
    assert_that(&mut 1_isize).is_positive();
}

#[test]
fn verify_i32_is_positive_fails() {
    let failures = verify_that(0_i32)
        .named("some_number")
        .is_positive()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be positive
   but was: 0
  expected: > 0
"]
    );
}

#[test]
fn i8_is_not_positive() {
    assert_that(-42_i8).is_not_positive();
    assert_that(-1_i8).is_not_positive();
    assert_that(0_i8).is_not_positive();
}

#[test]
fn i16_is_not_positive() {
    assert_that(-42_i16).is_not_positive();
    assert_that(-1_i16).is_not_positive();
    assert_that(0_i16).is_not_positive();
}

#[test]
fn i32_is_not_positive() {
    assert_that(-42_i32).is_not_positive();
    assert_that(-1_i32).is_not_positive();
    assert_that(0_i32).is_not_positive();
}

#[test]
fn i64_is_not_positive() {
    assert_that(-42_i64).is_not_positive();
    assert_that(-1_i64).is_not_positive();
    assert_that(0_i64).is_not_positive();
}

#[test]
fn i128_is_not_positive() {
    assert_that(-42_i128).is_not_positive();
    assert_that(-1_i128).is_not_positive();
    assert_that(0_i128).is_not_positive();
}

#[test]
fn isize_is_not_positive() {
    assert_that(-42_isize).is_not_positive();
    assert_that(-1_isize).is_not_positive();
    assert_that(0_isize).is_not_positive();
}

#[test]
fn borrowed_isize_is_not_positive() {
    assert_that(&-42_isize).is_not_positive();
    assert_that(&-1_isize).is_not_positive();
    assert_that(&0_isize).is_not_positive();
}

#[test]
fn mutable_borrowed_isize_is_not_positive() {
    assert_that(&mut -42_isize).is_not_positive();
    assert_that(&mut -1_isize).is_not_positive();
    assert_that(&mut 0_isize).is_not_positive();
}

#[test]
fn verify_i32_is_not_positive_fails() {
    let failures = verify_that(1_i32)
        .named("some_number")
        .is_not_positive()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be not positive
   but was: 1
  expected: <= 0
"]
    );
}

#[test]
fn i8_is_zero() {
    assert_that(0_i8).is_zero();
}

#[test]
fn i16_is_zero() {
    assert_that(0_i16).is_zero();
}

#[test]
fn i32_is_zero() {
    assert_that(0_i32).is_zero();
}

#[test]
fn i64_is_zero() {
    assert_that(0_i64).is_zero();
}

#[test]
fn i128_is_zero() {
    assert_that(0_i128).is_zero();
}

#[test]
fn isize_is_zero() {
    assert_that(0_isize).is_zero();
}

#[test]
fn u8_is_zero() {
    assert_that(0_u8).is_zero();
}

#[test]
fn u16_is_zero() {
    assert_that(0_u16).is_zero();
}

#[test]
fn u32_is_zero() {
    assert_that(0_u32).is_zero();
}

#[test]
fn u64_is_zero() {
    assert_that(0_u64).is_zero();
}

#[test]
fn u128_is_zero() {
    assert_that(0_u128).is_zero();
}

#[test]
fn usize_is_zero() {
    assert_that(0_usize).is_zero();
}

#[test]
fn borrowed_usize_is_zero() {
    assert_that(&0_usize).is_zero();
}

#[test]
fn verify_u64_is_zero_fails() {
    let failures = verify_that(1_u64)
        .named("some_number")
        .is_zero()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be zero
   but was: 1
  expected: 0
"]
    );
}

#[test]
fn i8_is_one() {
    assert_that(1_i8).is_one();
}

#[test]
fn i16_is_one() {
    assert_that(1_i16).is_one();
}

#[test]
fn i32_is_one() {
    assert_that(1_i32).is_one();
}

#[test]
fn i64_is_one() {
    assert_that(1_i64).is_one();
}

#[test]
fn i128_is_one() {
    assert_that(1_i128).is_one();
}

#[test]
fn isize_is_one() {
    assert_that(1_isize).is_one();
}

#[test]
fn u8_is_one() {
    assert_that(1_u8).is_one();
}

#[test]
fn u16_is_one() {
    assert_that(1_u16).is_one();
}

#[test]
fn u32_is_one() {
    assert_that(1_u32).is_one();
}

#[test]
fn u64_is_one() {
    assert_that(1_u64).is_one();
}

#[test]
fn u128_is_one() {
    assert_that(1_u128).is_one();
}

#[test]
fn usize_is_one() {
    assert_that(1_usize).is_one();
}

#[test]
fn borrowed_usize_is_one() {
    assert_that(&1_usize).is_one();
}

#[test]
fn verify_u64_is_one_fails() {
    let failures = verify_that(0_u64)
        .named("some_number")
        .is_one()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be one
   but was: 0
  expected: 1
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
            &["assertion failed: expected subject to be equal to 42\n   \
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
            &[
                "assertion failed: expected subject to be not equal to 42\n   \
               but was: 42\n  \
              expected: not 42\n\
            "
            ]
        );
    }
}
