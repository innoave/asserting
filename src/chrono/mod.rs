use crate::assertions::AssertIsCloseToWithinMargin;
use crate::colored::mark_diff;
use crate::derived_spec::DerivedSpec;
use crate::expectations::{IsCloseTo, is_close_to, not};
use crate::spec::{
    DiffFormat, DoFail, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::temporal_margin::{TemporalMargin, ToDuration};
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone};

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

impl ToDuration<Duration> for TemporalMargin {
    fn to_duration(self) -> Duration {
        self.to_time_delta()
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
        if *subject < self.expected {
            self.expected.signed_duration_since(*subject) <= self.margin.to_duration()
        } else {
            subject.signed_duration_since(self.expected) <= self.margin.to_duration()
        }
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
        if *subject < self.expected {
            self.expected.signed_duration_since(*subject) <= self.margin.to_duration()
        } else {
            subject.signed_duration_since(self.expected) <= self.margin.to_duration()
        }
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
        if *subject < self.expected {
            self.expected.signed_duration_since(*subject) <= self.margin.to_duration()
        } else {
            subject.signed_duration_since(self.expected) <= self.margin.to_duration()
        }
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

impl<TZ1, TZ2, D, R> AssertIsCloseToWithinMargin<DateTime<TZ2>, TemporalMargin>
    for Spec<'_, DateTime<TZ1>, D, R>
where
    TZ1: TimeZone,
    TZ2: TimeZone,
    D: Represent<DateTime<TZ1>> + Represent<DateTime<TZ2>> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(
        self,
        expected: DateTime<TZ2>,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: DateTime<TZ2>,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<TZ1, TZ2, D> Expectation<DateTime<TZ1>, D> for IsCloseTo<DateTime<TZ2>, TemporalMargin>
where
    TZ1: TimeZone,
    TZ2: TimeZone,
    D: Represent<DateTime<TZ1>> + Represent<DateTime<TZ2>> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &DateTime<TZ1>) -> bool {
        if *subject < self.expected {
            self.expected
                .naive_utc()
                .signed_duration_since(subject.naive_utc())
                <= self.margin.to_duration()
        } else {
            subject
                .naive_utc()
                .signed_duration_since(self.expected.naive_utc())
                <= self.margin.to_duration()
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &DateTime<TZ1>,
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

impl<TZ> Invertible for IsCloseTo<DateTime<TZ>, TemporalMargin> where TZ: TimeZone {}

impl<O, D> AssertIsCloseToWithinMargin<NaiveTime, TemporalMargin>
    for DerivedSpec<'_, O, NaiveTime, D>
where
    D: Represent<NaiveTime> + Represent<TemporalMargin>,
    O: DoFail,
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

impl<O, D> AssertIsCloseToWithinMargin<NaiveDate, TemporalMargin>
    for DerivedSpec<'_, O, NaiveDate, D>
where
    D: Represent<NaiveDate> + Represent<TemporalMargin>,
    O: DoFail,
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

impl<O, D> AssertIsCloseToWithinMargin<NaiveDateTime, TemporalMargin>
    for DerivedSpec<'_, O, NaiveDateTime, D>
where
    D: Represent<NaiveDateTime> + Represent<TemporalMargin>,
    O: DoFail,
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

impl<O, TZ1, TZ2, D> AssertIsCloseToWithinMargin<DateTime<TZ2>, TemporalMargin>
    for DerivedSpec<'_, O, DateTime<TZ1>, D>
where
    TZ1: TimeZone,
    TZ2: TimeZone,
    D: Represent<DateTime<TZ1>> + Represent<DateTime<TZ2>> + Represent<TemporalMargin>,
    O: DoFail,
{
    fn is_close_to_with_margin(
        self,
        expected: DateTime<TZ2>,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: DateTime<TZ2>,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

#[cfg(test)]
mod tests;
