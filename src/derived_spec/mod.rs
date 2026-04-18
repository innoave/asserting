//! Defines the [`DerivedSpec`], which keeps track of the original subject while doing assertions
//! on a derived subject.

use crate::assertions::{
    AssertBoolean, AssertChar, AssertDebugString, AssertDecimalNumber, AssertDisplayString,
    AssertEmptiness, AssertEquality, AssertErrorHasSource, AssertHasCharCount,
    AssertHasDebugString, AssertHasDisplayString, AssertHasError, AssertHasErrorMessage,
    AssertHasLength, AssertHasValue, AssertInRange, AssertInfinity, AssertIteratorContains,
    AssertIteratorContainsInAnyOrder, AssertIteratorContainsInOrder, AssertMapContainsKey,
    AssertMapContainsValue, AssertNotANumber, AssertNumericIdentity, AssertOption,
    AssertOptionValue, AssertOrder, AssertOrderedElements, AssertOrderedElementsRef, AssertResult,
    AssertResultValue, AssertSameAs, AssertSignum, AssertStringContainsAnyOf, AssertStringPattern,
};
use crate::expectations::{
    error_has_source, error_has_source_message, has_at_least_char_count, has_at_least_length,
    has_at_least_number_of_elements, has_at_most_char_count, has_at_most_length, has_char_count,
    has_char_count_greater_than, has_char_count_in_range, has_char_count_less_than,
    has_debug_string, has_display_string, has_error, has_length, has_length_greater_than,
    has_length_in_range, has_length_less_than, has_precision_of, has_scale_of, has_value,
    is_a_number, is_after, is_alphabetic, is_alphanumeric, is_ascii, is_at_least, is_at_most,
    is_before, is_between, is_control_char, is_digit, is_empty, is_equal_to, is_err, is_false,
    is_finite, is_greater_than, is_in_range, is_infinite, is_integer, is_less_than, is_lower_case,
    is_negative, is_none, is_ok, is_one, is_positive, is_same_as, is_some, is_true, is_upper_case,
    is_whitespace, is_zero, iterator_contains, iterator_contains_all_in_order,
    iterator_contains_all_of, iterator_contains_any_of, iterator_contains_exactly,
    iterator_contains_exactly_in_any_order, iterator_contains_only, iterator_contains_only_once,
    iterator_contains_sequence, iterator_ends_with, iterator_starts_with,
    map_contains_exactly_keys, map_contains_key, map_contains_keys, map_contains_value,
    map_contains_values, map_does_not_contain_keys, map_does_not_contain_values, not,
    string_contains, string_contains_any_of, string_ends_with, string_starts_with,
};
use crate::properties::{
    AdditiveIdentityProperty, CharCountProperty, DecimalProperties, DefinedOrderProperty,
    InfinityProperty, IsEmptyProperty, IsNanProperty, LengthProperty, MapProperties,
    MultiplicativeIdentityProperty, SignumProperty,
};
use crate::spec::{
    And, AssertFailure, DiffFormat, DoFail, Expectation, Expecting, Expression, FailingStrategy,
    GetFailures, PanicOnFail, SoftPanic,
};
use crate::std::borrow::{Cow, ToOwned};
use crate::std::error::Error;
use crate::std::fmt::{Debug, Display};
use crate::std::format;
use crate::std::ops::RangeBounds;
use crate::std::slice;
use crate::std::string::{String, ToString};
use crate::std::vec::Vec;
use hashbrown::HashSet;

/// A `DerivedSpec` does assertions on a derived subject while keeping track
/// of the original subject.
///
/// It has similar functionality to a [`Spec`], but additionally holds the
/// original subject. Calling the `and` method switches the subject back to the
/// original subject.
///
/// The derived subject can have its own name and diff format in failure
/// reports.
///
/// [`Spec`]: crate::spec::Spec
pub struct DerivedSpec<'a, O, S> {
    original: O,
    subject: S,
    expression: Expression<'a>,
    diff_format: DiffFormat,
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
    type Output = O;

    fn and(self) -> Self::Output {
        self.original
    }
}

