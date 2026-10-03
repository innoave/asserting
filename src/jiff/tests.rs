#![allow(clippy::expect_used)]

use crate::prelude::*;
use crate::temporal::TemporalMargin;
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
