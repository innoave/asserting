#![allow(clippy::approx_constant)]

use crate::prelude::*;

#[test]
fn f32_is_negative() {
    assert_that(-4.2_f32).is_negative();
    assert_that(-0.001_f32).is_negative();
}

#[test]
fn f64_is_negative() {
    assert_that(-4.2_f64).is_negative();
    assert_that(-0.001_f64).is_negative();
}

#[test]
fn borrowed_f64_is_negative() {
    assert_that!(&-4.2_f64).is_negative();
    assert_that!(&-0.001_f64).is_negative();
}

#[test]
fn mutable_borrowed_f64_is_negative() {
    assert_that!(&mut -4.2_f64).is_negative();
    assert_that!(&mut -0.001_f64).is_negative();
}

#[test]
fn verify_f64_is_negative_fails() {
    let failures = verify_that(0.0_f64)
        .named("some_number")
        .is_negative()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be negative
   but was: 0.0
  expected: < 0
"]
    );
}

#[test]
fn f32_is_not_negative() {
    assert_that(4.2_f32).is_not_negative();
    assert_that(0.001_f32).is_not_negative();
    assert_that(0.0_f32).is_not_negative();
}

#[test]
fn f64_is_not_negative() {
    assert_that(4.2_f64).is_not_negative();
    assert_that(0.001_f64).is_not_negative();
    assert_that(0.0_f64).is_not_negative();
}

#[test]
fn borrowed_f64_is_not_negative() {
    assert_that!(&4.2_f64).is_not_negative();
    assert_that!(&0.001_f64).is_not_negative();
    assert_that!(&0.0_f64).is_not_negative();
}

#[test]
fn mutable_borrowed_f64_is_not_negative() {
    assert_that!(&mut 4.2_f64).is_not_negative();
    assert_that!(&mut 0.001_f64).is_not_negative();
    assert_that!(&mut 0.0_f64).is_not_negative();
}

#[test]
fn verify_f64_is_not_negative_fails() {
    let failures = verify_that(-0.001_f64)
        .named("some_number")
        .is_not_negative()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be not negative
   but was: -0.001
  expected: >= 0
"]
    );
}

#[test]
fn f32_is_positive() {
    assert_that(4.2_f32).is_positive();
    assert_that(0.001_f32).is_positive();
}

#[test]
fn f64_is_positive() {
    assert_that(4.2_f64).is_positive();
    assert_that(0.001_f64).is_positive();
}

#[test]
fn borrowed_f64_is_positive() {
    assert_that!(&4.2_f64).is_positive();
    assert_that!(&0.001_f64).is_positive();
}

#[test]
fn mutable_borrowed_f64_is_positive() {
    assert_that!(&mut 4.2_f64).is_positive();
    assert_that!(&mut 0.001_f64).is_positive();
}

#[test]
fn verify_f64_is_positive_fails() {
    let failures = verify_that(0.0_f64)
        .named("some_number")
        .is_positive()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be positive
   but was: 0.0
  expected: > 0
"]
    );
}

#[test]
fn f32_is_not_positive() {
    assert_that(-4.2_f32).is_not_positive();
    assert_that(-0.001_f32).is_not_positive();
    assert_that(0.0_f32).is_not_positive();
}

#[test]
fn f64_is_not_positive() {
    assert_that(-4.2_f64).is_not_positive();
    assert_that(-0.001_f64).is_not_positive();
    assert_that(0.0_f64).is_not_positive();
}

#[test]
fn borrowed_f64_is_not_positive() {
    assert_that!(&-4.2_f64).is_not_positive();
    assert_that!(&-0.001_f64).is_not_positive();
    assert_that!(&0.0_f64).is_not_positive();
}

#[test]
fn mutable_borrowed_f64_is_not_positive() {
    assert_that!(&mut -4.2_f64).is_not_positive();
    assert_that!(&mut -0.001_f64).is_not_positive();
    assert_that!(&mut 0.0_f64).is_not_positive();
}

#[test]
fn verify_f64_is_not_positive_fails() {
    let failures = verify_that(0.001_f64)
        .named("some_number")
        .is_not_positive()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be not positive
   but was: 0.001
  expected: <= 0
"]
    );
}

