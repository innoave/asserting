use crate::temporal_margin::TemporalMargin;
#[cfg(not(feature = "time"))]
use jiff::SignedDuration;
use jiff::Span;

#[cfg(test)]
mod tests;

impl TemporalMargin {
    /// Converts this temporal margin to a [`Span`] value.
    pub fn to_span(self) -> Span {
        use jiff::ToSpan;
        match self {
            Self::NanoSeconds(nanos) => nanos.nanoseconds(),
            Self::MicroSeconds(micros) => micros.microseconds(),
            Self::MilliSeconds(millis) => millis.milliseconds(),
            Self::Seconds(seconds) => seconds.seconds(),
            Self::Minutes(minutes) => minutes.minutes(),
            Self::Hours(hours) => hours.hours(),
            Self::Days(days) => days.days(),
            Self::Weeks(weeks) => weeks.weeks(),
        }
    }

    /// Converts this temporal margin to a [`SignedDuration`] value.
    #[cfg(not(feature = "time"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "time"))))]
    pub fn to_signed_duration(self) -> SignedDuration {
        match self {
            Self::NanoSeconds(nanos) => SignedDuration::from_nanos(i64::from(nanos)),
            Self::MicroSeconds(micros) => SignedDuration::from_micros(i64::from(micros)),
            Self::MilliSeconds(millis) => SignedDuration::from_millis(i64::from(millis)),
            Self::Seconds(seconds) => SignedDuration::from_secs(i64::from(seconds)),
            Self::Minutes(minutes) => SignedDuration::from_mins(i64::from(minutes)),
            Self::Hours(hours) => SignedDuration::from_hours(i64::from(hours)),
            Self::Days(days) => SignedDuration::from_hours(i64::from(days) * 24),
            Self::Weeks(weeks) => SignedDuration::from_hours(i64::from(weeks) * 7 * 24),
        }
    }
}
