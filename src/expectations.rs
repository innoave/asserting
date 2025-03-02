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
