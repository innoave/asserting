use super::*;
use crate::prelude::*;

#[test]
fn the_default_temporal_margin_is_zero_seconds() {
    assert_eq!(TemporalMargin::default(), TemporalMargin::Seconds(0));
}

#[test]
fn debug_string_of_999_nanoseconds() {
    assert_eq!(format!("{:?}", TemporalMargin::NanoSeconds(999)), "999ns");
}

#[test]
fn debug_string_of_999_microseconds() {
    assert_eq!(format!("{:?}", TemporalMargin::MicroSeconds(999)), "999µs");
}

#[test]
fn debug_string_of_999_milliseconds() {
    assert_eq!(format!("{:?}", TemporalMargin::MilliSeconds(999)), "999ms");
}

#[test]
fn debug_string_of_59_seconds() {
    assert_eq!(format!("{:?}", TemporalMargin::Seconds(59)), "59s");
}

#[test]
fn debug_string_of_59_minutes() {
    assert_eq!(format!("{:?}", TemporalMargin::Minutes(59)), "59m");
}

#[test]
fn debug_string_of_23_hours() {
    assert_eq!(format!("{:?}", TemporalMargin::Hours(23)), "23h");
}

#[test]
fn debug_string_of_365_days() {
    assert_eq!(format!("{:?}", TemporalMargin::Days(365)), "365d");
}

#[test]
fn debug_string_of_52_weeks() {
    assert_eq!(format!("{:?}", TemporalMargin::Weeks(52)), "52w");
}

#[test]
fn display_string_of_999_nanoseconds() {
    assert_eq!(TemporalMargin::NanoSeconds(999).to_string(), "999ns");
}

#[test]
fn display_string_of_999_microseconds() {
    assert_eq!(TemporalMargin::MicroSeconds(999).to_string(), "999µs");
}

#[test]
fn display_string_of_999_milliseconds() {
    assert_eq!(TemporalMargin::MilliSeconds(999).to_string(), "999ms");
}

#[test]
fn display_string_of_59_seconds() {
    assert_eq!(TemporalMargin::Seconds(59).to_string(), "59s");
}

#[test]
fn display_string_of_59_minutes() {
    assert_eq!(TemporalMargin::Minutes(59).to_string(), "59m");
}

#[test]
fn display_string_of_23_hours() {
    assert_eq!(TemporalMargin::Hours(23).to_string(), "23h");
}

#[test]
fn display_string_of_365_days() {
    assert_eq!(TemporalMargin::Days(365).to_string(), "365d");
}

#[test]
fn display_string_of_52_weeks() {
    assert_eq!(TemporalMargin::Weeks(52).to_string(), "52w");
}

#[test]
fn temporal_margin_of_999ns_to_nanoseconds() {
    let nanoseconds = TemporalMargin::NanoSeconds(999);
    assert_eq!(nanoseconds.to_nanoseconds(), 999);
}

#[test]
fn temporal_margin_of_999us_to_nanoseconds() {
    let nanoseconds = TemporalMargin::MicroSeconds(999);
    assert_eq!(nanoseconds.to_nanoseconds(), 999_000);
}

#[test]
fn temporal_margin_of_999ms_to_nanoseconds() {
    let nanoseconds = TemporalMargin::MilliSeconds(999);
    assert_eq!(nanoseconds.to_nanoseconds(), 999_000_000);
}

#[test]
fn temporal_margin_of_59s_to_nanoseconds() {
    let nanoseconds = TemporalMargin::Seconds(59);
    assert_eq!(nanoseconds.to_nanoseconds(), 59 * 1_000_000_000);
}

#[test]
fn temporal_margin_of_59m_to_nanoseconds() {
    let nanoseconds = TemporalMargin::Minutes(59);
    assert_eq!(nanoseconds.to_nanoseconds(), 59 * 60 * 1_000_000_000);
}

#[test]
fn temporal_margin_of_24h_to_nanoseconds() {
    let nanoseconds = TemporalMargin::Hours(24);
    assert_eq!(nanoseconds.to_nanoseconds(), 24 * 60 * 60 * 1_000_000_000);
}

#[test]
fn temporal_margin_of_366d_to_nanoseconds() {
    let nanoseconds = TemporalMargin::Days(366);
    assert_eq!(
        nanoseconds.to_nanoseconds(),
        366 * 24 * 60 * 60 * 1_000_000_000
    );
}

#[test]
fn temporal_margin_of_52w_nanoseconds() {
    let nanoseconds = TemporalMargin::Weeks(52);
    assert_eq!(
        nanoseconds.to_nanoseconds(),
        52 * 7 * 24 * 60 * 60 * 1_000_000_000
    );
}

#[test]
fn construct_temporal_margin_of_nanoseconds_from_i32() {
    assert_eq!(999_i32.nanoseconds(), TemporalMargin::NanoSeconds(999));
}

#[test]
fn construct_temporal_margin_of_microseconds_from_i32() {
    assert_eq!(999_i32.microseconds(), TemporalMargin::MicroSeconds(999));
}

#[test]
fn construct_temporal_margin_of_milliseconds_from_i32() {
    assert_eq!(999_i32.milliseconds(), TemporalMargin::MilliSeconds(999));
}

#[test]
fn construct_temporal_margin_of_seconds_from_i32() {
    assert_eq!(60_i32.seconds(), TemporalMargin::Seconds(60));
}

#[test]
fn construct_temporal_margin_of_minutes_from_i32() {
    assert_eq!(60_i32.minutes(), TemporalMargin::Minutes(60));
}

#[test]
fn construct_temporal_margin_of_hours_from_i32() {
    assert_eq!(24_i32.hours(), TemporalMargin::Hours(24));
}

#[test]
fn construct_temporal_margin_of_days_from_i32() {
    assert_eq!(365_i32.days(), TemporalMargin::Days(365));
}

#[test]
fn construct_temporal_margin_of_weeks_from_i32() {
    assert_eq!(52_i32.weeks(), TemporalMargin::Weeks(52));
}

#[test]
fn compare_temporal_margin_23h_to_1d() {
    assert_that(23.hours()).is_less_than(1.days());
}

#[test]
fn compare_temporal_margin_2000us_to_1ms() {
    assert_that(2000.microseconds()).is_greater_than(1.milliseconds());
}

#[test]
fn compare_temporal_margin_14d_to_2w() {
    assert_that(14.days()).is_equal_to(2.weeks());
}

#[test]
fn compare_temporal_margin_2h_to_121m() {
    assert_that(2.hours()).is_less_than(121.minutes());
}
