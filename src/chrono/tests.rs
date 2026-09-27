#![allow(clippy::expect_used)]

use crate::prelude::*;
use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

#[test]
fn naive_time_is_equal_to() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    assert_that(subject).is_equal_to(
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );
}

#[test]
fn naive_time_is_before() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    assert_that(subject).is_before(
        NaiveTime::from_hms_milli_opt(12, 34, 56, 790).expect("invalid naive time value"),
    );
}

#[test]
fn naive_time_is_after() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    assert_that(subject).is_after(
        NaiveTime::from_hms_milli_opt(12, 34, 56, 788).expect("invalid naive time value"),
    );
}

#[test]
fn naive_time_is_between() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    assert_that(subject).is_between(
        NaiveTime::from_hms_milli_opt(12, 34, 56, 788).expect("invalid naive time value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 790).expect("invalid naive time value"),
    );
}

#[test]
fn naive_time_is_close_to_within_milliseconds_exact_equal() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    assert_that(subject).is_close_to_with_margin(
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
        10.milliseconds(),
    );
}

#[test]
fn naive_time_is_close_to_within_milliseconds_plus_margin() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    assert_that(subject).is_close_to_with_margin(
        NaiveTime::from_hms_milli_opt(12, 34, 56, 799).expect("invalid naive time value"),
        10.milliseconds(),
    );
}

#[test]
fn naive_time_is_close_to_within_milliseconds_minus_margin() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    assert_that(subject).is_close_to_with_margin(
        NaiveTime::from_hms_milli_opt(12, 34, 56, 779).expect("invalid naive time value"),
        10.milliseconds(),
    );
}

#[test]
fn verify_naive_time_is_close_to_within_milliseconds_minus_margin_fails() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            NaiveTime::from_hms_milli_opt(12, 34, 56, 778).expect("invalid naive time value"),
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
fn verify_naive_time_is_close_to_within_milliseconds_plus_margin_fails() {
    let subject = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            NaiveTime::from_hms_milli_opt(12, 34, 56, 800).expect("invalid naive time value"),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [r"expected subject to be close to 12:34:56.800 within 10ms
   but was: 12:34:56.789
  expected: 12:34:56.800
"]
    );
}

#[test]
fn naive_date_is_equal_to() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value");

    assert_that(subject)
        .is_equal_to(NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value"));
}

#[test]
fn naive_date_is_before() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 27).expect("invalid naive date value");

    assert_that(subject)
        .is_before(NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value"));
}

#[test]
fn naive_date_is_after() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value");

    assert_that(subject)
        .is_after(NaiveDate::from_ymd_opt(2023, 2, 27).expect("invalid naive date value"));
}

#[test]
fn naive_date_is_between() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 27).expect("invalid naive date value");

    assert_that(subject).is_between(
        NaiveDate::from_ymd_opt(2023, 2, 26).expect("invalid naive date value"),
        NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value"),
    );
}

#[test]
fn naive_date_is_close_to_within_days_exact_equal() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value");

    assert_that(subject).is_close_to_with_margin(
        NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value"),
        1.days(),
    );
}

#[test]
fn naive_date_is_close_to_within_days_plus_margin() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 27).expect("invalid naive date value");

    assert_that(subject).is_close_to_with_margin(
        NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value"),
        1.days(),
    );
}

#[test]
fn naive_date_is_close_to_within_days_minus_margin() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value");

    assert_that(subject).is_close_to_with_margin(
        NaiveDate::from_ymd_opt(2023, 2, 27).expect("invalid naive date value"),
        1.days(),
    );
}

#[test]
fn verify_naive_date_is_close_to_within_days_minus_margin_fails() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            NaiveDate::from_ymd_opt(2023, 2, 26).expect("invalid naive date value"),
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
fn verify_naive_date_is_close_to_within_days_plus_margin_fails() {
    let subject = NaiveDate::from_ymd_opt(2023, 2, 26).expect("invalid naive date value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            NaiveDate::from_ymd_opt(2023, 2, 28).expect("invalid naive date value"),
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

#[test]
fn naive_datetime_is_equal_to() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    assert_that(subject).is_equal_to(NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    ));
}

#[test]
fn naive_datetime_is_before() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    assert_that(subject).is_before(NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 790).expect("invalid naive time value"),
    ));
}

#[test]
fn naive_datetime_is_after() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    assert_that(subject).is_after(NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 788).expect("invalid naive time value"),
    ));
}

