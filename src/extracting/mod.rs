use crate::assertions::{
    AssertBoolean, AssertChar, AssertDebugString, AssertDecimalNumber, AssertDisplayString,
    AssertEmptiness, AssertEquality, AssertErrorHasSource, AssertHasCharCount,
    AssertHasDebugString, AssertHasDisplayString, AssertHasError, AssertHasErrorMessage,
    AssertHasLength, AssertHasValue, AssertInRange, AssertInfinity, AssertIsSorted,
    AssertIteratorContains, AssertIteratorContainsInAnyOrder, AssertIteratorContainsInOrder,
    AssertNotANumber, AssertNumericIdentity, AssertOption, AssertOptionValue, AssertOrder,
    AssertResult, AssertResultValue, AssertSameAs, AssertSignum, AssertStringContainsAnyOf,
    AssertStringPattern,
};
use crate::expectations::{
    error_has_source, error_has_source_message, has_at_least_char_count, has_at_least_length,
    has_at_most_char_count, has_at_most_length, has_char_count, has_char_count_greater_than,
    has_char_count_in_range, has_char_count_less_than, has_debug_string, has_display_string,
    has_error, has_length, has_length_greater_than, has_length_in_range, has_length_less_than,
    has_precision_of, has_scale_of, has_value, is_a_number, is_after, is_alphabetic,
    is_alphanumeric, is_ascii, is_at_least, is_at_most, is_before, is_between, is_control_char,
    is_digit, is_empty, is_equal_to, is_err, is_false, is_finite, is_greater_than, is_in_range,
    is_infinite, is_integer, is_less_than, is_lower_case, is_negative, is_none, is_ok, is_one,
    is_positive, is_same_as, is_some, is_true, is_upper_case, is_whitespace, is_zero,
    iterator_contains, iterator_contains_all_in_order, iterator_contains_all_of,
    iterator_contains_any_of, iterator_contains_exactly, iterator_contains_exactly_in_any_order,
    iterator_contains_only, iterator_contains_only_once, iterator_contains_sequence,
    iterator_ends_with, iterator_starts_with, not, string_contains, string_contains_any_of,
    string_ends_with, string_starts_with,
};
use crate::properties::{
    AdditiveIdentityProperty, CharCountProperty, DecimalProperties, DefinedOrderProperty,
    InfinityProperty, IsEmptyProperty, IsNanProperty, LengthProperty,
    MultiplicativeIdentityProperty, SignumProperty,
};
use crate::spec::{
    And, AssertFailure, DiffFormat, DoFail, Expectation, Expression, GetFailures, SoftPanic,
};
use crate::std::borrow::{Cow, ToOwned};
use crate::std::error::Error;
use crate::std::fmt::{Debug, Display};
use crate::std::format;
use crate::std::ops::RangeBounds;
use crate::std::string::{String, ToString};
use crate::std::vec::Vec;

pub struct DerivedSpec<'a, O, S> {
    original: O,
    subject: S,
    expression: Expression<'a>,
    diff_format: DiffFormat,
}

impl<O, S> GetFailures for DerivedSpec<'_, O, S>
where
    O: GetFailures,
{
    fn has_failures(&self) -> bool {
        self.original.has_failures()
    }

    fn failures(&self) -> Vec<AssertFailure> {
        self.original.failures()
    }

    fn display_failures(&self) -> Vec<String> {
        self.original.display_failures()
    }
}

impl<O, S> DerivedSpec<'_, O, S> {
    /// Returns the expression (or subject name) if one has been set.
    pub fn expression(&self) -> &Expression<'_> {
        &self.expression
    }

    /// Returns the diff format used with this assertion.
    pub const fn diff_format(&self) -> &DiffFormat {
        &self.diff_format
    }
}

impl<'a, O, S> DerivedSpec<'a, O, S> {
    #[must_use = "a derived spec does nothing unless an assertion method is called"]
    pub(crate) fn new(
        original: O,
        derived_subject: S,
        expression: Expression<'a>,
        diff_format: DiffFormat,
    ) -> Self {
        Self {
            original,
            subject: derived_subject,
            expression,
            diff_format,
        }
    }

    /// Sets the subject name or expression for this assertion.
    #[must_use = "a derived spec does nothing unless an assertion method is called"]
    pub fn named(mut self, subject_name: impl Into<Cow<'a, str>>) -> Self {
        self.expression = Expression(subject_name.into());
        self
    }

