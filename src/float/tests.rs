#![allow(clippy::approx_constant)]

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
        &[r"assertion failed: expected tau / 2 is close to 3.15
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
        &[r"assertion failed: expected tau / 2 is not close to 3.14
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
        &[r"assertion failed: expected tau / 2 is close to 3.15
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
fn verify_f32_is_not_close_to_another_f32_within_givenh_margin_fails() {
    let failures = verify_that(6.28_f32 / 2.)
        .named("tau / 2")
        .is_not_close_to_with_margin(3.14, (2. * f32::EPSILON, 3))
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected tau / 2 is not close to 3.14
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
        &[r"assertion failed: expected tau / 2 is close to 3.15
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
        &[r"assertion failed: expected tau / 2 is not close to 3.14
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
        &[r"assertion failed: expected tau / 2 is close to 3.15
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
fn verify_f64_is_not_close_to_another_f64_within_givenh_margin_fails() {
    let failures = verify_that(6.28_f64 / 2.)
        .named("tau / 2")
        .is_not_close_to_with_margin(3.14, (2. * f64::EPSILON, 3))
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected tau / 2 is not close to 3.14
  within a margin of epsilon=4.440892098500626e-16 and ulps=3
   but was: 3.14
  expected: 3.14
"]
    );
}
