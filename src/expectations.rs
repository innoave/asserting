use crate::std::cell::RefCell;
use crate::std::ops::RangeInclusive;
#[cfg(any(feature = "std", test))]
use crate::std::rc::Rc;
#[cfg(not(any(feature = "std", test)))]
use alloc::{rc::Rc, string::String, vec::Vec};
use hashbrown::HashSet;

pub struct IsTrue;

pub struct IsFalse;

pub struct IsEqualTo<E> {
    pub expected: E,
}

pub struct IsNotEqualTo<E> {
    pub expected: E,
}

pub struct IsLessThan<E> {
    pub expected: E,
}

pub struct IsLessThanOrEqualTo<E> {
    pub expected: E,
}

pub struct IsGreaterThan<E> {
    pub expected: E,
}

pub struct IsGreaterThanOrEqualTo<E> {
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

pub struct IterContains<E> {
    pub expected: E,
}

pub struct ContainsAnyOf<E> {
    pub expected: E,
}

pub struct StartWith<E> {
    pub expected: E,
}

pub struct EndsWith<E> {
    pub expected: E,
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

pub struct IterContainsExactlyInAnyOrder<E> {
    pub(crate) expected: Vec<E>,
    pub(crate) missing: Rc<RefCell<HashSet<usize>>>,
    pub(crate) extra: Rc<RefCell<HashSet<usize>>>,
}

impl<E> IterContainsExactlyInAnyOrder<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: Rc::new(RefCell::new(HashSet::new())),
            extra: Rc::new(RefCell::new(HashSet::new())),
        }
    }
}

pub struct IterContainsAnyOf<E> {
    pub expected: Vec<E>,
}

pub struct IterContainsAllOf<E> {
    pub(crate) expected: Vec<E>,
    pub(crate) missing: Rc<RefCell<HashSet<usize>>>,
}

impl<E> IterContainsAllOf<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            missing: Rc::new(RefCell::new(HashSet::new())),
        }
    }
}

pub struct IterContainsOnly<E> {
    pub(crate) expected: Vec<E>,
    pub(crate) extra: Rc<RefCell<HashSet<usize>>>,
}

impl<E> IterContainsOnly<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: Rc::new(RefCell::new(HashSet::new())),
        }
    }
}

pub struct IterContainsOnlyOnce<E> {
    pub(crate) expected: Vec<E>,
    pub(crate) extra: Rc<RefCell<HashSet<usize>>>,
    pub(crate) duplicates: Rc<RefCell<HashSet<usize>>>,
}

impl<E> IterContainsOnlyOnce<E> {
    #[must_use]
    pub fn new(expected: Vec<E>) -> Self {
        Self {
            expected,
            extra: Rc::new(RefCell::new(HashSet::new())),
            duplicates: Rc::new(RefCell::new(HashSet::new())),
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
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct DoesPanic {
        pub(crate) expected_message: Option<String>,
        pub(crate) actual_message: Rc<RefCell<Option<String>>>,
    }

    impl DoesPanic {
        #[must_use]
        pub fn with_any_message() -> Self {
            Self::default()
        }

        pub fn with_message(message: impl Into<String>) -> Self {
            Self {
                expected_message: Some(message.into()),
                actual_message: Rc::new(RefCell::new(None)),
            }
        }
    }

    #[derive(Default)]
    pub struct DoesNotPanic {
        pub(crate) actual_message: Rc<RefCell<Option<Box<dyn Any + Send>>>>,
    }
}