#[test]
fn f32_is_zero() {
    assert_that(0.0_f32).is_zero();
}

#[test]
fn borrowed_f32_is_zero() {
    assert_that(&0.0_f32).is_zero();
}

#[test]
fn f64_is_zero() {
    assert_that(0.0_f64).is_zero();
}

#[test]
fn borrowed_f64_is_zero() {
    assert_that(&0.0_f64).is_zero();
}

#[test]
fn verify_f64_is_zero_fails() {
    let failures = verify_that(1.0_f64)
        .named("some_number")
        .is_zero()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be zero
   but was: 1.0
  expected: 0.0
"]
    );
}

#[test]
fn f32_is_one() {
    assert_that(1.0_f32).is_one();
}

#[test]
fn borrowed_f32_is_one() {
    assert_that(&1.0_f32).is_one();
}

#[test]
fn f64_is_one() {
    assert_that(1.0_f64).is_one();
}

#[test]
fn borrowed_f64_is_one() {
    assert_that(&1.0_f64).is_one();
}

#[test]
fn verify_f64_is_one_fails() {
    let failures = verify_that(0.0_f64)
        .named("some_number")
        .is_one()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be one
   but was: 0.0
  expected: 1.0
"]
    );
}

#[test]
fn f32_is_finite() {
    assert_that(0.123_f32).is_finite();
    assert_that(0.0_f32).is_finite();
    assert_that(-0.123_f32).is_finite();
}

#[test]
fn f32_is_infinite() {
    assert_that(f32::INFINITY).is_infinite();
    assert_that(f32::NEG_INFINITY).is_infinite();
}

#[test]
fn f64_is_finite() {
    assert_that(0.123_f64).is_finite();
    assert_that(0.0_f64).is_finite();
    assert_that(-0.123_f64).is_finite();
}

#[test]
fn borrowed_f64_is_finite() {
    assert_that!(&0.123_f64).is_finite();
    assert_that!(&0.0_f64).is_finite();
    assert_that!(&-0.123_f64).is_finite();
}

#[test]
fn mutable_borrowed_f64_is_finite() {
    assert_that!(&mut 0.123_f64).is_finite();
    assert_that!(&mut 0.0_f64).is_finite();
    assert_that!(&mut -0.123_f64).is_finite();
}

#[test]
fn verify_f64_is_finite_fails() {
    let failures = verify_that(f64::NEG_INFINITY)
        .named("some_number")
        .is_finite()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be finite
   but was: -inf
  expected: a finite number
"]
    );
}

#[test]
fn f64_is_infinite() {
    assert_that(f64::INFINITY).is_infinite();
    assert_that(f64::NEG_INFINITY).is_infinite();
}

#[test]
fn borrowed_f64_is_infinite() {
    assert_that!(&f64::INFINITY).is_infinite();
    assert_that!(&f64::NEG_INFINITY).is_infinite();
}

#[allow(const_item_mutation)]
#[test]
fn mutable_borrowed_f64_is_infinite() {
    assert_that!(&mut f64::INFINITY).is_infinite();
    assert_that!(&mut f64::NEG_INFINITY).is_infinite();
}

#[test]
fn verify_f64_is_infinite_fails() {
    let failures = verify_that(0.0_f64)
        .named("some_number")
        .is_infinite()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be infinite
   but was: 0.0
  expected: an infinite number
"]
    );
}

#[test]
fn f32_is_a_number() {
    assert_that(0.1_f32).is_a_number();
    assert_that(0.0_f32).is_a_number();
    assert_that(-0.1_f32).is_a_number();
    assert_that(-0.1_f32).is_a_number();
    assert_that(f32::INFINITY).is_a_number();
    assert_that(f32::NEG_INFINITY).is_a_number();
}

#[test]
fn f64_is_a_number() {
    assert_that(0.1_f64).is_a_number();
    assert_that(0.0_f64).is_a_number();
    assert_that(-0.1_f64).is_a_number();
    assert_that(f64::INFINITY).is_a_number();
    assert_that(f64::NEG_INFINITY).is_a_number();
}

#[test]
fn borrowed_f64_is_a_number() {
    assert_that(&0.1_f64).is_a_number();
    assert_that(&0.0_f64).is_a_number();
    assert_that(&-0.1_f64).is_a_number();
    assert_that(&f64::INFINITY).is_a_number();
    assert_that(&f64::NEG_INFINITY).is_a_number();
}

