#![allow(clippy::expect_used)]

use crate::prelude::*;
use crate::temporal::TemporalMargin;
use chrono::{
    DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Utc,
};

#[test]
fn temporal_margin_of_nanoseconds_to_duration() {
    let subject: TimeDelta = TemporalMargin::NanoSeconds(999).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::nanoseconds(999));
}

#[test]
fn temporal_margin_of_microseconds_to_duration() {
    let subject: TimeDelta = TemporalMargin::MicroSeconds(-999).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::microseconds(-999));
}

#[test]
fn temporal_margin_of_milliseconds_to_duration() {
    let subject: TimeDelta = TemporalMargin::MilliSeconds(999).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::milliseconds(999));
}

#[test]
fn temporal_margin_of_seconds_to_duration() {
    let subject: TimeDelta = TemporalMargin::Seconds(60).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::seconds(60));
}

#[test]
fn temporal_margin_of_minutes_to_duration() {
    let subject: TimeDelta = TemporalMargin::Minutes(-60).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::minutes(-60));
}

#[test]
fn temporal_margin_of_hours_to_duration() {
    let subject: TimeDelta = TemporalMargin::Hours(24).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::hours(24));
}

#[test]
fn temporal_margin_of_days_to_duration() {
    let subject: TimeDelta = TemporalMargin::Days(366).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::days(366));
}

#[test]
fn temporal_margin_of_weeks_to_duration() {
    let subject: TimeDelta = TemporalMargin::Weeks(52).to_duration();
    assert_that(subject).is_equal_to(TimeDelta::weeks(52));
}

#[test]
fn temporal_margin_of_nanoseconds_to_time_delta() {
    assert_that(TemporalMargin::NanoSeconds(999).to_time_delta())
        .is_equal_to(TimeDelta::nanoseconds(999));
}

#[test]
fn temporal_margin_of_microseconds_to_time_delta() {
    assert_that(TemporalMargin::MicroSeconds(-999).to_time_delta())
        .is_equal_to(TimeDelta::microseconds(-999));
}

#[test]
fn temporal_margin_of_milliseconds_to_time_delta() {
    assert_that(TemporalMargin::MilliSeconds(999).to_time_delta())
        .is_equal_to(TimeDelta::milliseconds(999));
}

#[test]
fn temporal_margin_of_seconds_to_time_delta() {
    assert_that(TemporalMargin::Seconds(60).to_time_delta()).is_equal_to(TimeDelta::seconds(60));
}

#[test]
fn temporal_margin_of_minutes_to_time_delta() {
    assert_that(TemporalMargin::Minutes(-60).to_time_delta()).is_equal_to(TimeDelta::minutes(-60));
}

#[test]
fn temporal_margin_of_hours_to_time_delta() {
    assert_that(TemporalMargin::Hours(24).to_time_delta()).is_equal_to(TimeDelta::hours(24));
}

#[test]
fn temporal_margin_of_days_to_time_delta() {
    assert_that(TemporalMargin::Days(366).to_time_delta()).is_equal_to(TimeDelta::days(366));
}

#[test]
fn temporal_margin_of_weeks_to_time_delta() {
    assert_that(TemporalMargin::Weeks(52).to_time_delta()).is_equal_to(TimeDelta::weeks(52));
}

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
        3.milliseconds(),
    );
}

#[test]
fn datetime_utc_is_close_to_within_milliseconds_plus_margin() {
    let subject =
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap() + TimeDelta::milliseconds(3);

    assert_that(subject).is_close_to_with_margin(
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
        3.milliseconds(),
    );
}

#[test]
fn datetime_utc_is_close_to_within_milliseconds_minus_margin() {
    let subject =
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap() - TimeDelta::milliseconds(3);

    assert_that(subject).is_close_to_with_margin(
        Utc.with_ymd_and_hms(2022, 1, 1, 12, 34, 56).unwrap(),
        3.milliseconds(),
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

#[test]
fn extracting_ref_naive_time_is_close_to_within_seconds() {
    struct MyNaiveTime(NaiveTime);

    let subject =
        MyNaiveTime(NaiveTime::from_hms_opt(12, 34, 56).expect("invalid naive time value"));

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            NaiveTime::from_hms_opt(12, 34, 57).expect("invalid naive time value"),
            2.seconds(),
        );
}

#[test]
fn extracting_ref_naive_date_is_close_to_within_hours() {
    struct MyNaiveDate(NaiveDate);

    let subject =
        MyNaiveDate(NaiveDate::from_ymd_opt(2024, 12, 8).expect("invalid naive date value"));

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            NaiveDate::from_ymd_opt(2024, 12, 9).expect("invalid naive date value"),
            24.hours(),
        );
}

#[test]
fn extracting_ref_naive_datetime_is_close_to_within_milliseconds() {
    struct MyNaiveDateTime(NaiveDateTime);

    let subject = MyNaiveDateTime(NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 12, 8).expect("invalid naive date value"),
        NaiveTime::from_hms_opt(12, 34, 56).expect("invalid naive time value"),
    ));

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 12, 8).expect("invalid naive date value"),
                NaiveTime::from_hms_opt(12, 34, 55).expect("invalid naive time value"),
            ),
            1000.milliseconds(),
        );
}

#[test]
fn extracting_ref_datetime_utc_is_close_to_within_minutes() {
    struct MyDateTime(DateTime<Utc>);

    let subject = MyDateTime(Utc.with_ymd_and_hms(2023, 6, 30, 12, 34, 56).unwrap());

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            FixedOffset::east_opt(3600)
                .expect("invalid fixed offset value")
                .with_ymd_and_hms(2023, 6, 30, 12, 34, 56)
                .unwrap(),
            60.minutes(),
        );
}

#[test]
fn extracting_ref_datetime_fixed_offset_is_close_to_within_minutes() {
    struct MyDateTime(DateTime<FixedOffset>);

    let subject = MyDateTime(
        FixedOffset::east_opt(3600)
            .expect("invalid fixed offset value")
            .with_ymd_and_hms(2023, 6, 30, 12, 34, 56)
            .unwrap(),
    );

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            Utc.with_ymd_and_hms(2023, 6, 30, 12, 34, 56).unwrap(),
            60.minutes(),
        );
}
