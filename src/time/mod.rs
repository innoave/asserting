use crate::assertions::AssertIsCloseToWithinMargin;
use crate::colored::mark_diff;
use crate::derived_spec::DerivedSpec;
use crate::expectations::{IsCloseTo, is_close_to, not};
use crate::representation::{Represent, Represented};
use crate::spec::{
    DiffFormat, DoFail, Expectation, Expecting, Expression, FailingStrategy, Invertible, Spec,
};
use crate::temporal::{TemporalMargin, ToDuration};
use time::{Date, OffsetDateTime, PlainDateTime, SignedDuration, Time, Timestamp};

impl ToDuration<SignedDuration> for TemporalMargin {
    fn to_duration(self) -> SignedDuration {
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
            self.expected - *subject
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
        } else {
            *subject - self.expected
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
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
            subject.duration_until(self.expected)
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
        } else {
            subject.duration_since(self.expected)
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
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
            self.expected - *subject
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
        } else {
            *subject - self.expected
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
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
            self.expected - *subject
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
        } else {
            *subject - self.expected
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
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

impl<D, R> AssertIsCloseToWithinMargin<OffsetDateTime, TemporalMargin>
    for Spec<'_, OffsetDateTime, D, R>
where
    D: Represent<OffsetDateTime> + Represent<TemporalMargin>,
    R: FailingStrategy,
{
    fn is_close_to_with_margin(
        self,
        expected: OffsetDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: OffsetDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

impl<D> Expectation<OffsetDateTime, D> for IsCloseTo<OffsetDateTime, TemporalMargin>
where
    D: Represent<OffsetDateTime> + Represent<TemporalMargin>,
{
    fn test(&mut self, subject: &OffsetDateTime) -> bool {
        if *subject < self.expected {
            self.expected - *subject
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
        } else {
            *subject - self.expected
                <= <TemporalMargin as ToDuration<SignedDuration>>::to_duration(self.margin)
        }
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &OffsetDateTime,
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

impl Invertible for IsCloseTo<OffsetDateTime, TemporalMargin> {}

impl<O, D> AssertIsCloseToWithinMargin<Time, TemporalMargin> for DerivedSpec<'_, O, Time, D>
where
    D: Represent<Time> + Represent<TemporalMargin>,
    O: DoFail,
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

impl<O, D> AssertIsCloseToWithinMargin<Date, TemporalMargin> for DerivedSpec<'_, O, Date, D>
where
    D: Represent<Date> + Represent<TemporalMargin>,
    O: DoFail,
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

impl<O, D> AssertIsCloseToWithinMargin<PlainDateTime, TemporalMargin>
    for DerivedSpec<'_, O, PlainDateTime, D>
where
    D: Represent<PlainDateTime> + Represent<TemporalMargin>,
    O: DoFail,
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

impl<O, D> AssertIsCloseToWithinMargin<OffsetDateTime, TemporalMargin>
    for DerivedSpec<'_, O, OffsetDateTime, D>
where
    D: Represent<OffsetDateTime> + Represent<TemporalMargin>,
    O: DoFail,
{
    fn is_close_to_with_margin(
        self,
        expected: OffsetDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(is_close_to(expected).within_margin(margin))
    }

    fn is_not_close_to_with_margin(
        self,
        expected: OffsetDateTime,
        margin: impl Into<TemporalMargin>,
    ) -> Self {
        self.expecting(not(is_close_to(expected).within_margin(margin)))
    }
}

#[cfg(test)]
mod tests;
