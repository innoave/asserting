use crate::prelude::*;
#[cfg(feature = "std")]
use crate::std::vec;
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

#[derive(Debug)]
struct CustomCollection<T> {
    inner: Vec<T>,
}

struct CustomIter<T> {
    inner: vec::IntoIter<T>,
}

impl<T> Iterator for CustomIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> IntoIterator for CustomCollection<T> {
    type Item = T;
    type IntoIter = CustomIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        CustomIter {
            inner: self.inner.into_iter(),
        }
    }
}

impl<T> IsEmptyProperty for CustomCollection<T> {
    fn is_empty_property(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> LengthProperty for CustomCollection<T> {
    fn length_property(&self) -> usize {
        self.inner.len()
    }
}

#[test]
fn custom_collection_is_empty() {
    let subject: CustomCollection<i32> = CustomCollection { inner: vec![] };

    assert_that(subject).is_empty();
}

#[test]
fn custom_collection_is_not_empty() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17],
    };

    assert_that(subject).is_not_empty();
}

#[test]
fn custom_collection_has_length() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13],
    };

    assert_that(subject).has_length(6);
}

#[test]
fn custom_collection_has_length_in_range() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19],
    };

    assert_that(subject).has_length_in_range(7..=8);
}

#[test]
fn custom_collection_contains() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    };

    assert_that(subject).contains(19).contains(43).contains(1);
}

#[test]
fn verify_custom_collection_contains_fails() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    };

    let failures = verify_that(subject)
        .named("my_thing")
        .contains(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing to contain 42
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: 42
"]
    );
}

#[test]
fn custom_iterator_contains() {
    let subject: CustomIter<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    }
    .into_iter();

    assert_that(subject).contains(19).contains(43).contains(1);
}

#[test]
fn verify_custom_iterator_contains_fails() {
    let subject: CustomIter<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    }
    .into_iter();

    let failures = verify_that(subject)
        .named("my_thing")
        .contains(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing to contain 42
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: 42
"]
    );
}