#[test]
fn naive_datetime_is_between() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    assert_that(subject).is_between(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
            NaiveTime::from_hms_milli_opt(12, 34, 56, 788).expect("invalid naive time value"),
        ),
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
            NaiveTime::from_hms_milli_opt(12, 34, 56, 790).expect("invalid naive time value"),
        ),
    );
}

#[test]
fn naive_datetime_is_close_to_within_milliseconds_exact_equal() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    assert_that(subject).is_close_to_with_margin(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
            NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
        ),
        10.milliseconds(),
    );
}

#[test]
fn naive_datetime_is_close_to_within_milliseconds_plus_margin() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    assert_that(subject).is_close_to_with_margin(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
            NaiveTime::from_hms_milli_opt(12, 34, 56, 799).expect("invalid naive time value"),
        ),
        10.milliseconds(),
    );
}

#[test]
fn naive_datetime_is_close_to_within_milliseconds_minus_margin() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    assert_that(subject).is_close_to_with_margin(
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
            NaiveTime::from_hms_milli_opt(12, 34, 56, 779).expect("invalid naive time value"),
        ),
        10.milliseconds(),
    );
}

#[test]
fn verify_naive_datetime_is_close_to_within_milliseconds_minus_margin_fails() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
                NaiveTime::from_hms_milli_opt(12, 34, 56, 778).expect("invalid naive time value"),
            ),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:56.778 within 10ms
   but was: 2022-01-01T12:34:56.789
  expected: 2022-01-01T12:34:56.778
"
        ]
    );
}

#[test]
fn verify_naive_datetime_is_close_to_within_milliseconds_plus_margin_fails() {
    let subject = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
        NaiveTime::from_hms_milli_opt(12, 34, 56, 789).expect("invalid naive time value"),
    );

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2022, 1, 1).expect("invalid naive date value"),
                NaiveTime::from_hms_milli_opt(12, 34, 56, 800).expect("invalid naive time value"),
            ),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:56.800 within 10ms
   but was: 2022-01-01T12:34:56.789
  expected: 2022-01-01T12:34:56.800
"
        ]
    );
}

#[test]
fn datetime_utc_is_equal_to() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    assert_that(subject).is_equal_to(Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap());
}

#[test]
fn datetime_utc_is_before() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    assert_that(subject).is_before(Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 57).unwrap());
}

#[test]
fn datetime_utc_is_after() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    assert_that(subject).is_after(Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 55).unwrap());
}

#[test]
fn datetime_utc_is_between() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    assert_that(subject).is_between(
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
    );
}

#[test]
fn datetime_utc_is_close_to_within_milliseconds_exact_equal() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    assert_that(subject).is_close_to_with_margin(
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
        3.seconds(),
    );
}

#[test]
fn datetime_utc_is_close_to_within_milliseconds_plus_margin() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    assert_that(subject).is_close_to_with_margin(
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
        3.seconds(),
    );
}

#[test]
fn datetime_utc_is_close_to_within_milliseconds_minus_margin() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    assert_that(subject).is_close_to_with_margin(
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
        3.seconds(),
    );
}

#[test]
fn verify_datetime_utc_is_close_to_within_seconds_minus_margin_fails() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 52).unwrap(),
            3.seconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:52Z within 3s
   but was: 2022-01-01T12:34:56Z
  expected: 2022-01-01T12:34:52Z
"
        ]
    );
}

#[test]
fn verify_datetime_utc_is_close_to_within_seconds_plus_margin_fails() {
    let subject = Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap();

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Utc.with_ymd_and_hms(2022, 1, 1, 12, 35, 00).unwrap(),
            3.seconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:35:00Z within 3s
   but was: 2022-01-01T12:34:56Z
  expected: 2022-01-01T12:35:00Z
"
        ]
    );
}

#[test]
fn datetime_fixedoffset_is_close_to_within_minutes_a_datetime_utc() {
    let subject = FixedOffset::east_opt(3600)
        .expect("invalid fixed offset value")
        .with_ymd_and_hms(2022, 1, 1, 12, 34, 56)
        .unwrap();

    assert_that(subject).is_close_to_with_margin(
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
        60.minutes(),
    );
}

#[test]
fn verify_datetime_fixedoffset_is_close_to_within_minutes_a_datetime_utc_fails() {
    let subject = FixedOffset::east_opt(3600)
        .expect("invalid fixed offset value")
        .with_ymd_and_hms(2022, 1, 1, 12, 34, 56)
        .unwrap();

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
            59.minutes(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:56Z within 59m
   but was: 2022-01-01T12:34:56+01:00
  expected: 2022-01-01T12:34:56Z
"
        ]
    );
}
