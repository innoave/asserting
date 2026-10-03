use crate::assertions::AssertIsCloseToWithinMargin;
use crate::colored::mark_diff;
use crate::expectations::{IsCloseTo, is_close_to, not};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::temporal::{TemporalMargin, ToDuration};
use jiff::{SignedDuration, Span, Timestamp};

#[cfg(test)]
mod tests;

impl ToDuration<SignedDuration> for TemporalMargin {
    fn to_duration(self) -> SignedDuration {
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
}

impl<D, R> AssertIsCloseToWithinMargin<Timestamp, TemporalMargin> for Spec<'_, Timestamp, D, R>
where
    D: Represent<Timestamp> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(
        self,
        expected: Timestamp,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: Timestamp,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<Timestamp, D> for IsCloseTo<Timestamp, TemporalMargin>
where
    D: Represent<Timestamp> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &Timestamp) -> bool {
        if *subject < self.expected {
            subject.duration_until(self.expected) <= self.margin.to_duration()
        } else {
            subject.duration_since(self.expected) <= self.margin.to_duration()
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Timestamp,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let not = if inverted { "not " } else { "" };
        let (marked_actual, marked_expected) =
            mark_diff(actual, &self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        let represented_margin = Represented::from((&self.margin, representation));
        format!(
            r"expected {expression} to be {not}close to {represented_expected:?} within {represented_margin}
   but was: {marked_actual}
  expected: {marked_expected}"
        )
    }
}

impl Invertible for IsCloseTo<Timestamp, TemporalMargin> {}