impl<'a, O, S> DerivedSpec<'a, O, S> {
    /// Extracts a property from the current subject.
    ///
    /// The extracting closure gets a reference to the current subject as an
    /// argument and should return a reference to the extracted property. The
    /// given property name is used in failure reports for referencing the
    /// property for which an assertion fails.
    ///
    /// Use this method if you want to extract multiple properties from the
    /// same subject for individual assertions on each of these properties.
    /// To extract another property from the previous subject, call the `and`
    /// method to switch back to the previous subject before calling
    /// `extracting_ref` for the other property.
    ///
    /// # Arguments
    ///
    /// * `property_name` - A name describing the extracted property used for
    ///   referencing that property in failure reports.
    /// * `extract` - A closure that returns a reference to the property to be
    ///   extracted.
    ///
    /// # Examples
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// #[derive(Debug, Clone)]
    /// struct Item {
    ///     name: String,
    ///     price: f32,
    ///     quantity: u32,
    /// }
    ///
    /// struct Order {
    ///     id: String,
    ///     items: Vec<Item>,
    /// }
    ///
    /// let my_order = Order {
    ///     id: "O261234".into(),
    ///     items: vec![
    ///         Item {
    ///             name: "Apple".into(),
    ///             price: 0.99,
    ///             quantity: 6,
    ///         },
    ///         Item {
    ///             name: "Orange".into(),
    ///             price: 1.99,
    ///             quantity: 4,
    ///         },
    ///     ],
    /// };
    ///
    /// assert_that!(my_order)
    ///     .extracting_ref("my_order.items", |o| &o.items)
    ///     .extracting_ref("my_order.items[0].name", |i| &i[0].name)
    ///     .is_equal_to("Apple")
    ///     .and()
    ///     .extracting_ref("my_order.items[1].name", |i| &i[1].name)
    ///     .is_equal_to("Orange")
    ///     .and()
    ///     .extracting_ref("my_order.items[1].quantity", |i| &i[1].quantity)
    ///     .is_equal_to(4)
    ///     .and()  // switches back to `my_order.items`
    ///     .and()  // second call to `and()` switches back to `my_order`
    ///     .extracting_ref("my_order.id", |o| &o.id)
    ///     .is_equal_to("O261234");
    /// ```
    ///
    /// Hint: To avoid having to call the `and()` method two or more times, it
    /// is recommended to first extract all properties from the higher level
    /// subject and then extract fields from deeper down in the hierarchy.
    ///
    /// ```
    /// # use asserting::prelude::*;
    /// #
    /// # #[derive(Debug, Clone)]
    /// # struct Item {
    /// #     name: String,
    /// #     price: f32,
    /// #     quantity: u32,
    /// # }
    /// #
    /// # struct Order {
    /// #     id: String,
    /// #     items: Vec<Item>,
    /// # }
    /// #
    /// # let my_order = Order {
    /// #     id: "O261234".into(),
    /// #     items: vec![
    /// #         Item {
    /// #             name: "Apple".into(),
    /// #             price: 0.99,
    /// #             quantity: 6,
    /// #         },
    /// #         Item {
    /// #             name: "Orange".into(),
    /// #             price: 1.99,
    /// #             quantity: 4,
    /// #         },
    /// #     ],
    /// # };
    /// #
    /// assert_that!(my_order)
    ///     .extracting_ref("my_order.id", |o| &o.id)
    ///     .is_equal_to("O261234")
    ///     .and()
    ///     .extracting_ref("my_order.items", |o| &o.items)
    ///     .extracting_ref("my_order.items[0].name", |i| &i[0].name)
    ///     .is_equal_to("Apple")
    ///     .and()
    ///     .extracting_ref("my_order.items[1].name", |i| &i[1].name)
    ///     .is_equal_to("Orange")
    ///     .and()
    ///     .extracting_ref("my_order.items[1].quantity", |i| &i[1].quantity)
    ///     .is_equal_to(4);
    /// ```
    #[must_use = "a derived spec does nothing unless an assertion method is called"]
    pub fn extracting_ref<F, B, U>(
        self,
        property_name: impl Into<Cow<'a, str>>,
        extract: F,
    ) -> DerivedSpec<'a, Self, U>
    where
        F: FnOnce(&S) -> &B,
        B: ToOwned<Owned = U> + ?Sized,
    {
        let extracted = extract(&self.subject).to_owned();
        let expression = Expression(property_name.into());
        let diff_format = self.diff_format.clone();
        DerivedSpec {
            original: self,
            subject: extracted,
            expression,
            diff_format,
        }
    }

