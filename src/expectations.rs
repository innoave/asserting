#[cfg(not(any(feature = "std", test)))]
use alloc::string::String;

pub struct IsTrue;

pub struct IsFalse;

pub struct IsEqualTo<E> {
    pub expected: E,
}

pub struct IsNotEqualTo<E> {
    pub expected: E,
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

pub struct HasLength {
    pub expected_length: usize,
}

pub struct Contains<E> {
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
    use core::any::Any;
    use core::cell::RefCell;
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