#[allow(const_item_mutation)]
#[test]
fn mutable_borrowed_f64_is_a_number() {
    assert_that(&mut 0.1_f64).is_a_number();
    assert_that(&mut 0.0_f64).is_a_number();
    assert_that(&mut -0.1_f64).is_a_number();
    assert_that(&mut f64::INFINITY).is_a_number();
    assert_that(&mut f64::NEG_INFINITY).is_a_number();
}

#[test]
fn verify_f64_is_a_number_fails() {
    let failures = verify_that(f64::NAN)
        .named("some_number")
        .is_a_number()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be a number
   but was: NaN
  expected: a number
"]
    );
}

#[test]
fn f32_is_not_a_number() {
    assert_that(f32::NAN).is_not_a_number();
}

#[test]
fn verify_f32_is_not_a_number_fails() {
    let failures = verify_that(0.0_f32)
        .named("some_number")
        .is_not_a_number()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be not a number
   but was: 0.0
  expected: NaN
"]
    );
}

#[test]
fn f64_is_not_a_number() {
    assert_that(f64::NAN).is_not_a_number();
}

#[test]
fn borrowed_f64_is_not_a_number() {
    assert_that(&f64::NAN).is_not_a_number();
}

#[allow(const_item_mutation)]
#[test]
fn mutable_borrowed_f64_is_not_a_number() {
    assert_that(&mut f64::NAN).is_not_a_number();
}

#[test]
fn verify_f64_is_not_a_number_fails() {
    let failures = verify_that(0.0_f64)
        .named("some_number")
        .is_not_a_number()
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected some_number to be not a number
   but was: 0.0
  expected: NaN
"]
    );
}

#[cfg(feature = "float-cmp")]
mod cmp {
    use crate::prelude::*;

    #[test]
    fn f32_is_close_to_another_f32_within_default_margin() {
        assert_that(6.28_f32 / 2.).is_close_to(3.14);
    }