    /// Sets the diff format used to highlight differences between the actual
    /// value and the expected value.
    ///
    /// Note: This method must be called before an assertion method is called to
    /// affect the failure message of the assertion as failure messages are
    /// formatted immediately when an assertion is executed.
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub const fn with_diff_format(mut self, diff_format: DiffFormat) -> Self {
        self.diff_format = diff_format;
        self
    }

    /// Sets the diff format used to highlight differences between the actual
    /// value and the expected value according to the configured mode.
    ///
    /// The mode is configured via environment variables as described in the
    /// module [colored].
    #[cfg(feature = "colored")]
    #[cfg_attr(docsrs, doc(cfg(feature = "colored")))]
    #[must_use = "a spec does nothing unless an assertion method is called"]
    pub fn with_configured_diff_format(self) -> Self {
        use crate::colored::configured_diff_format;
        #[cfg(not(feature = "std"))]
        {
            self.with_diff_format(configured_diff_format())
        }
        #[cfg(feature = "std")]
        {
            use crate::std::sync::OnceLock;
            static DIFF_FORMAT: OnceLock<DiffFormat> = OnceLock::new();
            let diff_format = DIFF_FORMAT.get_or_init(configured_diff_format);
            self.with_diff_format(diff_format.clone())
        }
    }
}

impl<O, S> DoFail for DerivedSpec<'_, O, S>
where
    O: DoFail,
{
    fn do_fail_with(&mut self, failures: impl IntoIterator<Item = AssertFailure>) {
        self.original.do_fail_with(failures);
    }

    fn do_fail_with_message(&mut self, message: impl Into<String>) {
        self.original.do_fail_with_message(message);
    }
}

impl<O, S> SoftPanic for DerivedSpec<'_, O, S>
where
    O: SoftPanic,
{
    fn soft_panic(&self) {
        self.original.soft_panic();
    }
}

impl<O, S> And for DerivedSpec<'_, O, S> {
    type Target = O;

    fn and(self) -> Self::Target {
        self.original
    }
}

impl<'a, O, S> DerivedSpec<'a, O, S> {
    #[must_use = "a derived spec does nothing unless an assertion method is called"]
    pub fn extracting_ref<F, B, U>(self, extract: F) -> DerivedSpec<'a, Self, U>
    where
        F: FnOnce(&S) -> &B,
        B: ToOwned<Owned = U> + ?Sized,
    {
        let extracted = extract(&self.subject).to_owned();
        let expression = Expression::default();
        let diff_format = self.diff_format.clone();
        DerivedSpec {
            original: self,
            subject: extracted,
            expression,
            diff_format,
        }
    }

    #[must_use = "a derived spec does nothing unless an assertion method is called"]
    pub fn extracting<F, U>(self, extract: F) -> DerivedSpec<'a, O, U>
    where
        F: FnOnce(S) -> U,
    {
        let extracted = extract(self.subject);
        let diff_format = self.diff_format.clone();
        DerivedSpec {
            original: self.original,
            subject: extracted,
            expression: Expression::default(),
            diff_format,
        }
    }

    #[must_use = "a derived spec does nothing unless an assertion method is called"]
    pub fn mapping<F, U>(self, map: F) -> DerivedSpec<'a, O, U>
    where
        F: FnOnce(S) -> U,
    {
        let mapped = map(self.subject);
        DerivedSpec {
            original: self.original,
            subject: mapped,
            expression: self.expression,
            diff_format: self.diff_format,
        }
    }
}

impl<O, S> DerivedSpec<'_, O, S>
where
    O: DoFail,
{
    #[allow(clippy::needless_pass_by_value, clippy::return_self_not_must_use)]
    #[track_caller]
    pub fn expecting(mut self, mut expectation: impl Expectation<S>) -> Self {
        if !expectation.test(&self.subject) {
            let message =
                expectation.message(&self.expression, &self.subject, false, &self.diff_format);
            self.do_fail_with_message(message);
        }
        self
    }
}

impl<O, S, E> AssertEquality<E> for DerivedSpec<'_, O, S>
where
    S: PartialEq<E> + Debug,
    E: Debug,
    O: DoFail,
{
    fn is_equal_to(self, expected: E) -> Self {
        self.expecting(is_equal_to(expected))
    }

    fn is_not_equal_to(self, expected: E) -> Self {
        self.expecting(not(is_equal_to(expected)))
    }
}

