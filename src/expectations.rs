//! Definitions of the expectations that are provided by this crate.

#![allow(missing_docs)]

use crate::std::ops::RangeInclusive;
use crate::std::{string::String, vec::Vec};
use hashbrown::HashSet;

pub struct IsTrue;

pub struct IsFalse;

pub struct IsEqualTo<E> {
    pub expected: E,
}

pub struct IsNotEqualTo<E> {
    pub expected: E,
}

pub struct IsCloseTo<E, M> {
    pub expected: E,
    pub margin: M,
}

impl<E, M> IsCloseTo<E, M>
where
    M: Default,
{
    #[must_use]
    pub fn new(expected: E) -> Self {
        Self {
            expected,
            margin: M::default(),
        }
    }
}

impl<E, M> IsCloseTo<E, M> {
    #[must_use]
    pub fn within_margin(mut self, margin: impl Into<M>) -> Self {
        self.margin = margin.into();
        self
    }
}

pub struct IsNotCloseTo<E, M> {
    pub expected: E,
    pub margin: M,
}

impl<E, M> IsNotCloseTo<E, M>
where
    M: Default,
{
    #[must_use]
    pub fn new(expected: E) -> Self {
        Self {
            expected,
            margin: M::default(),
        }
    }
}

impl<E, M> IsNotCloseTo<E, M> {
    #[must_use]
    pub fn within_margin(mut self, margin: impl Into<M>) -> Self {
        self.margin = margin.into();
        self
    }
}

pub struct IsLessThan<E> {
    pub expected: E,
}

pub struct IsAtMost<E> {
    pub expected: E,
}

pub struct IsGreaterThan<E> {
    pub expected: E,
}

pub struct IsAtLeast<E> {
    pub expected: E,
}

pub struct IsInRange<E> {
    pub expected_range: RangeInclusive<E>,
}

pub struct IsNotInRange<E> {
    pub expected_range: RangeInclusive<E>,
}

pub struct IsSome;

pub struct IsNone;

pub struct HasValue<E> {
    pub expected: E,
}

pub struct IsOk;

pub struct IsErr;

pub struct HasError<E> {
    pub expected: E,
}

pub struct IsEmpty;

pub struct IsNotEmpty;

pub struct HasLength<E> {
    pub expected_length: E,
}

pub struct HasLengthInRange<E> {
    pub expected_range: RangeInclusive<E>,
}

pub struct StringContains<E> {
    pub expected: E,
}

pub struct StringContainsAnyOf<E> {
    pub expected: E,
}

pub struct StringStartWith<E> {
    pub expected: E,
}

pub struct StringEndsWith<E> {
    pub expected: E,
}

pub struct IterContains<E> {
    pub expected: E,
}

pub struct IterContainsExactlyInAnyOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsExactlyInAnyOrder<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

pub struct IterContainsAnyOf<E> {
    pub expected: Vec<E>,
}

pub struct IterContainsAllOf<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IterContainsAllOf<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

pub struct IterContainsOnly<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsOnly<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
        }
    }
}

pub struct IterContainsOnlyOnce<E> {
    pub expected: Vec<E>,
    pub extra: HashSet<usize>,
    pub duplicates: HashSet<usize>,
}

impl<E> IterContainsOnlyOnce<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: HashSet::new(),
            duplicates: HashSet::new(),
        }
    }
}

pub struct IterContainsExactly<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
    pub out_of_order: HashSet<usize>,
}

impl<E> IterContainsExactly<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
            out_of_order: HashSet::new(),
        }
    }
}

pub struct IterContainsSequence<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterContainsSequence<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

pub struct IterContainsAllInOrder<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
}

impl<E> IterContainsAllInOrder<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
        }
    }
}

pub struct IterStartsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterStartsWith<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

pub struct IterEndsWith<E> {
    pub expected: Vec<E>,
    pub missing: HashSet<usize>,
    pub extra: HashSet<usize>,
}

impl<E> IterEndsWith<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: HashSet::new(),
            extra: HashSet::new(),
        }
    }
}

pub struct Predicate<F> {
    pub predicate: F,
    pub message: Option<String>,
}

#[cfg(feature = "panic")]
pub use panic::DoesNotPanic;
#[cfg(feature = "panic")]
pub use panic::DoesPanic;

#[cfg(feature = "panic")]
mod panic {
    use std::any::Any;

    #[derive(Default)]
    pub struct DoesPanic {
        pub expected_message: Option<String>,
        pub actual_message: Option<String>,
    }

    impl DoesPanic {
        #[must_use]
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

    #[derive(Default)]
    pub struct DoesNotPanic {
        pub actual_message: Option<Box<dyn Any + Send>>,
    }
}
