use crate::assertions::{AssertIteratorContains, AssertIteratorContainsInAnyOrder};
use crate::expectations::{
    IterContains, IterContainsAllOf, IterContainsAnyOf, IterContainsExactlyInAnyOrder,
    IterContainsOnly, IterContainsOnlyOnce,
};
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
#[cfg(not(any(feature = "std", test)))]
use alloc::{format, string::String, vec::Vec};
use hashbrown::HashSet;

impl<'a, S, T, E, R> AssertIteratorContains<'a, Vec<T>, E, R> for Spec<'a, S, R>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn contains(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.map(Vec::from_iter)
            .expecting(IterContains { expected })
    }
}

impl<T, E> Expectation<Vec<T>> for IterContains<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Vec<T>) -> bool {
        subject.iter().any(|e| e == &self.expected)
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        format!(
            "expected {expression} to contain {:?}\n   but was: {actual:?}\n  expected: {:?}",
            &self.expected, &self.expected
        )
    }
}

impl<'a, S, T, E, R> AssertIteratorContainsInAnyOrder<'a, Vec<T>, E, R> for Spec<'a, S, R>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<<E as IntoIterator>::Item> + Debug,
    E: IntoIterator,
    <E as IntoIterator>::Item: Debug,
    R: FailingStrategy,
{
    fn contains_exactly_in_any_order(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.map(Vec::from_iter)
            .expecting(IterContainsExactlyInAnyOrder::new(Vec::from_iter(expected)))
    }

    fn contains_any_of(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.map(Vec::from_iter).expecting(IterContainsAnyOf {
            expected: Vec::from_iter(expected),
        })
    }

    fn contains_all_of(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.map(Vec::from_iter)
            .expecting(IterContainsAllOf::new(Vec::from_iter(expected)))
    }

    fn contains_only(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.map(Vec::from_iter)
            .expecting(IterContainsOnly::new(Vec::from_iter(expected)))
    }

    fn contains_only_once(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.map(Vec::from_iter)
            .expecting(IterContainsOnlyOnce::new(Vec::from_iter(expected)))
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsExactlyInAnyOrder<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Vec<T>) -> bool {
        let mut missing = self.missing.borrow_mut();
        let mut extra = self.extra.borrow_mut();
        *extra = (0..subject.len()).collect();

        let mut subject_values = subject.iter().enumerate().collect::<Vec<_>>();
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if let Some(index) = subject_values
                .iter()
                .position(|(_, value)| *value == expected)
            {
                let (subject_index, _) = subject_values.remove(index);
                extra.remove(&subject_index);
            } else {
                missing.insert(expected_index);
            }
        }

        extra.is_empty() && missing.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing.borrow(), &self.expected);
        let extra = collect_values(&self.extra.borrow(), actual);

        format!(
            r"expected {expression} to contain exactly in any order {:?}
   but was: {actual:?}
  expected: {:?}
   missing: {missing:?}
     extra: {extra:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsAnyOf<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Vec<T>) -> bool {
        for expected in &self.expected {
            if subject.iter().any(|value| value == expected) {
                return true;
            }
        }
        false
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        format!(
            r"expected {expression} to contain any of {:?}, but contained none of them
   but was: {actual:?}
  expected: {:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsAllOf<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Vec<T>) -> bool {
        let mut missing = self.missing.borrow_mut();

        for (expected_index, expected) in self.expected.iter().enumerate() {
            if !subject.iter().any(|value| value == expected) {
                missing.insert(expected_index);
            }
        }

        missing.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing.borrow(), &self.expected);

        format!(
            r"expected {expression} to contain all of {:?}
   but was: {actual:?}
  expected: {:?}
   missing: {missing:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsOnly<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Vec<T>) -> bool {
        let mut extra = self.extra.borrow_mut();

        for (actual_index, value) in subject.iter().enumerate() {
            if !self.expected.iter().any(|expected| value == expected) {
                extra.insert(actual_index);
            }
        }

        extra.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let extra = collect_values(&self.extra.borrow(), actual);

        format!(
            r"expected {expression} to contain only {:?}
   but was: {actual:?}
  expected: {:?}
     extra: {extra:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsOnlyOnce<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&self, subject: &Vec<T>) -> bool {
        let mut extra = self.extra.borrow_mut();
        let mut duplicates = self.duplicates.borrow_mut();

        for (actual_index, value) in subject.iter().enumerate() {
            if let Some(expected) = self.expected.iter().find(|expected| value == *expected) {
                if subject.iter().filter(|actual| *actual == expected).count() > 1 {
                    duplicates.insert(actual_index);
                }
            } else {
                extra.insert(actual_index);
            }
        }

        duplicates.is_empty() && extra.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let extra = collect_values(&self.extra.borrow(), actual);
        let duplicates = collect_values(&self.duplicates.borrow(), actual);

        format!(
            r"expected {expression} to contain only once {:?}
     but was: {actual:?}
    expected: {:?}
       extra: {extra:?}
  duplicates: {duplicates:?}",
            &self.expected, &self.expected
        )
    }
}

fn collect_values<'a, T>(indices: &HashSet<usize>, collection: &'a [T]) -> Vec<&'a T> {
    collection
        .iter()
        .enumerate()
        .filter_map(|(idx, value)| {
            if indices.contains(&idx) {
                Some(value)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