    #[test]
    fn verify_f32_is_close_to_another_f32_within_default_margin_fails() {
        let failures = verify_that(6.28_f32 / 2.)
            .named("tau / 2")
            .is_close_to(3.15)
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be close to 3.15
  within a margin of epsilon=4.7683716e-7 and ulps=4
   but was: 3.14
  expected: 3.15
"]
        );
    }

    #[test]
    fn f32_is_not_close_to_another_f32_within_default_margin() {
        assert_that(6.28_f32 / 2.).is_not_close_to(3.15);
    }

    #[test]
    fn verify_f32_is_not_close_to_another_f32_within_default_margin_fails() {
        let failures = verify_that(6.28_f32 / 2.)
            .named("tau / 2")
            .is_not_close_to(3.14)
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be not close to 3.14
  within a margin of epsilon=4.7683716e-7 and ulps=4
   but was: 3.14
  expected: 3.14
"]
        );
    }

    #[test]
    fn f32_is_close_to_another_f32_within_given_margin() {
        assert_that(6.28_f32 / 2.).is_close_to_with_margin(3.14, (2. * f32::EPSILON, 3));
    }

    #[test]
    fn verify_f32_is_close_to_another_f32_within_given_margin_fails() {
        let failures = verify_that(6.28_f32 / 2.)
            .named("tau / 2")
            .is_close_to_with_margin(3.15, (2. * f32::EPSILON, 3))
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be close to 3.15
  within a margin of epsilon=2.3841858e-7 and ulps=3
   but was: 3.14
  expected: 3.15
"]
        );
    }

    #[test]
    fn f32_is_not_close_to_another_f32_within_given_margin() {
        assert_that(6.28_f32 / 2.).is_not_close_to_with_margin(3.15, (2. * f32::EPSILON, 3));
    }

    #[test]
    fn verify_f32_is_not_close_to_another_f32_within_given_margin_fails() {
        let failures = verify_that(6.28_f32 / 2.)
            .named("tau / 2")
            .is_not_close_to_with_margin(3.14, (2. * f32::EPSILON, 3))
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be not close to 3.14
  within a margin of epsilon=2.3841858e-7 and ulps=3
   but was: 3.14
  expected: 3.14
"]
        );
    }

    #[test]
    fn f64_is_close_to_another_f64_within_default_margin() {
        assert_that(6.28_f64 / 2.).is_close_to(3.14);
    }

    #[test]
    fn verify_f64_is_close_to_another_f64_within_default_margin_fails() {
        let failures = verify_that(6.28_f64 / 2.)
            .named("tau / 2")
            .is_close_to(3.15)
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be close to 3.15
  within a margin of epsilon=8.881784197001252e-16 and ulps=4
   but was: 3.14
  expected: 3.15
"]
        );
    }

    #[test]
    fn f64_is_not_close_to_another_f64_within_default_margin() {
        assert_that(6.28_f64 / 2.).is_not_close_to(3.15);
    }

    #[test]
    fn verify_f64_is_not_close_to_another_f64_within_default_margin_fails() {
        let failures = verify_that(6.28_f64 / 2.)
            .named("tau / 2")
            .is_not_close_to(3.14)
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be not close to 3.14
  within a margin of epsilon=8.881784197001252e-16 and ulps=4
   but was: 3.14
  expected: 3.14
"]
        );
    }

    #[test]
    fn f64_is_close_to_another_f64_within_given_margin() {
        assert_that(6.28_f64 / 2.).is_close_to_with_margin(3.14, (2. * f64::EPSILON, 3));
    }

    #[test]
    fn verify_f64_is_close_to_another_f64_within_given_margin_fails() {
        let failures = verify_that(6.28_f64 / 2.)
            .named("tau / 2")
            .is_close_to_with_margin(3.15, (2. * f64::EPSILON, 3))
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be close to 3.15
  within a margin of epsilon=4.440892098500626e-16 and ulps=3
   but was: 3.14
  expected: 3.15
"]
        );
    }

    #[test]
    fn f64_is_not_close_to_another_f64_within_given_margin() {
        assert_that(6.28_f64 / 2.).is_not_close_to_with_margin(3.15, (2. * f64::EPSILON, 3));
    }

    #[test]
    fn verify_f64_is_not_close_to_another_f64_within_given_margin_fails() {
        let failures = verify_that(6.28_f64 / 2.)
            .named("tau / 2")
            .is_not_close_to_with_margin(3.14, (2. * f64::EPSILON, 3))
            .display_failures();

        assert_eq!(
            failures,
            &[r"assertion failed: expected tau / 2 to be not close to 3.14
  within a margin of epsilon=4.440892098500626e-16 and ulps=3
   but was: 3.14
  expected: 3.14
"]
        );
    }

    #[cfg(feature = "colored")]
    mod colored {
        use crate::prelude::*;

        #[test]
        fn highlight_diffs_f32_is_close_to() {
            let failures = verify_that(6.28318_f32 / 2.)
                .with_diff_format(DIFF_FORMAT_RED_BLUE)
                .is_close_to_with_margin(3.15148, (2. * f32::EPSILON, 3))
                .display_failures();

            assert_eq!(
                failures,
                &[
                    "assertion failed: expected subject to be close to 3.15148\n  \
                within a margin of epsilon=2.3841858e-7 and ulps=3\n   \
                 but was: 3.1\u{1b}[31m41\u{1b}[0m5\u{1b}[31m9\u{1b}[0m\n  \
                expected: 3.15\u{1b}[34m148\u{1b}[0m\n\
            "
                ]
            );
        }

        #[test]
        fn highlight_diffs_f64_is_close_to() {
            let failures = verify_that(6.28318_f64 / 2.)
                .with_diff_format(DIFF_FORMAT_RED_BLUE)
                .is_close_to_with_margin(3.15148, (2. * f64::EPSILON, 3))
                .display_failures();

            assert_eq!(
                failures,
                &[
                    "assertion failed: expected subject to be close to 3.15148\n  \
                within a margin of epsilon=4.440892098500626e-16 and ulps=3\n   \
                 but was: 3.1\u{1b}[31m41\u{1b}[0m5\u{1b}[31m9\u{1b}[0m\n  \
                expected: 3.15\u{1b}[34m148\u{1b}[0m\n\
            "
                ]
            );
        }
    }
}
