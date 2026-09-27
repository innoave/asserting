use crate::assertions::AssertIsCloseToWithinMargin;
use crate::colored::mark_diff;
use crate::expectations::{IsCloseTo, is_close_to, not};
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::temporal_margin::TemporalMargin;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

impl TemporalMargin {
    /// Converts this temporal margin to a [`TimeDelta`] value.
    pub fn to_time_delta(self) -> TimeDelta {
        match self {
            Self::NanoSeconds(nanos) => TimeDelta::nanoseconds(i64::from(nanos)),
            Self::MicroSeconds(micros) => TimeDelta::microseconds(i64::from(micros)),
            Self::MilliSeconds(millis) => TimeDelta::milliseconds(i64::from(millis)),
            Self::Seconds(seconds) => TimeDelta::seconds(i64::from(seconds)),
            Self::Minutes(minutes) => TimeDelta::minutes(i64::from(minutes)),
            Self::Hours(hours) => TimeDelta::hours(i64::from(hours)),
            Self::Days(days) => TimeDelta::days(i64::from(days)),
            Self::Weeks(weeks) => TimeDelta::weeks(i64::from(weeks)),
        }
    }
}

impl<D, R> AssertIsCloseToWithinMargin<NaiveTime, TemporalMargin> for Spec<'_, NaiveTime, D, R>
where
    D: Represent<NaiveTime> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(
        self,
        expected: NaiveTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: NaiveTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<NaiveTime, D> for IsCloseTo<NaiveTime, TemporalMargin>
where
    D: Represent<NaiveTime> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &NaiveTime) -> bool {
        subject.signed_duration_since(self.expected).abs() <= self.margin.to_time_delta()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &NaiveTime,
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

impl Invertible for IsCloseTo<NaiveTime, TemporalMargin> {}

impl<D, R> AssertIsCloseToWithinMargin<NaiveDate, TemporalMargin> for Spec<'_, NaiveDate, D, R>
where
    D: Represent<NaiveDate> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(
        self,
        expected: NaiveDate,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: NaiveDate,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<NaiveDate, D> for IsCloseTo<NaiveDate, TemporalMargin>
where
    D: Represent<NaiveDate> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &NaiveDate) -> bool {
        subject.signed_duration_since(self.expected).abs() <= self.margin.to_time_delta()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &NaiveDate,
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

impl Invertible for IsCloseTo<NaiveDate, TemporalMargin> {}

impl<D, R> AssertIsCloseToWithinMargin<NaiveDateTime, TemporalMargin>
    for Spec<'_, NaiveDateTime, D, R>
where
    D: Represent<NaiveDateTime> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(
        self,
        expected: NaiveDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: NaiveDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<NaiveDateTime, D> for IsCloseTo<NaiveDateTime, TemporalMargin>
where
    D: Represent<NaiveDateTime> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &NaiveDateTime) -> bool {
        subject.signed_duration_since(self.expected).abs() <= self.margin.to_time_delta()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &NaiveDateTime,
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

impl Invertible for IsCloseTo<NaiveDateTime, TemporalMargin> {}

#[cfg(test)]
mod tests;
