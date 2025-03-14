//! Definitions of the assertions that are provided by this crate.
//!
//! Assertions define the methods that are used to assert that the actual test
//! result is as expected. Assertions are defined by traits that are implemented
//! for one or several types. An assertion can be applied to all types that
//! implement this assertion.
//!
//! All assertions provided by this crate are defined in this module. Browse
//! over the traits in this module to get information about all provided
//! assertions.
#![allow(clippy::wrong_self_convention, clippy::return_self_not_must_use)]

use crate::spec::Spec;
use crate::std::ops::RangeInclusive;

/// Assert whether two values are equal or not.
///
/// These assertions are implemented for all types that implement `PartialEq<E>`
/// with `E` being the type of the expected value.
///
/// ## Examples
///
/// assert that a value of type `String` is equal to an expected value of type
/// `&str`:
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = "ea rebum dignissim suscipit".to_string();
///
/// assert_that!(subject).is_equal_to("ea rebum dignissim suscipit");
/// ```
///
/// assert that two integers are equal:
///
/// ```
/// use asserting::prelude::*;
///
/// let the_answer = 42;
///
/// assert_that!(the_answer).is_equal_to(42);
/// ```
pub trait AssertEquality<E> {
    /// Verifies that the subject is equal to some other value.
    #[track_caller]
    fn is_equal_to(self, expected: E) -> Self;

    /// Verifies that subject is not equal to some other value.
    #[track_caller]
    fn is_not_equal_to(self, expected: E) -> Self;
}

/// Assert whether a value is greater than or less than another value, as well
/// as at most as big or at least as big as another value.
///
/// These assertions are implemented for all types that implement
/// `PartialOrd<E>` with `E` being the type of the expected value the subject
/// is being compared to.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let some_result: u16 = 42;
///
/// assert_that!(some_result).is_at_most(43);
/// assert_that!(some_result).is_at_most(42);
/// assert_that!(some_result).is_at_least(42);
/// assert_that!(some_result).is_at_least(41);
/// assert_that!(some_result).is_greater_than(41);
/// assert_that!(some_result).is_less_than(43);
///```
pub trait AssertOrder<E> {
    /// Verifies that the subject is less than some expected value.
    #[track_caller]
    fn is_less_than(self, expected: E) -> Self;

    /// Verifies that the subject is greater than some expected value.
    #[track_caller]
    fn is_greater_than(self, expected: E) -> Self;

    /// Verifies that the subject is less than or equal to some expected value.
    #[track_caller]
    fn is_at_most(self, expected: E) -> Self;

    /// Verifies that the subject is greater than or equal to some expected
    /// value.
    #[track_caller]
    fn is_at_least(self, expected: E) -> Self;
}

/// Assert whether a value is within an expected range. The expected range must
/// be a closed range with both ends inclusive.
///
/// These assertions are implemented for all types `T` that implement
/// `PartialOrd<E>` with `E` being the type of the expected values and `E` must
/// implement `PartialOrd<T>`.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let some_char = 'M';
///
/// assert_that!(some_char).is_in_range('A'..='Z');
/// assert_that!(some_char).is_not_in_range('a'..='z');
/// ```
pub trait AssertInRange<E> {
    /// Verifies that the subject is within the expected range.
    ///
    /// The expected range must be a closed range with both ends inclusive.
    #[track_caller]
    fn is_in_range(self, range: RangeInclusive<E>) -> Self;

    /// Verifies that the subject is not within the expected range.
    ///
    /// The expected range must be a closed range with both ends inclusive.
    #[track_caller]
    fn is_not_in_range(self, range: RangeInclusive<E>) -> Self;
}

/// Assert whether some value or expression is true or false.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = 42 > 41;
/// assert_that!(subject).is_true();
///
/// assert_that!(12 == 12).is_true();
///
/// assert_that!(42 < 42).is_false();
/// ```
pub trait AssertBoolean {
    /// Verifies that the subject is true.
    #[track_caller]
    fn is_true(self) -> Self;

    /// Verifies that the subject is false.
    #[track_caller]
    fn is_false(self) -> Self;
}

