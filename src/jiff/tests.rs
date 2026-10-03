#![allow(clippy::expect_used)]

use crate::prelude::*;
use crate::temporal::TemporalMargin;
use jiff::civil::{date, datetime, time};
use jiff::{SignedDuration, Span, Timestamp};

#[test]
fn temporal_margin_of_nanoseconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::NanoSeconds(999).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_nanos(999));
}

#[test]
fn temporal_margin_of_microseconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::MicroSeconds(-999).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_micros(-999));
}

#[test]
fn temporal_margin_of_milliseconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::MilliSeconds(999).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_millis(999));
}

#[test]
fn temporal_margin_of_seconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::Seconds(60).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_secs(60));
}

#[test]
fn temporal_margin_of_minutes_to_duration() {
    let subject: SignedDuration = TemporalMargin::Minutes(-60).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_mins(-60));
}

#[test]
fn temporal_margin_of_hours_to_duration() {
    let subject: SignedDuration = TemporalMargin::Hours(24).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_hours(24));
}

#[test]
fn temporal_margin_of_days_to_duration() {
    let subject: SignedDuration = TemporalMargin::Days(366).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_hours(366 * 24));
}

#[test]
fn temporal_margin_of_weeks_to_duration() {
    let subject: SignedDuration = TemporalMargin::Weeks(52).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::from_hours(52 * 7 * 24));
}

#[test]
fn temporal_margin_of_nanoseconds_to_span() {
    assert_that(TemporalMargin::NanoSeconds(999).to_span())
        .is_equal_to(Span::new().nanoseconds(999).fieldwise());
}

#[test]
fn temporal_margin_of_microseconds_to_span() {
    assert_that(TemporalMargin::MicroSeconds(-999).to_span())
        .is_equal_to(Span::new().microseconds(-999).fieldwise());
}

#[test]
fn temporal_margin_of_milliseconds_to_span() {
    assert_that(TemporalMargin::MilliSeconds(999).to_span())
        .is_equal_to(Span::new().milliseconds(999).fieldwise());
}

#[test]
fn temporal_margin_of_seconds_to_span() {
    assert_that(TemporalMargin::Seconds(60).to_span())
        .is_equal_to(Span::new().seconds(60).fieldwise());
}

#[test]
fn temporal_margin_of_minutes_to_span() {
    assert_that(TemporalMargin::Minutes(-60).to_span())
        .is_equal_to(Span::new().minutes(-60).fieldwise());
}

#[test]
fn temporal_margin_of_hours_to_span() {
    assert_that(TemporalMargin::Hours(24).to_span()).is_equal_to(Span::new().hours(24).fieldwise());
}

#[test]
fn temporal_margin_of_days_to_span() {
    assert_that(TemporalMargin::Days(366).to_span()).is_equal_to(Span::new().days(366).fieldwise());
}

#[test]
fn temporal_margin_of_weeks_to_span() {
    assert_that(TemporalMargin::Weeks(52).to_span()).is_equal_to(Span::new().weeks(52).fieldwise());
}

#[test]
fn timestamp_is_equal_to() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    assert_that(subject).is_equal_to(Timestamp::constant(123_456, 987_654_321));
}

#[test]
fn timestamp_is_before() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    assert_that(subject)
        .is_before(Timestamp::new(123_456, 987_654_322).expect("invalid timestamp value"));
}

#[test]
fn timestamp_is_after() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    assert_that(subject)
        .is_after(Timestamp::new(123_456, 987_654_320).expect("invalid timestamp value"));
}

#[test]
fn timestamp_is_between() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    assert_that(subject).is_between(
        Timestamp::new(123_455, 987_654_321).expect("invalid timestamp value"),
        Timestamp::new(123_457, 987_654_321).expect("invalid timestamp value"),
    );
}

#[test]
fn timestamp_is_close_to_within_nanoseconds_exact_equal() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    assert_that(subject).is_close_to_with_margin(
        Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value"),
        10.nanoseconds(),
    );
}

#[test]
fn timestamp_is_close_to_within_nanoseconds_plus_margin() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    assert_that(subject).is_close_to_with_margin(
        Timestamp::new(123_456, 987_654_311).expect("invalid timestamp value"),
        10.nanoseconds(),
    );
}

#[test]
fn timestamp_is_close_to_within_nanoseconds_minus_margin() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    assert_that(subject).is_close_to_with_margin(
        Timestamp::new(123_456, 987_654_331).expect("invalid timestamp value"),
        10.nanoseconds(),
    );
}

#[test]
fn verify_timestamp_is_close_to_within_nanoseconds_minus_margin_fails() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Timestamp::new(123_456, 987_654_332).expect("invalid timestamp value"),
            10.nanoseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 1970-01-02T10:17:36.987654332Z within 10ns
   but was: 1970-01-02T10:17:36.987654321Z
  expected: 1970-01-02T10:17:36.987654332Z
"
        ]
    );
}