    /// Maps the current subject to some other value.
    ///
    /// It takes a closure that maps the current subject to a new subject and
    /// returns a new `DerivedSpec` with the value returned by the closure as
    /// the new subject. The new subject may have a different type than the
    /// original subject. All other data like description, location, and diff
    /// format are taken over from this `DerivedSpec` into the returned
    /// `DerivedSpec`.
    ///
    /// This method is useful when having a custom type, and one specific
    /// property of this type shall be asserted only. If you want to assert
    /// multiple properties of the same subject, use the [`extracting_ref`]
    /// method instead.
    ///
    /// This method is similar to the [`mapping`] method. In contrast to
    /// [`mapping`], this method does not copy the subject's name
    /// (or expression) but resets it to the default "subject". The idea is
    /// that the "extracted" property is most likely a different subject than
    /// the original one.
    ///
    /// It is recommended to give the extracted property a specific name by
    /// calling the `named` method. This helps with spotting the cause of a
    /// failing assertion.
    ///
    /// This method does not memorize the current subject. Calling `and` on the
    /// extracted property switches back to the original subject of this
    /// `DerivedSpec`. The current subject is omitted. So, `and` always switches
    /// back to the subject before the last `extracting_ref` call.
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// #[derive(Debug, Clone)]
    /// struct Item {
    ///     name: String,
    ///     price: f32,
    ///     quantity: u32,
    /// }
    ///
    /// struct Order {
    ///     id: String,
    ///     items: Vec<Item>,
    /// }
    ///
    /// let my_order = Order {
    ///     id: "O261234".into(),
    ///     items: vec![
    ///         Item {
    ///             name: "Apple".into(),
    ///             price: 0.99,
    ///             quantity: 6,
    ///         },
    ///         Item {
    ///             name: "Orange".into(),
    ///             price: 1.99,
    ///             quantity: 4,
    ///         },
    ///     ],
    /// };
    ///
    /// assert_that!(my_order)
    ///     .extracting_ref("my_order.items", |o| &o.items)
    ///     .extracting(|i| i[0].name.clone())
    ///     .is_equal_to("Apple")
    ///     .and()  // switches back to `my_order` not `my_order.items`
    ///     .extracting(|o| o.id)
    ///     .is_equal_to("O261234");
    /// ```
    ///
    /// [`extracting_ref`]: Self::extracting_ref
    /// [`mapping`]: Self::mapping
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