/// Assert whether a string, collection or iterator is empty or not.
///
/// These assertions are implemented for all types `T` that implement the
/// trait [`IsEmptyProperty`](crate::properties::IsEmptyProperty). This
/// property trait is implemented for string like types and collection like
/// types of the `std` lib. For example:
///
/// * `String`, `&str`, `OsString`, `CString`, etc.
/// * `Vec`, array, slice, `VecDeque`, `LinkedList`, etc.
/// * `HashMap`, `HashSet`, `BTreeSet`, etc.
///
/// ## Examples
///
/// ```
/// use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
/// use asserting::prelude::*;
///
/// let some_string = String::new();
/// assert_that!(some_string).is_empty();
///
/// let some_str = "ad praesent aliqua qui";
/// assert_that!(some_str).is_not_empty();
///
/// let some_vec: Vec<String> = vec![];
/// assert_that!(some_vec).is_empty();
///
/// let some_array = [12, 24, 36, 48];
/// assert_that!(some_array).is_not_empty();
///
/// let some_slice: &[_] = &['a', 'b', 'c'][..];
/// assert_that!(some_slice).is_not_empty();
///
/// let some_btree_set = BTreeSet::<i64>::new();
/// assert_that!(&some_btree_set).is_empty();
///
/// let some_dequeue = VecDeque::<String>::new();
/// assert_that!(some_dequeue).is_empty();
/// ```
///
/// with crate feature `std` enabled:
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {
/// # }
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use std::collections::{HashMap, HashSet};
/// use asserting::prelude::*;
///
/// let some_set: HashSet<_> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
/// assert_that!(&some_set).is_not_empty();
///
/// let some_map: HashMap<String, usize> = HashMap::new();
/// assert_that!(some_map).is_empty();
/// # }
/// ```
pub trait AssertEmptiness {
    /// Verifies that the subject is empty.
    #[track_caller]
    fn is_empty(self) -> Self;

    /// Verifies that the subject is not empty.
    #[track_caller]
    fn is_not_empty(self) -> Self;
}

/// Assert the length of a subject.
///
/// These assertions are implemented for all types `T` that implement the
/// trait [`LengthProperty`](crate::properties::LengthProperty). This
/// property trait is implemented for string like types and collection like
/// types of the `std` lib. For example:
///
/// * `String`, `&str`, `OsString`, `OsStr`
/// * `Vec`, array, slice, `VecDeque`, `LinkedList`, etc.
/// * `HashMap`, `HashSet`, `BTreeSet`, etc.
///
/// ## Examples
///
/// ```
/// use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
/// use asserting::prelude::*;
///
/// let some_str = "takimata te iriure nonummy";
/// assert_that!(some_str).has_length(26);
///
/// let some_array = [12, 24, 36, 48];
/// assert_that!(some_array).has_length(4);
///
/// let some_slice: &[_] = &['a', 'b', 'c'][..];
/// assert_that!(some_slice).has_length(3);
///
/// let some_btree_set = BTreeSet::from_iter([1, 3, 5, 7, 11, 13, 17, 19]);
/// assert_that!(some_btree_set).has_length(8);
///
/// let some_dequeue = VecDeque::from_iter(["one", "two", "three"]);
/// assert_that!(&some_dequeue).has_length(3);
/// ```
///
/// with crate feature `std` enabled:
///
/// ```
/// # #[cfg(not(feature = "std"))]
/// # fn main() {
/// # }
/// # #[cfg(feature = "std")]
/// # fn main() {
/// use std::collections::{HashMap, HashSet};
/// use asserting::prelude::*;
///
/// let some_set: HashSet<u8> = [1, 3, 5, 7, 11, 13, 17, 19].into_iter().collect();
/// assert_that!(some_set).has_length(8);
///
/// let some_map: HashMap<char, usize> = [('A', 25), ('B', 2), ('C', 12), ('D', 18)].into_iter().collect();
/// assert_that!(&some_map).has_length(4);
/// # }
/// ```
pub trait AssertHasLength<E> {
    /// Verifies that the subject has the expected length.
    #[track_caller]
    fn has_length(self, expected: E) -> Self;

    /// Verifies that the subject has a length in the expected range.
    ///
    /// The expected range must be a closed range with both ends inclusive.
    #[track_caller]
    fn has_length_in_range(self, range: RangeInclusive<E>) -> Self;
}

/// Assert whether a subject of the `Option` type holds some value or has none.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = Some("nisl possim nobis non".to_string());
/// assert_that!(subject).is_some();
///
/// #[derive(Debug)]
/// struct MyType;
///
/// let subject: Option<MyType> = None;
/// assert_that!(subject).is_none();
/// ```
pub trait AssertOption {
    /// Verifies that the subject has some value.
    #[track_caller]
    fn is_some(self) -> Self;

    /// Verifies that the subject has no value.
    #[track_caller]
    fn is_none(self) -> Self;
}

/// Assert whether a subject of the `Result` type holds some value or an error.
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Result<f64, String> = Ok(-3.14);
/// assert_that!(subject).is_ok();
///
/// let subject: Result<(), String> = Err("consequat sanctus ea exercitation".to_string());
/// assert_that!(subject).is_err();
/// ```
pub trait AssertResult {
    /// Verifies that the subject has an ok value.
    #[track_caller]
    fn is_ok(self) -> Self;

    /// Verifies that the subject has an err value.
    #[track_caller]
    fn is_err(self) -> Self;
}

/// Assert that a subject of some container type holds a value that is equal to
/// the expected one.
///
/// This assertion is implemented for the `Option` type and the `Result` type.
/// For `Option` it compares the value to the expected one if it has some or
/// fails if it holds none. For `Result` it compares the ok value to the
/// expected one if it is an ok or fails if it holds an error.
///
/// The value type of the `Option` or `Result` must implement `PartialEq<E>`
/// where `E` is the type of the expected value.
///
/// To assert the error value of a `Result` use [`AssertHasError::has_error`].
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject = Some(-3.14);
/// assert_that!(subject).has_value(-3.14);
///
/// let subject: Result<f64, String> = Ok(6.28);
/// assert_that!(subject).has_value(6.28);
/// ```
pub trait AssertHasValue<E> {
    /// Verifies that the subject holds a value that is equal to the expected
    /// one.
    ///
    /// For `Option` it compares the value in `Some(value)` and for `Result`
    /// it compares the value in `Ok(value)`. If an `Option` is `None` or a
    /// `Result` is `Err(error)` than the assertion fails.
    #[track_caller]
    fn has_value(self, expected: E) -> Self;
}

/// Assert that a subject of some container type holds an error value that is
/// equal to the expected one.
///
/// This assertion is implemented for the `Result` type. It compares the value
/// in `Err(value)` with the expected one. The error type in the `Result` must
/// implement `PartialEq<E>` where `E` is the type of the expected error value.
///
/// To assert the ok value of a `Result` use [`AssertHasValue::has_value`].
///
/// ## Examples
///
/// ```
/// use asserting::prelude::*;
///
/// let subject: Result<(), String> = Err("labore gubergren ut ipsum".to_string());
/// assert_that!(subject).has_error("labore gubergren ut ipsum");
/// ```
pub trait AssertHasError<E> {
    /// Verifies that the subject holds an error value that is equal to the
    /// expected one.
    ///
    /// For `Result` it compares the value in `Err(value)`. If the `Result`
    /// holds an `Ok(value)` the assertion fails.
    #[track_caller]
    fn has_error(self, expected: E) -> Self;
}

pub trait AssertStringPattern<E> {
    #[track_caller]
    fn contains(self, pattern: E) -> Self;

    #[track_caller]
    fn starts_with(self, pattern: E) -> Self;

    #[track_caller]
    fn ends_with(self, pattern: E) -> Self;
}

pub trait AssertStringContainsAnyOf<E> {
    #[track_caller]
    fn contains_any_of(self, pattern: E) -> Self;
}

pub trait AssertIteratorContains<'a, U, E, R> {
    #[track_caller]
    fn contains(self, element: E) -> Spec<'a, U, R>;
}