#[test]
fn verify_timestamp_is_close_to_within_nanoseconds_plus_margin_fails() {
    let subject = Timestamp::new(123_456, 987_654_321).expect("invalid timestamp value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Timestamp::new(123_456, 987_654_310).expect("invalid timestamp value"),
            10.nanoseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 1970-01-02T10:17:36.98765431Z within 10ns
   but was: 1970-01-02T10:17:36.987654321Z
  expected: 1970-01-02T10:17:36.98765431Z
"
        ]
    );
}

#[test]
fn civil_time_is_equal_to() {
    let subject = time(12, 34, 56, 789_000_000);

    assert_that(subject).is_equal_to(time(12, 34, 56, 789_000_000));
}

#[test]
fn civil_time_is_before() {
    let subject = time(12, 34, 56, 789_000_000);

    assert_that(subject).is_before(time(12, 34, 56, 790_000_000));
}

#[test]
fn civil_time_is_after() {
    let subject = time(12, 34, 56, 789_000_000);

    assert_that(subject).is_after(time(12, 34, 56, 788_000_000));
}

#[test]
fn civil_time_is_between() {
    let subject = time(12, 34, 56, 789_000_000);

    assert_that(subject).is_between(time(12, 34, 56, 788_000_000), time(12, 34, 56, 790_000_000));
}

#[test]
fn civil_time_is_close_to_within_milliseconds_exact_equal() {
    let subject = time(12, 34, 56, 789_000_000);

    assert_that(subject).is_close_to_with_margin(time(12, 34, 56, 789_000_000), 10.milliseconds());
}

#[test]
fn civil_time_is_close_to_within_milliseconds_plus_margin() {
    let subject = time(12, 34, 56, 789_000_000);

    assert_that(subject).is_close_to_with_margin(time(12, 34, 56, 799_000_000), 10.milliseconds());
}

#[test]
fn civil_time_is_close_to_within_milliseconds_minus_margin() {
    let subject = time(12, 34, 56, 789_000_000);

    assert_that(subject).is_close_to_with_margin(time(12, 34, 56, 779_000_000), 10.milliseconds());
}

#[test]
fn verify_civil_time_is_close_to_within_milliseconds_minus_margin_fails() {
    let subject = time(12, 34, 56, 789_000_000);

    let failures = verify_that(subject)
        .is_close_to_with_margin(time(12, 34, 56, 778_000_000), 10.milliseconds())
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
fn verify_civil_time_is_close_to_within_milliseconds_plus_margin_fails() {
    let subject = time(12, 34, 56, 789_000_000);

    let failures = verify_that(subject)
        .is_close_to_with_margin(time(12, 34, 56, 800_000_000), 10.milliseconds())
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
fn civil_date_is_equal_to() {
    let subject = date(2023, 2, 28);

    assert_that(subject).is_equal_to(date(2023, 2, 28));
}

#[test]
fn civil_date_is_before() {
    let subject = date(2023, 2, 27);

    assert_that(subject).is_before(date(2023, 2, 28));
}

#[test]
fn civil_date_is_after() {
    let subject = date(2023, 2, 28);

    assert_that(subject).is_after(date(2023, 2, 27));
}

#[test]
fn civil_date_is_between() {
    let subject = date(2023, 2, 27);

    assert_that(subject).is_between(date(2023, 2, 26), date(2023, 2, 28));
}

#[test]
fn civil_date_is_close_to_within_days_exact_equal() {
    let subject = date(2023, 2, 28);

    assert_that(subject).is_close_to_with_margin(date(2023, 2, 28), 1.days());
}

#[test]
fn civil_date_is_close_to_within_days_plus_margin() {
    let subject = date(2023, 2, 27);

    assert_that(subject).is_close_to_with_margin(date(2023, 2, 28), 1.days());
}

#[test]
fn civil_date_is_close_to_within_days_minus_margin() {
    let subject = date(2023, 2, 28);

    assert_that(subject).is_close_to_with_margin(date(2023, 2, 27), 1.days());
}

#[test]
fn verify_civil_date_is_close_to_within_days_minus_margin_fails() {
    let subject = date(2023, 2, 28);

    let failures = verify_that(subject)
        .is_close_to_with_margin(date(2023, 2, 26), 1.days())
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
fn verify_civil_date_is_close_to_within_days_plus_margin_fails() {
    let subject = date(2023, 2, 26);

    let failures = verify_that(subject)
        .is_close_to_with_margin(date(2023, 2, 28), 1.days())
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
fn civil_datetime_is_equal_to() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    assert_that(subject).is_equal_to(datetime(2022, 1, 1, 12, 34, 56, 789_000_000));
}

#[test]
fn civil_datetime_is_before() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    assert_that(subject).is_before(datetime(2022, 1, 1, 12, 34, 56, 790_000_000));
}

#[test]
fn civil_datetime_is_after() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    assert_that(subject).is_after(datetime(2022, 1, 1, 12, 34, 56, 788_000_000));
}

#[test]
fn civil_datetime_is_between() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    assert_that(subject).is_between(
        datetime(2022, 1, 1, 12, 34, 56, 788_000_000),
        datetime(2022, 1, 1, 12, 34, 56, 790_000_000),
    );
}