    /// Maps the current subject to some other value.
    ///
    /// It takes a closure that maps the current subject to a new subject and
    /// returns a new `DerivedSpec` with the value returned by the closure as
    /// the new subject. The new subject may have a different type than the
    /// original subject. All other data like expression, description, and
    /// location are taken over from this `DerivedSpec` into the returned
    /// `DerivedSpec`.
    ///
    /// This method is useful if some type does not implement a trait required
    /// for an assertion.
    ///
    /// `DerivedSpec` also provides the [`extracting()`](DerivedSpec::extracting)
    /// method, which is similar to this method. In contrast to this method,
    /// [`extracting()`](DerivedSpec::extracting) does not copy the subject's
    /// name (or expression) but resets it to the default "subject".
    ///
    /// # Example
    ///
    /// ```
    /// use asserting::prelude::*;
    ///
    /// #[derive(Clone, Copy)]
    /// struct Point {
    ///     x: i64,
    ///     y: i64,
    /// }
    ///
    /// struct Line {
    ///     a: Point,
    ///     b: Point,
    /// }
    ///
    /// let line = Line {
    ///     a: Point { x: 12, y: -64 },
    ///     b: Point { x: -28, y: 17 },
    /// };
    ///
    /// assert_that!(line)
    ///     .extracting_ref("line.a", |l| &l.a)
    ///     .mapping(|p| (p.x, p.y))
    ///     .is_equal_to((12, -64))
    ///     .and()
    ///     .extracting_ref("line.b", |l| &l.b)
    ///     .mapping(|p| (p.x, p.y))
    ///     .is_equal_to((-28, 17));
    /// ```
    ///
    /// The custom type `Point` does not implement the `PartialEq` trait nor
    /// the `Debug` trait, which are both required for an `is_equal_to`
    /// assertion. So we map the subject of the type `Point` to a tuple of its
    /// fields.
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

impl<'a, O, I> DerivedSpec<'a, O, I>
where
    I: IntoIterator,
{
    pub(crate) fn extracting_ref_iter<F, U>(
        self,
        property_name: impl Into<Cow<'a, str>>,
        extract: F,
    ) -> DerivedSpec<'a, DerivedSpec<'a, O, Vec<<I as IntoIterator>::Item>>, Vec<U>>
    where
        for<'b> F: Fn(slice::Iter<'b, <I as IntoIterator>::Item>) -> Vec<U>,
    {
        let property_name = Expression(property_name.into());
        let diff_format = self.diff_format.clone();
        let orig_spec = self.mapping(Vec::from_iter);
        let new_subject = extract(orig_spec.subject.iter());
        DerivedSpec::new(orig_spec, new_subject, property_name, diff_format)
    }
}

impl<O, S> Expecting<S> for DerivedSpec<'_, O, S>
where
    O: DoFail,
{
    fn expecting(mut self, mut expectation: impl Expectation<S>) -> Self {
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
    use crate::spec::{DoFail, Expecting};
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
    use crate::derived_spec::DerivedSpec;
    use crate::expectations::{not, string_matches};
    use crate::spec::{DoFail, Expecting};
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

impl<O, S, E> AssertMapContainsKey<E> for DerivedSpec<'_, O, S>
where
    S: MapProperties + Debug,
    <S as MapProperties>::Key: PartialEq<E> + Debug,
    <S as MapProperties>::Value: Debug,
    E: Debug,
    O: DoFail,
{
    fn contains_key(self, expected_key: E) -> Self {
        self.expecting(map_contains_key(expected_key))
    }

    fn does_not_contain_key(self, expected_key: E) -> Self {
        self.expecting(not(map_contains_key(expected_key)))
    }

    fn contains_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_contains_keys(expected_keys))
    }

    fn does_not_contain_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_does_not_contain_keys(expected_keys))
    }

    fn contains_exactly_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_contains_exactly_keys(expected_keys))
    }
}

impl<O, S, E> AssertMapContainsValue<E> for DerivedSpec<'_, O, S>
where
    S: MapProperties + Debug,
    <S as MapProperties>::Key: Debug,
    <S as MapProperties>::Value: PartialEq<E> + Debug,
    E: Debug,
    O: DoFail,
{
    fn contains_value(self, expected_value: E) -> Self {
        self.expecting(map_contains_value(expected_value))
    }

    fn does_not_contain_value(self, expected_value: E) -> Self {
        self.expecting(not(map_contains_value(expected_value)))
    }

    fn contains_values(self, expected_values: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_contains_values(expected_values))
    }

    fn does_not_contain_values(self, expected_values: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_does_not_contain_values(expected_values))
    }
}