/// Assert values in a collection.
///
/// These assertions do not rely on the order in which the collection iterates
/// over its values.
pub trait AssertIteratorContainsInAnyOrder<'a, S, E, R> {
    /// Verifies that the actual collection/iterator contains exactly the given
    /// values and nothing else in any order.
    #[track_caller]
    fn contains_exactly_in_any_order(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains at least one of
    /// the given values.
    #[track_caller]
    fn contains_any_of(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// in any order.
    ///
    /// The collection/iterator may contain more values than the given ones, but
    /// at least all the specified ones.
    #[track_caller]
    fn contains_all_of(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains only the given
    /// values and nothing else in any order and ignoring duplicates.
    ///
    /// The collection may contain fewer values than the expected ones.
    #[track_caller]
    fn contains_only(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains only the given
    /// values in any order and each of them only once.
    ///
    /// The collection may contain fewer values than the expected ones.
    #[track_caller]
    fn contains_only_once(self, expected: E) -> Spec<'a, S, R>;
}

/// Assert values in an ordered collection.
///
/// These assertions are applicable to collections which iterate over their
/// values in a defined order.
pub trait AssertIteratorContainsInOrder<'a, S, E, R> {
    /// Verifies that the actual collection/iterator contains exactly the given
    /// values and nothing else in the given order.
    #[track_caller]
    fn contains_exactly(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given sequence
    /// of values in the given order and without extra values between the
    /// sequence values.
    ///
    /// May contain more values as in the given sequence before and after the
    /// sequence.
    #[track_caller]
    fn contains_sequence(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains all the given
    /// values and in the given order, possible with other values between them.
    #[track_caller]
    fn contains_all_in_order(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// as the first elements in order.
    #[track_caller]
    fn starts_with(self, expected: E) -> Spec<'a, S, R>;

    /// Verifies that the actual collection/iterator contains the given values
    /// as the last elements in order.
    #[track_caller]
    fn ends_with(self, expected: E) -> Spec<'a, S, R>;
}

/// Assert the order of the values within a collection.
///
/// These assertions are applicable to ordered collections only.
pub trait AssertIsSorted {
    /// Verifies that the actual collection is sorted in ascending order.
    #[track_caller]
    fn is_sorted_ascending(self) -> Self;

    /// Verifies that the actual collection is sorted in descending order.
    #[track_caller]
    fn is_sorted_descending(self) -> Self;
}

/// Assert that the code under test panics, panics with a certain message or
/// does not panic.
#[cfg(feature = "panic")]
pub trait AssertCodePanics {
    /// Verifies that the actual code under test does not panic.
    #[track_caller]
    fn does_not_panic(self) -> Self;

    /// Verifies that the actual code under test panics with any message.
    #[track_caller]
    fn panics(self) -> Self;

    /// Verifies that the actual code under test panics with the given
    /// message.
    #[track_caller]
    fn panics_with_message(self, message: impl Into<String>) -> Self;
}