impl<O, S> AssertSameAs<S> for DerivedSpec<'_, O, S>
where
    S: PartialEq + Debug,
    O: DoFail,
{
    fn is_same_as(self, expected: S) -> Self {
        self.expecting(is_same_as(expected))
    }

    fn is_not_same_as(self, expected: S) -> Self {
        self.expecting(not(is_same_as(expected)))
    }
}

#[cfg(feature = "float-cmp")]
mod float_cmp {
    use super::DerivedSpec;
    use crate::assertions::{AssertIsCloseToWithDefaultMargin, AssertIsCloseToWithinMargin};
    use crate::expectations::{is_close_to, not};
    use crate::spec::DoFail;
    use float_cmp::{F32Margin, F64Margin};

    impl<O> AssertIsCloseToWithinMargin<f32, F32Margin> for DerivedSpec<'_, O, f32>
    where
        O: DoFail,
    {
        fn is_close_to_with_margin(self, expected: f32, margin: impl Into<F32Margin>) -> Self {
            self.expecting(is_close_to(expected).within_margin(margin))
        }

        fn is_not_close_to_with_margin(self, expected: f32, margin: impl Into<F32Margin>) -> Self {
            self.expecting(not(is_close_to(expected).within_margin(margin)))
        }
    }

    impl<O> AssertIsCloseToWithDefaultMargin<f32> for DerivedSpec<'_, O, f32>
    where
        O: DoFail,
    {
        fn is_close_to(self, expected: f32) -> Self {
            self.expecting(is_close_to(expected).within_margin((4. * f32::EPSILON, 4)))
        }

        fn is_not_close_to(self, expected: f32) -> Self {
            self.expecting(not(
                is_close_to(expected).within_margin((4. * f32::EPSILON, 4))
            ))
        }
    }

    impl<O> AssertIsCloseToWithinMargin<f64, F64Margin> for DerivedSpec<'_, O, f64>
    where
        O: DoFail,
    {
        fn is_close_to_with_margin(self, expected: f64, margin: impl Into<F64Margin>) -> Self {
            self.expecting(is_close_to(expected).within_margin(margin))
        }

        fn is_not_close_to_with_margin(self, expected: f64, margin: impl Into<F64Margin>) -> Self {
            self.expecting(not(is_close_to(expected).within_margin(margin)))
        }
    }

    impl<O> AssertIsCloseToWithDefaultMargin<f64> for DerivedSpec<'_, O, f64>
    where
        O: DoFail,
    {
        fn is_close_to(self, expected: f64) -> Self {
            self.expecting(is_close_to(expected).within_margin((4. * f64::EPSILON, 4)))
        }

        fn is_not_close_to(self, expected: f64) -> Self {
            self.expecting(not(
                is_close_to(expected).within_margin((4. * f64::EPSILON, 4))
            ))
        }
    }
}

impl<O, S, E> AssertOrder<E> for DerivedSpec<'_, O, S>
where
    S: PartialOrd<E> + Debug,
    E: Debug,
    O: DoFail,
{
    fn is_less_than(self, expected: E) -> Self {
        self.expecting(is_less_than(expected))
    }

    fn is_greater_than(self, expected: E) -> Self {
        self.expecting(is_greater_than(expected))
    }

    fn is_at_most(self, expected: E) -> Self {
        self.expecting(is_at_most(expected))
    }

    fn is_at_least(self, expected: E) -> Self {
        self.expecting(is_at_least(expected))
    }

    fn is_before(self, expected: E) -> Self {
        self.expecting(is_before(expected))
    }

    fn is_after(self, expected: E) -> Self {
        self.expecting(is_after(expected))
    }

    fn is_between(self, min: E, max: E) -> Self {
        self.expecting(is_between(min, max))
    }
}

impl<O, S, E> AssertInRange<E> for DerivedSpec<'_, O, S>
where
    S: PartialOrd<E> + Debug,
    E: PartialOrd<S> + Debug,
    O: DoFail,
{
    fn is_in_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<E> + Debug,
    {
        self.expecting(is_in_range(range))
    }

    fn is_not_in_range<R>(self, range: R) -> Self
    where
        R: RangeBounds<E> + Debug,
    {
        self.expecting(not(is_in_range(range)))
    }
}

