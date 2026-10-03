#![allow(clippy::expect_used)]

use crate::prelude::*;
use crate::temporal::TemporalMargin;
use time::{Date, Month, OffsetDateTime, PlainDateTime, SignedDuration, Time, UtcOffset};

#[test]
fn temporal_margin_of_nanoseconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::NanoSeconds(999).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::nanoseconds(999));
}

#[test]
fn temporal_margin_of_microseconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::MicroSeconds(-999).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::microseconds(-999));
}

#[test]
fn temporal_margin_of_milliseconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::MilliSeconds(999).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::milliseconds(999));
}

#[test]
fn temporal_margin_of_seconds_to_duration() {
    let subject: SignedDuration = TemporalMargin::Seconds(60).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::seconds(60));
}

#[test]
fn temporal_margin_of_minutes_to_duration() {
    let subject: SignedDuration = TemporalMargin::Minutes(-60).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::minutes(-60));
}

#[test]
fn temporal_margin_of_hours_to_duration() {
    let subject: SignedDuration = TemporalMargin::Hours(24).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::hours(24));
}

#[test]
fn temporal_margin_of_days_to_duration() {
    let subject: SignedDuration = TemporalMargin::Days(366).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::days(366));
}

#[test]
fn temporal_margin_of_weeks_to_duration() {
    let subject: SignedDuration = TemporalMargin::Weeks(52).to_duration();
    assert_that(subject).is_equal_to(SignedDuration::weeks(52));
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
    let subject = Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value");

    assert_that(subject).is_equal_to(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value"),
    );
}

#[test]
fn date_is_before() {
    let subject = Date::from_calendar_date(2023, Month::February, 27).expect("invalid date value");

    assert_that(subject).is_before(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value"),
    );
}

#[test]
fn date_is_after() {
    let subject = Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value");

    assert_that(subject)
        .is_after(Date::from_calendar_date(2023, Month::February, 27).expect("invalid date value"));
}

#[test]
fn date_is_between() {
    let subject = Date::from_calendar_date(2023, Month::February, 27).expect("invalid date value");

    assert_that(subject).is_between(
        Date::from_calendar_date(2023, Month::February, 26).expect("invalid date value"),
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value"),
    );
}

#[test]
fn date_is_close_to_within_days_exact_equal() {
    let subject = Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value");

    assert_that(subject).is_close_to_with_margin(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value"),
        1.days(),
    );
}

#[test]
fn date_is_close_to_within_days_plus_margin() {
    let subject = Date::from_calendar_date(2023, Month::February, 27).expect("invalid date value");

    assert_that(subject).is_close_to_with_margin(
        Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value"),
        1.days(),
    );
}

#[test]
fn date_is_close_to_within_days_minus_margin() {
    let subject = Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value");

    assert_that(subject).is_close_to_with_margin(
        Date::from_calendar_date(2023, Month::February, 27).expect("invalid date value"),
        1.days(),
    );
}

#[test]
fn verify_date_is_close_to_within_days_minus_margin_fails() {
    let subject = Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Date::from_calendar_date(2023, Month::February, 26).expect("invalid date value"),
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
    let subject = Date::from_calendar_date(2023, Month::February, 26).expect("invalid date value");

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            Date::from_calendar_date(2023, Month::February, 28).expect("invalid date value"),
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
fn plain_datetime_is_equal_to() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    assert_that(subject).is_equal_to(PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    ));
}

#[test]
fn plain_datetime_is_before() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    assert_that(subject).is_before(PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 790).expect("invalid time value"),
    ));
}

#[test]
fn plain_datetime_is_after() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    assert_that(subject).is_after(PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 788).expect("invalid time value"),
    ));
}

#[test]
fn plain_datetime_is_between() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    assert_that(subject).is_between(
        PlainDateTime::new(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms_milli(12, 34, 56, 788).expect("invalid time value"),
        ),
        PlainDateTime::new(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms_milli(12, 34, 56, 790).expect("invalid time value"),
        ),
    );
}

