#![allow(clippy::expect_used)]

use crate::prelude::*;
use crate::temporal_margin::TemporalMargin;
use time::{Date, Month, SignedDuration, Time};

#[test]
fn temporal_margin_of_nanoseconds_to_signed_duration() {
    assert_that(TemporalMargin::NanoSeconds(999).to_signed_duration())
        .is_equal_to(SignedDuration::nanoseconds(999));
}

#[test]
fn temporal_margin_of_microseconds_to_signed_duration() {
    assert_that(TemporalMargin::MicroSeconds(-999).to_signed_duration())
        .is_equal_to(SignedDuration::microseconds(-999));
}

#[test]
fn temporal_margin_of_milliseconds_to_signed_duration() {
    assert_that(TemporalMargin::MilliSeconds(999).to_signed_duration())
        .is_equal_to(SignedDuration::milliseconds(999));
}

#[test]
fn temporal_margin_of_seconds_to_signed_duration() {
    assert_that(TemporalMargin::Seconds(60).to_signed_duration())
        .is_equal_to(SignedDuration::seconds(60));
}

#[test]
fn temporal_margin_of_minutes_to_signed_duration() {
    assert_that(TemporalMargin::Minutes(-60).to_signed_duration())
        .is_equal_to(SignedDuration::minutes(-60));
}

#[test]
fn temporal_margin_of_hours_to_signed_duration() {
    assert_that(TemporalMargin::Hours(24).to_signed_duration())
        .is_equal_to(SignedDuration::hours(24));
}

#[test]
fn temporal_margin_of_days_to_signed_duration() {
    assert_that(TemporalMargin::Days(366).to_signed_duration())
        .is_equal_to(SignedDuration::days(366));
}

#[test]
fn temporal_margin_of_weeks_to_signed_duration() {
    assert_that(TemporalMargin::Weeks(52).to_signed_duration())
        .is_equal_to(SignedDuration::weeks(52));
}

#[test]
fn time_is_equal_to() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    assert_that(subject)
        .is_equal_to(Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"));
}

#[test]
fn time_is_before() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    assert_that(subject)
        .is_before(Time::from_hms_milli(12, 34, 56, 790).expect("invalid time value"));
}

#[test]
fn time_is_after() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    assert_that(subject)
        .is_after(Time::from_hms_milli(12, 34, 56, 788).expect("invalid time value"));
}

#[test]
fn time_is_between() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    assert_that(subject).is_between(
        Time::from_hms_milli(12, 34, 56, 788).expect("invalid time value"),
        Time::from_hms_milli(12, 34, 56, 790).expect("invalid time value"),
    );
}

#[test]
fn time_is_close_to_within_milliseconds_exact_equal() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    assert_that(subject).is_close_to_with_margin(
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
        10.milliseconds(),
    );
}

#[test]
fn time_is_close_to_within_milliseconds_plus_margin() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    assert_that(subject).is_close_to_with_margin(
        Time::from_hms_milli(12, 34, 56, 799).expect("invalid time value"),
        10.milliseconds(),
    );
}

#[test]
fn time_is_close_to_within_milliseconds_minus_margin() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    assert_that(subject).is_close_to_with_margin(
        Time::from_hms_milli(12, 34, 56, 779).expect("invalid time value"),
        10.milliseconds(),
    );
}

#[test]
fn verify_time_is_close_to_within_milliseconds_minus_margin_fails() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Time::from_hms_milli(12, 34, 56, 778).expect("invalid time value"),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [r"expected subject to be close to 12:34:56.778 within 10ms
   but was: 12:34:56.789
  expected: 12:34:56.778
"]
    );
}

#[test]
fn verify_time_is_close_to_within_milliseconds_plus_margin_fails() {
    let subject = Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Time::from_hms_milli(12, 34, 56, 800).expect("invalid time value"),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [r"expected subject to be close to 12:34:56.8 within 10ms
   but was: 12:34:56.789
  expected: 12:34:56.8
"]
    );
}

#[test]
fn date_is_equal_to() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value");

    assert_that(subject).is_equal_to(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value"),
    );
}

#[test]
fn date_is_before() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 27).expect("invalid naive date value");

    assert_that(subject).is_before(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value"),
    );
}

#[test]
fn date_is_after() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value");

    assert_that(subject).is_after(
        Date::from_calendar_date(2023, Month::February, 27).expect("invalid naive date value"),
    );
}

#[test]
fn date_is_between() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 27).expect("invalid naive date value");

    assert_that(subject).is_between(
        Date::from_calendar_date(2023, Month::February, 26).expect("invalid naive date value"),
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value"),
    );
}

#[test]
fn date_is_close_to_within_days_exact_equal() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value");

    assert_that(subject).is_close_to_with_margin(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value"),
        1.days(),
    );
}

#[test]
fn date_is_close_to_within_days_plus_margin() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 27).expect("invalid naive date value");

    assert_that(subject).is_close_to_with_margin(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value"),
        1.days(),
    );
}

#[test]
fn date_is_close_to_within_days_minus_margin() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value");

    assert_that(subject).is_close_to_with_margin(
        Date::from_calendar_date(2023, Month::February, 27).expect("invalid naive date value"),
        1.days(),
    );
}

#[test]
fn verify_date_is_close_to_within_days_minus_margin_fails() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Date::from_calendar_date(2023, Month::February, 26).expect("invalid naive date value"),
            1.days(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [r"expected subject to be close to 2023-02-26 within 1d
   but was: 2023-02-28
  expected: 2023-02-26
"]
    );
}

#[test]
fn verify_date_is_close_to_within_days_plus_margin_fails() {
    let subject =
        Date::from_calendar_date(2023, Month::February, 26).expect("invalid naive date value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Date::from_calendar_date(2023, Month::February, 28).expect("invalid naive date value"),
            1.days(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [r"expected subject to be close to 2023-02-28 within 1d
   but was: 2023-02-26
  expected: 2023-02-28
"]
    );
}
