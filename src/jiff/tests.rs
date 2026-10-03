use crate::prelude::*;
use crate::temporal_margin::TemporalMargin;
use jiff::Span;

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

#[cfg(not(feature = "time"))]
mod signed_duration {
    use crate::prelude::*;
    use crate::temporal_margin::TemporalMargin;
    use jiff::SignedDuration;

    #[test]
    fn temporal_margin_of_nanoseconds_to_signed_duration() {
        assert_that(TemporalMargin::NanoSeconds(999).to_signed_duration())
            .is_equal_to(SignedDuration::from_nanos(999));
    }

    #[test]
    fn temporal_margin_of_microseconds_to_signed_duration() {
        assert_that(TemporalMargin::MicroSeconds(-999).to_signed_duration())
            .is_equal_to(SignedDuration::from_micros(-999));
    }

    #[test]
    fn temporal_margin_of_milliseconds_to_signed_duration() {
        assert_that(TemporalMargin::MilliSeconds(999).to_signed_duration())
            .is_equal_to(SignedDuration::from_millis(999));
    }

    #[test]
    fn temporal_margin_of_seconds_to_signed_duration() {
        assert_that(TemporalMargin::Seconds(60).to_signed_duration())
            .is_equal_to(SignedDuration::from_secs(60));
    }

    #[test]
    fn temporal_margin_of_minutes_to_signed_duration() {
        assert_that(TemporalMargin::Minutes(-60).to_signed_duration())
            .is_equal_to(SignedDuration::from_mins(-60));
    }

    #[test]
    fn temporal_margin_of_hours_to_signed_duration() {
        assert_that(TemporalMargin::Hours(24).to_signed_duration())
            .is_equal_to(SignedDuration::from_hours(24));
    }

    #[test]
    fn temporal_margin_of_days_to_signed_duration() {
        assert_that(TemporalMargin::Days(366).to_signed_duration())
            .is_equal_to(SignedDuration::from_hours(366 * 24));
    }

    #[test]
    fn temporal_margin_of_weeks_to_signed_duration() {
        assert_that(TemporalMargin::Weeks(52).to_signed_duration())
            .is_equal_to(SignedDuration::from_hours(52 * 7 * 24));
    }
}