impl<O, S> AssertNumericIdentity for DerivedSpec<'_, O, S>
where
    S: AdditiveIdentityProperty + MultiplicativeIdentityProperty + PartialEq + Debug,
    O: DoFail,
{
    fn is_zero(self) -> Self {
        self.expecting(is_zero())
    }

    fn is_one(self) -> Self {
        self.expecting(is_one())
    }
}

impl<O, S> AssertSignum for DerivedSpec<'_, O, S>
where
    S: SignumProperty + Debug,
    O: DoFail,
{
    fn is_negative(self) -> Self {
        self.expecting(is_negative())
    }

    fn is_not_negative(self) -> Self {
        self.expecting(not(is_negative()))
    }

    fn is_positive(self) -> Self {
        self.expecting(is_positive())
    }

    fn is_not_positive(self) -> Self {
        self.expecting(not(is_positive()))
    }
}

impl<O, S> AssertInfinity for DerivedSpec<'_, O, S>
where
    S: InfinityProperty + Debug,
    O: DoFail,
{
    fn is_infinite(self) -> Self {
        self.expecting(is_infinite())
    }

    fn is_finite(self) -> Self {
        self.expecting(is_finite())
    }
}

impl<O, S> AssertNotANumber for DerivedSpec<'_, O, S>
where
    S: IsNanProperty + Debug,
    O: DoFail,
{
    fn is_not_a_number(self) -> Self {
        self.expecting(not(is_a_number()))
    }

    fn is_a_number(self) -> Self {
        self.expecting(is_a_number())
    }
}

impl<O, S> AssertDecimalNumber for DerivedSpec<'_, O, S>
where
    S: DecimalProperties + Debug,
    O: DoFail,
{
    fn has_scale_of(self, expected_scale: i64) -> Self {
        self.expecting(has_scale_of(expected_scale))
    }

    fn has_precision_of(self, expected_precision: u64) -> Self {
        self.expecting(has_precision_of(expected_precision))
    }

    fn is_integer(self) -> Self {
        self.expecting(is_integer())
    }
}

impl<O> AssertBoolean for DerivedSpec<'_, O, bool>
where
    O: DoFail,
{
    fn is_true(self) -> Self {
        self.expecting(is_true())
    }

    fn is_false(self) -> Self {
        self.expecting(is_false())
    }
}

impl<O> AssertChar for DerivedSpec<'_, O, char>
where
    O: DoFail,
{
    fn is_lowercase(self) -> Self {
        self.expecting(is_lower_case())
    }

    fn is_uppercase(self) -> Self {
        self.expecting(is_upper_case())
    }

    fn is_ascii(self) -> Self {
        self.expecting(is_ascii())
    }

    fn is_alphabetic(self) -> Self {
        self.expecting(is_alphabetic())
    }

    fn is_alphanumeric(self) -> Self {
        self.expecting(is_alphanumeric())
    }

    fn is_control_char(self) -> Self {
        self.expecting(is_control_char())
    }

    fn is_digit(self, radix: u32) -> Self {
        self.expecting(is_digit(radix))
    }

    fn is_whitespace(self) -> Self {
        self.expecting(is_whitespace())
    }
}

impl<O> AssertChar for DerivedSpec<'_, O, &char>
where
    O: DoFail,
{
    fn is_lowercase(self) -> Self {
        self.expecting(is_lower_case())
    }

    fn is_uppercase(self) -> Self {
        self.expecting(is_upper_case())
    }

    fn is_ascii(self) -> Self {
        self.expecting(is_ascii())
    }

    fn is_alphabetic(self) -> Self {
        self.expecting(is_alphabetic())
    }

    fn is_alphanumeric(self) -> Self {
        self.expecting(is_alphanumeric())
    }

    fn is_control_char(self) -> Self {
        self.expecting(is_control_char())
    }

    fn is_digit(self, radix: u32) -> Self {
        self.expecting(is_digit(radix))
    }

    fn is_whitespace(self) -> Self {
        self.expecting(is_whitespace())
    }
}

impl<O, S> AssertEmptiness for DerivedSpec<'_, O, S>
where
    S: IsEmptyProperty + Debug,
    O: DoFail,
{
    fn is_empty(self) -> Self {
        self.expecting(is_empty())
    }

    fn is_not_empty(self) -> Self {
        self.expecting(not(is_empty()))
    }
}