impl<'a, O, S, T> AssertOrderedElements for DerivedSpec<'a, O, S>
where
    S: IntoIterator<Item = T>,
    <S as IntoIterator>::IntoIter: DefinedOrderProperty,
    T: Debug,
    O: DoFail + GetFailures,
{
    type SingleElement = DerivedSpec<'a, O, T>;
    type MultipleElements = DerivedSpec<'a, O, Vec<T>>;

    fn first_element(self) -> Self::SingleElement {
        let spec = self
            .mapping(Vec::from_iter)
            .expecting(has_at_least_number_of_elements(1));
        if spec.has_failures() {
            PanicOnFail.do_fail_with(&spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        spec.extracting(|mut collection| collection.remove(0))
    }

    fn last_element(self) -> Self::SingleElement {
        let spec = self
            .mapping(Vec::from_iter)
            .expecting(has_at_least_number_of_elements(1));
        if spec.has_failures() {
            PanicOnFail.do_fail_with(&spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        spec.extracting(|mut collection| {
            collection.pop().unwrap_or_else(|| {
                unreachable!("Assertion failed and should have panicked! Please report a bug.")
            })
        })
    }

    fn nth_element(self, n: usize) -> Self::SingleElement {
        let min_len = n + 1;
        let spec = self
            .mapping(Vec::from_iter)
            .expecting(has_at_least_number_of_elements(min_len));
        if spec.has_failures() {
            PanicOnFail.do_fail_with(&spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        spec.extracting(|mut collection| collection.remove(n))
    }

    fn elements_at(self, indices: impl IntoIterator<Item = usize>) -> Self::MultipleElements {
        let indices = HashSet::<_>::from_iter(indices);
        self.mapping(|subject| {
            subject
                .into_iter()
                .enumerate()
                .filter_map(|(i, v)| if indices.contains(&i) { Some(v) } else { None })
                .collect()
        })
    }
}

impl<'a, O, S, T, U> AssertOrderedElementsRef for DerivedSpec<'a, O, S>
where
    S: IntoIterator<Item = T>,
    <S as IntoIterator>::IntoIter: DefinedOrderProperty,
    T: ToOwned<Owned = U> + Debug,
    O: DoFail + GetFailures,
{
    type SingleElement = DerivedSpec<'a, DerivedSpec<'a, O, Vec<T>>, U>;
    type MultipleElements = DerivedSpec<'a, DerivedSpec<'a, O, Vec<T>>, Vec<U>>;

    fn first_element_ref(self) -> Self::SingleElement {
        let original_spec = self
            .mapping(Vec::from_iter)
            .expecting(has_at_least_number_of_elements(1));
        if original_spec.has_failures() {
            PanicOnFail.do_fail_with(&original_spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = original_spec.expression();
        let new_subject_name = format!("the first element of {orig_subject_name}");
        original_spec.extracting_ref(new_subject_name, |collection|
            collection.first()
                .unwrap_or_else(||
                    unreachable!("We should have asserted before, that there is at least one element in the collection/iterator. Please file a bug.")
                )
        )
    }

    fn last_element_ref(self) -> Self::SingleElement {
        let original_spec = self
            .mapping(Vec::from_iter)
            .expecting(has_at_least_number_of_elements(1));
        if original_spec.has_failures() {
            PanicOnFail.do_fail_with(&original_spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = original_spec.expression();
        let new_subject_name = format!("the last element of {orig_subject_name}");
        original_spec.extracting_ref(new_subject_name, |collection|
            collection.last()
                .unwrap_or_else(||
                    unreachable!("We should have asserted before, that there is at least one element in the collection/iterator. Please file a bug.")
                )
        )
    }

    fn nth_element_ref(self, n: usize) -> Self::SingleElement {
        let min_len = n + 1;
        let original_spec = self
            .mapping(Vec::from_iter)
            .expecting(has_at_least_number_of_elements(min_len));
        if original_spec.has_failures() {
            PanicOnFail.do_fail_with(&original_spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = original_spec.expression();
        let new_subject_name = format!("{orig_subject_name}[{n}]");
        original_spec.extracting_ref(new_subject_name, |collection|
            collection.get(n)
                .unwrap_or_else(||
                    unreachable!("We should have asserted before, that there is at least one element in the collection/iterator. Please file a bug.")
                )
        )
    }

    fn elements_ref_at(self, indices: impl IntoIterator<Item = usize>) -> Self::MultipleElements {
        let indices = Vec::from_iter(indices);
        let orig_subject_name = self.expression();
        let new_subject_name = format!("{orig_subject_name} at positions {indices:?}");
        let indices = HashSet::<_>::from_iter(indices);
        let original_spec = self.mapping(Vec::from_iter);
        original_spec.extracting_ref_iter(new_subject_name, |collection| {
            collection
                .enumerate()
                .filter_map(|(i, e)| {
                    if indices.contains(&i) {
                        Some(e.to_owned())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod tests;