#[test]
fn civil_datetime_is_close_to_within_milliseconds_exact_equal() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    assert_that(subject).is_close_to_with_margin(
        datetime(2022, 1, 1, 12, 34, 56, 789_000_000),
        10.milliseconds(),
    );
}

#[test]
fn civil_datetime_is_close_to_within_milliseconds_plus_margin() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    assert_that(subject).is_close_to_with_margin(
        datetime(2022, 1, 1, 12, 34, 56, 799_000_000),
        10.milliseconds(),
    );
}

#[test]
fn civil_datetime_is_close_to_within_milliseconds_minus_margin() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    assert_that(subject).is_close_to_with_margin(
        datetime(2022, 1, 1, 12, 34, 56, 779_000_000),
        10.milliseconds(),
    );
}

#[test]
fn verify_civil_datetime_is_close_to_within_milliseconds_minus_margin_fails() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            datetime(2022, 1, 1, 12, 34, 56, 778_000_000),
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
fn verify_civil_datetime_is_close_to_within_milliseconds_plus_margin_fails() {
    let subject = datetime(2022, 1, 1, 12, 34, 56, 789_000_000);

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            datetime(2022, 1, 1, 12, 34, 56, 800_000_000),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:56.8 within 10ms
   but was: 2022-01-01T12:34:56.789
  expected: 2022-01-01T12:34:56.8
"
        ]
    );
}

#[test]
fn zoned_is_equal_to() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("UTC")
        .expect("invalid zoned value");

    assert_that(subject).is_equal_to(
        date(2022, 1, 1)
            .at(12, 34, 56, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
    );
}

#[test]
fn zoned_is_before() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value");

    assert_that(subject).is_before(
        date(2022, 1, 1)
            .at(12, 34, 57, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
    );
}

#[test]
fn zoned_is_after() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value");

    assert_that(subject).is_after(
        date(2022, 1, 1)
            .at(12, 34, 55, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
    );
}

#[test]
fn zoned_is_between() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value");

    assert_that(subject).is_between(
        date(2022, 1, 1)
            .at(12, 34, 56, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
        date(2022, 1, 1)
            .at(12, 34, 56, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
    );
}

#[test]
fn zoned_is_close_to_within_milliseconds_exact_equal() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value");

    assert_that(subject).is_close_to_with_margin(
        date(2022, 1, 1)
            .at(12, 34, 56, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
        3.milliseconds(),
    );
}

#[test]
fn zoned_is_close_to_within_milliseconds_plus_margin() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value")
        + SignedDuration::from_millis(3);

    assert_that(subject).is_close_to_with_margin(
        date(2022, 1, 1)
            .at(12, 34, 56, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
        3.milliseconds(),
    );
}

#[test]
fn zoned_is_close_to_within_milliseconds_minus_margin() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value")
        - SignedDuration::from_millis(3);

    assert_that(subject).is_close_to_with_margin(
        date(2022, 1, 1)
            .at(12, 34, 56, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
        3.seconds(),
    );
}

#[test]
fn verify_zoned_is_close_to_within_seconds_minus_margin_fails() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 53, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value")
        - SignedDuration::from_millis(3);

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            date(2022, 1, 1)
                .at(12, 34, 56, 0)
                .in_tz("UTC")
                .expect("invalid Zoned value"),
            3.seconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:56+00:00[UTC] within 3s
   but was: 2022-01-01T12:34:52.997+00:00[UTC]
  expected: 2022-01-01T12:34:56+00:00[UTC]
"
        ]
    );
}

#[test]
fn verify_zoned_is_close_to_within_seconds_plus_margin_fails() {
    let subject = date(2022, 1, 1)
        .at(12, 35, 0, 0)
        .in_tz("UTC")
        .expect("invalid Zoned value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            date(2022, 1, 1)
                .at(12, 34, 56, 0)
                .in_tz("UTC")
                .expect("invalid Zoned value"),
            3.seconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:56+00:00[UTC] within 3s
   but was: 2022-01-01T12:35:00+00:00[UTC]
  expected: 2022-01-01T12:34:56+00:00[UTC]
"
        ]
    );
}

#[test]
fn zoned_is_close_to_within_minutes_different_utc_offset() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("Europe/Vienna")
        .expect("invalid Zoned value");

    assert_that(subject).is_close_to_with_margin(
        date(2022, 1, 1)
            .at(12, 34, 56, 0)
            .in_tz("UTC")
            .expect("invalid Zoned value"),
        60.minutes(),
    );
}

#[test]
fn verify_zoned_is_close_to_within_minutes_different_utc_offset_fails() {
    let subject = date(2022, 1, 1)
        .at(12, 34, 56, 0)
        .in_tz("Europe/Vienna")
        .expect("invalid utc offset value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            date(2022, 1, 1)
                .at(12, 34, 56, 0)
                .in_tz("UTC")
                .expect("invalid Zoned value"),
            59.minutes(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01T12:34:56+00:00[UTC] within 59m
   but was: 2022-01-01T12:34:56+01:00[Europe/Vienna]
  expected: 2022-01-01T12:34:56+00:00[UTC]
"
        ]
    );
}