impl<O, S> AssertHasLength<usize> for DerivedSpec<'_, O, S>
where
    S: LengthProperty + Debug,
    O: DoFail,
{
    fn has_length(self, expected_length: usize) -> Self {
        self.expecting(has_length(expected_length))
    }

    fn has_length_in_range<R>(self, expected_range: R) -> Self
    where
        R: RangeBounds<usize> + Debug,
    {
        self.expecting(has_length_in_range(expected_range))
    }

    fn has_length_less_than(self, expected_length: usize) -> Self {
        self.expecting(has_length_less_than(expected_length))
    }

    fn has_length_greater_than(self, expected_length: usize) -> Self {
        self.expecting(has_length_greater_than(expected_length))
    }

    fn has_at_most_length(self, expected_length: usize) -> Self {
        self.expecting(has_at_most_length(expected_length))
    }

    fn has_at_least_length(self, expected_length: usize) -> Self {
        self.expecting(has_at_least_length(expected_length))
    }
}

impl<O, S> AssertHasCharCount<usize> for DerivedSpec<'_, O, S>
where
    S: CharCountProperty + Debug,
    O: DoFail,
{
    fn has_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(has_char_count(expected_char_count))
    }

    fn has_char_count_in_range<U>(self, expected_range: U) -> Self
    where
        U: RangeBounds<usize> + Debug,
    {
        self.expecting(has_char_count_in_range(expected_range))
    }

    fn has_char_count_less_than(self, expected_char_count: usize) -> Self {
        self.expecting(has_char_count_less_than(expected_char_count))
    }

    fn has_char_count_greater_than(self, expected_char_count: usize) -> Self {
        self.expecting(has_char_count_greater_than(expected_char_count))
    }

    fn has_at_most_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(has_at_most_char_count(expected_char_count))
    }

    fn has_at_least_char_count(self, expected_char_count: usize) -> Self {
        self.expecting(has_at_least_char_count(expected_char_count))
    }
}

impl<O, S> AssertOption for DerivedSpec<'_, O, Option<S>>
where
    S: Debug,
    O: DoFail,
{
    fn is_some(self) -> Self {
        self.expecting(is_some())
    }

    fn is_none(self) -> Self {
        self.expecting(is_none())
    }
}

impl<'a, O, T> AssertOptionValue for DerivedSpec<'a, O, Option<T>>
where
    O: DoFail,
{
    type Some = DerivedSpec<'a, O, T>;

    fn some(self) -> Self::Some {
        self.mapping(|subject| match subject {
            None => {
                panic!("expected the subject to be `Some(_)`, but was `None`")
            },
            Some(value) => value,
        })
    }
}

impl<'a, O, T> AssertOptionValue for DerivedSpec<'a, O, &'a Option<T>>
where
    T: 'a,
    O: DoFail,
{
    type Some = DerivedSpec<'a, O, &'a T>;

    fn some(self) -> Self::Some {
        self.mapping(|subject| match subject {
            None => {
                panic!("expected the subject to be `Some(_)`, but was `None`")
            },
            Some(value) => value,
        })
    }
}

