use crate::assertions::AssertIsCloseToWithinMargin;
use crate::colored::mark_diff;
use crate::expectations::{IsCloseTo, is_close_to, not};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::temporal_margin::TemporalMargin;
use time::{Date, PlainDateTime, SignedDuration, Time};

impl TemporalMargin {
    /// Converts this temporal margin to a [`SignedDuration`] value.
    pub fn to_signed_duration(self) -> SignedDuration {
        match self {
            Self::NanoSeconds(nanos) => SignedDuration::nanoseconds(i64::from(nanos)),
            Self::MicroSeconds(micros) => SignedDuration::microseconds(i64::from(micros)),
            Self::MilliSeconds(millis) => SignedDuration::milliseconds(i64::from(millis)),
            Self::Seconds(seconds) => SignedDuration::seconds(i64::from(seconds)),
            Self::Minutes(minutes) => SignedDuration::minutes(i64::from(minutes)),
            Self::Hours(hours) => SignedDuration::hours(i64::from(hours)),
            Self::Days(days) => SignedDuration::days(i64::from(days)),
            Self::Weeks(weeks) => SignedDuration::weeks(i64::from(weeks)),
        }
    }
}

impl<D, R> AssertIsCloseToWithinMargin<Time, TemporalMargin> for Spec<'_, Time, D, R>
where
    D: Represent<Time> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(self, expected: Time, margin: impl Into<TemporalMargin>) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: Time,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<Time, D> for IsCloseTo<Time, TemporalMargin>
where
    D: Represent<Time> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &Time) -> bool {
        if *subject < self.expected {
            subject.duration_until(self.expected) <= self.margin.to_signed_duration()
        } else {
            subject.duration_since(self.expected) <= self.margin.to_signed_duration()
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Time,
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

impl Invertible for IsCloseTo<Time, TemporalMargin> {}

impl<D, R> AssertIsCloseToWithinMargin<Date, TemporalMargin> for Spec<'_, Date, D, R>
where
    D: Represent<Date> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(self, expected: Date, margin: impl Into<TemporalMargin>) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: Date,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<Date, D> for IsCloseTo<Date, TemporalMargin>
where
    D: Represent<Date> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &Date) -> bool {
        if *subject < self.expected {
            self.expected - *subject <= self.margin.to_signed_duration()
        } else {
            *subject - self.expected <= self.margin.to_signed_duration()
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Date,
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

impl Invertible for IsCloseTo<Date, TemporalMargin> {}

impl<D, R> AssertIsCloseToWithinMargin<PlainDateTime, TemporalMargin>
    for Spec<'_, PlainDateTime, D, R>
where
    D: Represent<PlainDateTime> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(
        self,
        expected: PlainDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: PlainDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<PlainDateTime, D> for IsCloseTo<PlainDateTime, TemporalMargin>
where
    D: Represent<PlainDateTime> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &PlainDateTime) -> bool {
        if *subject < self.expected {
            self.expected - *subject <= self.margin.to_signed_duration()
        } else {
            *subject - self.expected <= self.margin.to_signed_duration()
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &PlainDateTime,
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

impl Invertible for IsCloseTo<PlainDateTime, TemporalMargin> {}

#[cfg(test)]
mod tests;