#[test]
fn plain_datetime_is_close_to_within_milliseconds_exact_equal() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    assert_that(subject).is_close_to_with_margin(
        PlainDateTime::new(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
        ),
        10.milliseconds(),
    );
}

#[test]
fn plain_datetime_is_close_to_within_milliseconds_plus_margin() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    assert_that(subject).is_close_to_with_margin(
        PlainDateTime::new(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms_milli(12, 34, 56, 799).expect("invalid time value"),
        ),
        10.milliseconds(),
    );
}

#[test]
fn plain_datetime_is_close_to_within_milliseconds_minus_margin() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    assert_that(subject).is_close_to_with_margin(
        PlainDateTime::new(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms_milli(12, 34, 56, 779).expect("invalid time value"),
        ),
        10.milliseconds(),
    );
}

#[test]
fn verify_plain_datetime_is_close_to_within_milliseconds_minus_margin_fails() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            PlainDateTime::new(
                Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
                Time::from_hms_milli(12, 34, 56, 778).expect("invalid time value"),
            ),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01 12:34:56.778 within 10ms
   but was: 2022-01-01 12:34:56.789
  expected: 2022-01-01 12:34:56.778
"
        ]
    );
}

#[test]
fn verify_plain_datetime_is_close_to_within_milliseconds_plus_margin_fails() {
    let subject = PlainDateTime::new(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms_milli(12, 34, 56, 789).expect("invalid time value"),
    );

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            PlainDateTime::new(
                Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
                Time::from_hms_milli(12, 34, 56, 800).expect("invalid time value"),
            ),
            10.milliseconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01 12:34:56.8 within 10ms
   but was: 2022-01-01 12:34:56.789
  expected: 2022-01-01 12:34:56.8
"
        ]
    );
}

#[test]
fn offsetdatetime_is_equal_to() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    );

    assert_that(subject).is_equal_to(OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    ));
}

#[test]
fn offsetdatetime_is_before() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    );

    assert_that(subject).is_before(OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 57).expect("invalid time value"),
        UtcOffset::UTC,
    ));
}

#[test]
fn offsetdatetime_is_after() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    );

    assert_that(subject).is_after(OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 55).expect("invalid time value"),
        UtcOffset::UTC,
    ));
}

#[test]
fn offsetdatetime_is_between() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    );

    assert_that(subject).is_between(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms(12, 34, 56).expect("invalid time value"),
            UtcOffset::UTC,
        ),
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms(12, 34, 56).expect("invalid time value"),
            UtcOffset::UTC,
        ),
    );
}

#[test]
fn offsetdatetime_is_close_to_within_milliseconds_exact_equal() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    );

    assert_that(subject).is_close_to_with_margin(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms(12, 34, 56).expect("invalid time value"),
            UtcOffset::UTC,
        ),
        3.milliseconds(),
    );
}

#[test]
fn offsetdatetime_is_close_to_within_milliseconds_plus_margin() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    ) + SignedDuration::milliseconds(3);

    assert_that(subject).is_close_to_with_margin(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms(12, 34, 56).expect("invalid time value"),
            UtcOffset::UTC,
        ),
        3.milliseconds(),
    );
}

#[test]
fn offsetdatetime_is_close_to_within_milliseconds_minus_margin() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::UTC,
    ) - SignedDuration::milliseconds(3);

    assert_that(subject).is_close_to_with_margin(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms(12, 34, 56).expect("invalid time value"),
            UtcOffset::UTC,
        ),
        3.seconds(),
    );
}

#[test]
fn verify_offsetdatetime_is_close_to_within_seconds_minus_margin_fails() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 53).expect("invalid time value"),
        UtcOffset::UTC,
    ) - SignedDuration::milliseconds(3);

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            OffsetDateTime::new_in_offset(
                Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
                Time::from_hms(12, 34, 56).expect("invalid time value"),
                UtcOffset::UTC,
            ),
            3.seconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01 12:34:56.0 +00:00:00 within 3s
   but was: 2022-01-01 12:34:52.997 +00:00:00
  expected: 2022-01-01 12:34:56.0 +00:00:00