impl<O, T, E> AssertHasValue<E> for DerivedSpec<'_, O, Option<T>>
where
    T: PartialEq<E> + Debug,
    E: Debug,
    O: DoFail,
{
    fn has_value(self, expected: E) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<O, T, E> AssertHasValue<E> for DerivedSpec<'_, O, &Option<T>>
where
    T: PartialEq<E> + Debug,
    E: Debug,
    O: DoFail,
{
    fn has_value(self, expected: E) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<O, T, E> AssertResult for DerivedSpec<'_, O, Result<T, E>>
where
    T: Debug,
    E: Debug,
    O: DoFail,
{
    fn is_ok(self) -> Self {
        self.expecting(is_ok())
    }

    fn is_err(self) -> Self {
        self.expecting(is_err())
    }
}

impl<O, T, E> AssertResult for DerivedSpec<'_, O, &Result<T, E>>
where
    T: Debug,
    E: Debug,
    O: DoFail,
{
    fn is_ok(self) -> Self {
        self.expecting(is_ok())
    }

    fn is_err(self) -> Self {
        self.expecting(is_err())
    }
}

impl<'a, O, T, E> AssertResultValue for DerivedSpec<'a, O, Result<T, E>>
where
    T: Debug,
    E: Debug,
    O: DoFail,
{
    type Ok = DerivedSpec<'a, O, T>;
    type Err = DerivedSpec<'a, O, E>;

    fn ok(self) -> Self::Ok {
        self.mapping(|subject| match subject {
            Ok(value) => value,
            Err(error) => {
                panic!("expected the subject to be `Ok(_)`, but was `Err({error:?})`")
            },
        })
    }

    fn err(self) -> Self::Err {
        self.mapping(|subject| match subject {
            Ok(value) => {
                panic!("expected the subject to be `Err(_)`, but was `Ok({value:?})`")
            },
            Err(error) => error,
        })
    }
}

impl<'a, O, T, E> AssertResultValue for DerivedSpec<'a, O, &'a Result<T, E>>
where
    T: Debug,
    E: Debug,
    O: DoFail,
{
    type Ok = DerivedSpec<'a, O, &'a T>;
    type Err = DerivedSpec<'a, O, &'a E>;

    fn ok(self) -> Self::Ok {
        self.mapping(|subject| match subject {
            Ok(value) => value,
            Err(error) => {
                panic!("expected the subject to be `Ok(_)`, but was `Err({error:?})`")
            },
        })
    }

    fn err(self) -> Self::Err {
        self.mapping(|subject| match subject {
            Ok(value) => {
                panic!("expected the subject to be `Err(_)`, but was `Ok({value:?})`")
            },
            Err(error) => error,
        })
    }
}

impl<O, T, E, X> AssertHasValue<X> for DerivedSpec<'_, O, Result<T, E>>
where
    T: PartialEq<X> + Debug,
    E: Debug,
    X: Debug,
    O: DoFail,
{
    fn has_value(self, expected: X) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<O, T, E, X> AssertHasValue<X> for DerivedSpec<'_, O, &Result<T, E>>
where
    T: PartialEq<X> + Debug,
    E: Debug,
    X: Debug,
    O: DoFail,
{
    fn has_value(self, expected: X) -> Self {
        self.expecting(has_value(expected))
    }
}

impl<O, T, E, X> AssertHasError<X> for DerivedSpec<'_, O, Result<T, E>>
where
    T: Debug,
    E: PartialEq<X> + Debug,
    X: Debug,
    O: DoFail,
{
    fn has_error(self, expected: X) -> Self {
        self.expecting(has_error(expected))
    }
}

impl<O, T, E, X> AssertHasError<X> for DerivedSpec<'_, O, &Result<T, E>>
where
    T: Debug,
    E: PartialEq<X> + Debug,
    X: Debug,
    O: DoFail,
{
    fn has_error(self, expected: X) -> Self {
        self.expecting(has_error(expected))
    }
}

impl<'a, O, T, E, X> AssertHasErrorMessage<X> for DerivedSpec<'a, O, Result<T, E>>
where
    T: Debug,
    E: Display,
    X: Debug,
    String: PartialEq<X>,
    O: DoFail,
{
    type ErrorMessage = DerivedSpec<'a, O, String>;

    fn has_error_message(self, expected: X) -> Self::ErrorMessage {
        self.mapping(|result| match result {
            Ok(value) => panic!("expected the subject to be `Err(_)` with message {expected:?}, but was `Ok({value:?})`"),
            Err(error) => error.to_string(),
        }).expecting(is_equal_to(expected))
    }
}

impl<'a, O, T, E, X> AssertHasErrorMessage<X> for DerivedSpec<'a, O, &Result<T, E>>
where
    T: Debug,
    E: Display,
    X: Debug,
    String: PartialEq<X>,
    O: DoFail,
{
    type ErrorMessage = DerivedSpec<'a, O, String>;

    fn has_error_message(self, expected: X) -> Self::ErrorMessage {
        self.mapping(|result| match result {
            Ok(value) => panic!("expected the subject to be `Err(_)` with message {expected:?}, but was `Ok({value:?})`"),
            Err(error) => error.to_string(),
        }).expecting(is_equal_to(expected))
    }
}

impl<'a, O, S> AssertErrorHasSource for DerivedSpec<'a, O, S>
where
    S: Error,
    O: DoFail,
{
    type SourceMessage = DerivedSpec<'a, O, Option<String>>;

    fn has_no_source(self) -> Self {
        self.expecting(not(error_has_source()))
    }

    fn has_source(self) -> Self {
        self.expecting(error_has_source())
    }

    fn has_source_message(self, expected_source_message: impl Into<String>) -> Self::SourceMessage {
        let expected_source_message = expected_source_message.into();
        self.expecting(error_has_source_message(expected_source_message))
            .mapping(|err| err.source().map(ToString::to_string))
    }
}

impl<O, S, E> AssertHasDebugString<E> for DerivedSpec<'_, O, S>
where
    S: Debug,
    E: AsRef<str>,
    O: DoFail,
{
    fn has_debug_string(self, expected: E) -> Self {
        self.expecting(has_debug_string(expected))
    }

    fn does_not_have_debug_string(self, expected: E) -> Self {
        self.expecting(not(has_debug_string(expected)))
    }
}

impl<'a, O, S> AssertDebugString for DerivedSpec<'a, O, S>
where
    S: Debug,
    O: DoFail,
{
    type DebugString = DerivedSpec<'a, O, String>;

    fn debug_string(self) -> Self::DebugString {
        let expression_debug_string = format!("{}'s debug string", self.expression);
        self.mapping(|subject| format!("{subject:?}"))
            .named(expression_debug_string)
    }
}

impl<O, S, E> AssertHasDisplayString<E> for DerivedSpec<'_, O, S>
where
    S: Display,
    E: AsRef<str>,
    O: DoFail,
{
    fn has_display_string(self, expected: E) -> Self {
        self.expecting(has_display_string(expected))
    }

    fn does_not_have_display_string(self, expected: E) -> Self {
        self.expecting(not(has_display_string(expected)))
    }
}

impl<'a, O, S> AssertDisplayString for DerivedSpec<'a, O, S>
where
    S: Display,
    O: DoFail,
{
    type DisplayString = DerivedSpec<'a, O, String>;

    fn display_string(self) -> Self::DisplayString {
        let expression_display_string = format!("{}'s display string", self.expression);
        self.mapping(|subject| subject.to_string())
            .named(expression_display_string)
    }
}

impl<'a, O, S> AssertStringPattern<&'a str> for DerivedSpec<'a, O, S>
where
    S: 'a + AsRef<str> + Debug,
    O: DoFail,
{
    fn contains(self, pattern: &'a str) -> Self {
        self.expecting(string_contains(pattern))
    }

    fn does_not_contain(self, pattern: &'a str) -> Self {
        self.expecting(not(string_contains(pattern)))
    }

    fn starts_with(self, pattern: &'a str) -> Self {
        self.expecting(string_starts_with(pattern))
    }

    fn does_not_start_with(self, pattern: &'a str) -> Self {
        self.expecting(not(string_starts_with(pattern)))
    }

    fn ends_with(self, pattern: &'a str) -> Self {
        self.expecting(string_ends_with(pattern))
    }

    fn does_not_end_with(self, pattern: &'a str) -> Self {
        self.expecting(not(string_ends_with(pattern)))
    }
}

impl<'a, O, S> AssertStringPattern<String> for DerivedSpec<'a, O, S>
where
    S: 'a + AsRef<str> + Debug,
    O: DoFail,
{
    fn contains(self, pattern: String) -> Self {
        self.expecting(string_contains(pattern))
    }

    fn does_not_contain(self, pattern: String) -> Self {
        self.expecting(not(string_contains(pattern)))
    }

    fn starts_with(self, pattern: String) -> Self {
        self.expecting(string_starts_with(pattern))
    }

    fn does_not_start_with(self, pattern: String) -> Self {
        self.expecting(not(string_starts_with(pattern)))
    }

    fn ends_with(self, pattern: String) -> Self {
        self.expecting(string_ends_with(pattern))
    }

    fn does_not_end_with(self, pattern: String) -> Self {
        self.expecting(not(string_ends_with(pattern)))
    }
}

impl<'a, O, S> AssertStringPattern<char> for DerivedSpec<'a, O, S>
where
    S: 'a + AsRef<str> + Debug,
    O: DoFail,
{
    fn contains(self, pattern: char) -> Self {
        self.expecting(string_contains(pattern))
    }

    fn does_not_contain(self, pattern: char) -> Self {
        self.expecting(not(string_contains(pattern)))
    }

    fn starts_with(self, pattern: char) -> Self {
        self.expecting(string_starts_with(pattern))
    }

    fn does_not_start_with(self, pattern: char) -> Self {
        self.expecting(not(string_starts_with(pattern)))
    }

    fn ends_with(self, pattern: char) -> Self {
        self.expecting(string_ends_with(pattern))
    }

    fn does_not_end_with(self, pattern: char) -> Self {
        self.expecting(not(string_ends_with(pattern)))
    }
}

impl<'a, O, S> AssertStringContainsAnyOf<&'a [char]> for DerivedSpec<'a, O, S>
where
    S: 'a + AsRef<str> + Debug,
    O: DoFail,
{
    fn contains_any_of(self, expected: &'a [char]) -> Self {
        self.expecting(string_contains_any_of(expected))
    }

    fn does_not_contain_any_of(self, expected: &'a [char]) -> Self {
        self.expecting(not(string_contains_any_of(expected)))
    }
}

impl<'a, O, S, const N: usize> AssertStringContainsAnyOf<[char; N]> for DerivedSpec<'a, O, S>
where
    S: 'a + AsRef<str> + Debug,
    O: DoFail,
{
    fn contains_any_of(self, expected: [char; N]) -> Self {
        self.expecting(string_contains_any_of(expected))
    }

    fn does_not_contain_any_of(self, expected: [char; N]) -> Self {
        self.expecting(not(string_contains_any_of(expected)))
    }
}

impl<'a, O, S, const N: usize> AssertStringContainsAnyOf<&'a [char; N]> for DerivedSpec<'a, O, S>
where
    S: 'a + AsRef<str> + Debug,
    O: DoFail,
{
    fn contains_any_of(self, expected: &'a [char; N]) -> Self {
        self.expecting(string_contains_any_of(expected))
    }

    fn does_not_contain_any_of(self, expected: &'a [char; N]) -> Self {
        self.expecting(not(string_contains_any_of(expected)))
    }
}

#[cfg(feature = "regex")]
mod regex {
    use crate::assertions::AssertStringMatches;
    use crate::expectations::{not, string_matches};
    use crate::extracting::DerivedSpec;
    use crate::spec::DoFail;
    use crate::std::fmt::Debug;

    impl<O, S> AssertStringMatches for DerivedSpec<'_, O, S>
    where
        S: AsRef<str> + Debug,
        O: DoFail,
    {
        fn matches(self, regex_pattern: &str) -> Self {
            self.expecting(string_matches(regex_pattern))
        }

        fn does_not_match(self, regex_pattern: &str) -> Self {
            self.expecting(not(string_matches(regex_pattern)))
        }
    }
}

impl<'a, O, S, T, E> AssertIteratorContains<E> for DerivedSpec<'a, O, S>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<E> + Debug,
    E: Debug,
    O: DoFail,
{
    type Sequence = DerivedSpec<'a, O, Vec<T>>;

    fn contains(self, element: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains(element))
    }

    fn does_not_contain(self, element: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(not(iterator_contains(element)))
    }
}

impl<'a, O, S, T, E> AssertIteratorContainsInAnyOrder<E> for DerivedSpec<'a, O, S>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<<E as IntoIterator>::Item> + Debug,
    E: IntoIterator,
    <E as IntoIterator>::Item: Debug,
    O: DoFail,
{
    type Sequence = DerivedSpec<'a, O, Vec<T>>;

    fn contains_exactly_in_any_order(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_exactly_in_any_order(expected))
    }

    fn contains_any_of(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_any_of(expected))
    }

    fn does_not_contain_any_of(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(not(iterator_contains_any_of(expected)))
    }

    fn contains_all_of(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_all_of(expected))
    }

    fn contains_only(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_only(expected))
    }

    fn contains_only_once(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_only_once(expected))
    }
}

impl<'a, O, S, T, E> AssertIteratorContainsInOrder<E> for DerivedSpec<'a, O, S>
where
    S: IntoIterator<Item = T>,
    <S as IntoIterator>::IntoIter: DefinedOrderProperty,
    E: IntoIterator,
    <E as IntoIterator>::IntoIter: DefinedOrderProperty,
    <E as IntoIterator>::Item: Debug,
    T: PartialEq<<E as IntoIterator>::Item> + Debug,
    O: DoFail,
{
    type Sequence = DerivedSpec<'a, O, Vec<T>>;

    fn contains_exactly(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_exactly(expected))
    }

    fn contains_sequence(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_sequence(expected))
    }

    fn contains_all_in_order(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_all_in_order(expected))
    }

    fn starts_with(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_starts_with(expected))
    }

    fn ends_with(self, expected: E) -> Self::Sequence {
        self.mapping(Vec::from_iter)
            .expecting(iterator_ends_with(expected))
    }
}

#[cfg(test)]
mod tests;
