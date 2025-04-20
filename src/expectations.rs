//! Definitions of the expectations that are provided by this crate.

#![allow(missing_docs)]
#![warn(clippy::return_self_not_must_use)]

use crate::std::ops::RangeInclusive;
use crate::std::{string::String, vec::Vec};
use hashbrown::HashSet;

#[must_use]
pub struct IsTrue;

#[must_use]
pub struct IsFalse;

#[must_use]
pub struct IsEqualTo<E> {
    pub expected: E,
}

#[must_use]
pub struct IsNotEqualTo<E> {
    pub expected: E,
}

#[must_use]
pub struct IsCloseTo<E, M> {
    pub expected: E,
    pub margin: M,
}

impl<E, M> IsCloseTo<E, M>
where
    M: Default,
{
    pub fn new(expected: E) -> Self {
        Self {
            expected,
            margin: M::default(),
        }
    }
}

impl<E, M> IsCloseTo<E, M> {
    pub fn within_margin(mut self, margin: impl Into<M>) -> Self {
        self.margin = margin.into();
        self
    }
}

#[must_use]
pub struct IsNotCloseTo<E, M> {
    pub expected: E,
    pub margin: M,
}

impl<E, M> IsNotCloseTo<E, M>
where
    M: Default,
{
    pub fn new(expected: E) -> Self {
        Self {
            expected,
            margin: M::default(),
        }
    }
}

impl<E, M> IsNotCloseTo<E, M> {
    pub fn within_margin(mut self, margin: impl Into<M>) -> Self {
        self.margin = margin.into();
        self
    }
}

#[must_use]
pub struct IsLessThan<E> {
    pub expected: E,
}

#[must_use]
pub struct IsAtMost<E> {
    pub expected: E,
}

#[must_use]
pub struct IsGreaterThan<E> {
    pub expected: E,
}

#[must_use]
pub struct IsAtLeast<E> {
    pub expected: E,
}

#[must_use]
pub struct IsBefore<E> {
    pub expected: E,
}

#[must_use]
pub struct IsAfter<E> {
    pub expected: E,
}

#[must_use]
pub struct IsBetween<E> {
    pub min: E,
    pub max: E,
}

#[must_use]
pub struct IsInRange<E> {
    pub expected_range: RangeInclusive<E>,
}

#[must_use]
pub struct IsNotInRange<E> {
    pub expected_range: RangeInclusive<E>,
}

#[must_use]
pub struct IsSome;

#[must_use]
pub struct IsNone;

#[must_use]
pub struct HasValue<E> {
    pub expected: E,
}

#[must_use]
pub struct IsOk;

#[must_use]
pub struct IsErr;

#[must_use]
pub struct HasError<E> {
    pub expected: E,
}

#[must_use]
pub struct IsEmpty;

#[must_use]
pub struct IsNotEmpty;

#[must_use]
pub struct HasLength<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasLengthInRange<E> {
    pub expected_range: RangeInclusive<E>,
}

#[must_use]
pub struct HasLengthLessThan<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasLengthGreaterThan<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasAtMostLength<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasAtLeastLength<E> {
    pub expected_length: E,
}

#[must_use]
pub struct HasCharCount<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasCharCountInRange<E> {
    pub expected_range: RangeInclusive<E>,
}

#[must_use]
pub struct HasCharCountLessThan<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasCharCountGreaterThan<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasAtMostCharCount<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct HasAtLeastCharCount<E> {
    pub expected_char_count: E,
}

#[must_use]
pub struct StringContains<E> {
    pub expected: E,
}

#[must_use]
pub struct StringContainsAnyOf<E> {
    pub expected: E,
}

#[must_use]
pub struct StringStartWith<E> {
    pub expected: E,
}

#[must_use]
pub struct StringEndsWith<E> {
    pub expected: E,
}

#[must_use]
pub struct IterContains<E> {
    pub expected: E,
}

#[must_use]
pub struct IterContainsExactlyInAnyOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsExactlyInAnyOrder<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsAnyOf<E> {
    pub expected: Vec<E>,
}

#[must_use]
pub struct IterContainsAllOf<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IterContainsAllOf<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsOnly<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsOnly<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsOnlyOnce<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
    pub duplicates: HashSet<usize>,
}

impl<E> IterContainsOnlyOnce<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
            duplicates: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsExactly<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
    pub out_of_order: HashSet<usize>,
}

impl<E> IterContainsExactly<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
            out_of_order: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsSequence<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsSequence<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterContainsAllInOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IterContainsAllInOrder<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterStartsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterStartsWith<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct IterEndsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterEndsWith<E> {
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

#[must_use]
pub struct Predicate<F> {
    pub predicate: F,
    pub message: Option<String>,
}

#[cfg(feature = "panic")]
pub use panic::{DoesNotPanic, DoesPanic};

#[cfg(feature = "panic")]
mod panic {
    use std::any::Any;

    #[must_use]
    #[derive(Default)]
    pub struct DoesPanic {
        pub expected_message: Option<String>,
        pub actual_message: Option<String>,
    }

    impl DoesPanic {
        pub fn with_any_message() -> Self {
            Self::default()
        }

        pub fn with_message(message: impl Into<String>) -> Self {
            Self {
                expected_message: Some(message.into()),
                actual_message: None,
            }
        }
    }

    #[must_use]
    #[derive(Default)]
    pub struct DoesNotPanic {
        pub actual_message: Option<Box<dyn Any + Send>>,
    }
}