"
        ]
    );
}

#[test]
fn verify_offsetdatetime_is_close_to_within_seconds_plus_margin_fails() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 35, 0).expect("invalid time value"),
        UtcOffset::UTC,
    );

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            OffsetDateTime::new_in_offset(
                Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
                Time::from_hms(12, 34, 56).expect("invalid time value"),
                UtcOffset::UTC,
            ),
            3.seconds(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01 12:34:56.0 +00:00:00 within 3s
   but was: 2022-01-01 12:35:00.0 +00:00:00
  expected: 2022-01-01 12:34:56.0 +00:00:00
"
        ]
    );
}

#[test]
fn offsetdatetime_is_close_to_within_minutes_different_utc_offset() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::from_hms(1, 0, 0).expect("invalid utc offset value"),
    );

    assert_that(subject).is_close_to_with_margin(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
            Time::from_hms(12, 34, 56).expect("invalid time value"),
            UtcOffset::UTC,
        ),
        60.minutes(),
    );
}

#[test]
fn verify_offsetdatetime_is_close_to_within_minutes_different_utc_offset_fails() {
    let subject = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::from_hms(1, 0, 0).expect("invalid utc offset value"),
    );

    let failures = verify_that(subject)
        .is_close_to_with_margin(
            OffsetDateTime::new_in_offset(
                Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
                Time::from_hms(12, 34, 56).expect("invalid time value"),
                UtcOffset::UTC,
            ),
            59.minutes(),
        )
        .display_failures();

    assert_eq!(
        failures,
        [
            r"expected subject to be close to 2022-01-01 12:34:56.0 +00:00:00 within 59m
   but was: 2022-01-01 12:34:56.0 +01:00:00
  expected: 2022-01-01 12:34:56.0 +00:00:00
"
        ]
    );
}

#[test]
fn extracting_ref_time_is_close_to_within_seconds() {
    struct MyTime(Time);

    let subject = MyTime(Time::from_hms(12, 34, 56).expect("invalid time value"));

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            Time::from_hms(12, 34, 57).expect("invalid time value"),
            2.seconds(),
        );
}

#[test]
fn extracting_ref_date_is_close_to_within_hours() {
    struct MyDate(Date);

    let subject =
        MyDate(Date::from_calendar_date(2024, Month::December, 8).expect("invalid date value"));

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            Date::from_calendar_date(2024, Month::December, 9).expect("invalid date value"),
            24.hours(),
        );
}

#[test]
fn extracting_ref_plain_datetime_is_close_to_within_milliseconds() {
    struct MyPlainDateTime(PlainDateTime);

    let subject = MyPlainDateTime(PlainDateTime::new(
        Date::from_calendar_date(2024, Month::December, 8).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
    ));

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            PlainDateTime::new(
                Date::from_calendar_date(2024, Month::December, 8).expect("invalid date value"),
                Time::from_hms(12, 34, 55).expect("invalid time value"),
            ),
            1000.milliseconds(),
        );
}

#[test]
fn extracting_ref_offset_datetime_is_close_to_within_minutes() {
    struct MyOffsetDateTime(OffsetDateTime);

    let subject = MyOffsetDateTime(OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
        Time::from_hms(12, 34, 56).expect("invalid time value"),
        UtcOffset::from_hms(1, 0, 0).expect("invalid utc offset value"),
    ));

    assert_that(&subject)
        .extracting_ref("0", |s| &s.0)
        .is_close_to_with_margin(
            OffsetDateTime::new_in_offset(
                Date::from_calendar_date(2022, Month::January, 1).expect("invalid date value"),
                Time::from_hms(12, 34, 56).expect("invalid time value"),
                UtcOffset::from_hms(2, 0, 0).expect("invalid utc offset value"),
            ),
            60.minutes(),
        );
}
