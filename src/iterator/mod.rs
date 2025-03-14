//! Implementations of assertions for `Iterator` values.

use crate::assertions::{
    AssertIteratorContains, AssertIteratorContainsInAnyOrder, AssertIteratorContainsInOrder,
};
use crate::expectations::{
    IterContains, IterContainsAllInOrder, IterContainsAllOf, IterContainsAnyOf,
    IterContainsExactly, IterContainsExactlyInAnyOrder, IterContainsOnly, IterContainsOnlyOnce,
    IterContainsSequence, IterEndsWith, IterStartsWith,
};
use crate::properties::DefinedOrder;
use crate::spec::{Expectation, Expression, FailingStrategy, Spec};
use crate::std::cmp::Ordering;
use crate::std::fmt::Debug;
use crate::std::mem;
#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec, vec::Vec};
use hashbrown::HashSet;

impl<'a, S, T, E, R> AssertIteratorContains<'a, Vec<T>, E, R> for Spec<'a, S, R>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn contains(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterContains { expected })
    }
}

impl<T, E> Expectation<Vec<T>> for IterContains<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
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
        self.mapping(Vec::from_iter)
            .expecting(IterContainsExactlyInAnyOrder::new(Vec::from_iter(expected)))
    }

    fn contains_any_of(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter).expecting(IterContainsAnyOf {
            expected: Vec::from_iter(expected),
        })
    }

    fn contains_all_of(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterContainsAllOf::new(Vec::from_iter(expected)))
    }

    fn contains_only(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterContainsOnly::new(Vec::from_iter(expected)))
    }

    fn contains_only_once(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterContainsOnlyOnce::new(Vec::from_iter(expected)))
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsExactlyInAnyOrder<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let missing = &mut self.missing;
        let extra = &mut self.extra;
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
        let missing = collect_values(&self.missing, &self.expected);
        let extra = collect_values(&self.extra, actual);

        format!(
            r"expected {expression} contains exactly in any order {:?}
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
    fn test(&mut self, subject: &Vec<T>) -> bool {
        for expected in &self.expected {
            if subject.iter().any(|value| value == expected) {
                return true;
            }
        }
        false
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        format!(
            r"expected {expression} contains any of {:?}, but contained none of them
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
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let missing = &mut self.missing;

        for (expected_index, expected) in self.expected.iter().enumerate() {
            if !subject.iter().any(|value| value == expected) {
                missing.insert(expected_index);
            }
        }

        missing.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing, &self.expected);

        format!(
            r"expected {expression} contains all of {:?}
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
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let extra = &mut self.extra;

        for (actual_index, value) in subject.iter().enumerate() {
            if !self.expected.iter().any(|expected| value == expected) {
                extra.insert(actual_index);
            }
        }

        extra.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let extra = collect_values(&self.extra, actual);

        format!(
            r"expected {expression} contains only {:?}
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
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let extra = &mut self.extra;
        let duplicates = &mut self.duplicates;

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
        let extra = collect_values(&self.extra, actual);
        let duplicates = collect_values(&self.duplicates, actual);

        format!(
            r"expected {expression} contains only once {:?}
     but was: {actual:?}
    expected: {:?}
       extra: {extra:?}
  duplicates: {duplicates:?}",
            &self.expected, &self.expected
        )
    }
}

impl<'a, S, T, E, R> AssertIteratorContainsInOrder<'a, Vec<T>, E, R> for Spec<'a, S, R>
where
    S: IntoIterator<Item = T>,
    <S as IntoIterator>::IntoIter: DefinedOrder,
    E: IntoIterator,
    <E as IntoIterator>::IntoIter: DefinedOrder,
    <E as IntoIterator>::Item: Debug,
    T: PartialEq<<E as IntoIterator>::Item> + Debug,
    R: FailingStrategy,
{
    fn contains_exactly(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterContainsExactly::new(Vec::from_iter(expected)))
    }

    fn contains_sequence(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterContainsSequence::new(Vec::from_iter(expected)))
    }

    fn contains_all_in_order(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterContainsAllInOrder::new(Vec::from_iter(expected)))
    }

    fn starts_with(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterStartsWith::new(Vec::from_iter(expected)))
    }

    fn ends_with(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(IterEndsWith::new(Vec::from_iter(expected)))
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsExactly<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let mut maybe_extras = Vec::new();
        let mut maybe_missing = Vec::new();
        let mut expected_iter = self.expected.iter().enumerate();
        let mut subject_iter = subject.iter().enumerate();
        loop {
            match (expected_iter.next(), subject_iter.next()) {
                (Some((expected_index, expected_value)), Some((subject_index, actual_value))) => {
                    if actual_value == expected_value {
                        continue;
                    }
                    maybe_missing.push((expected_index, expected_value));
                    maybe_extras.push((subject_index, actual_value));
                },
                (Some(expected), None) => maybe_missing.push(expected),
                (None, Some(actual)) => maybe_extras.push(actual),
                (None, None) => break,
            }
        }

        let missing = &mut self.missing;
        let extra = &mut self.extra;
        let out_of_order = &mut self.out_of_order;

        for (expected_index, expected_value) in maybe_missing {
            if let Some(index) = maybe_extras
                .iter()
                .position(|(_, value)| *value == expected_value)
            {
                let (subject_index, _) = maybe_extras.remove(index);
                out_of_order.insert(subject_index);
            } else {
                missing.insert(expected_index);
            }
        }
        for (subject_index, _) in maybe_extras {
            extra.insert(subject_index);
        }

        out_of_order.is_empty() && extra.is_empty() && missing.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing, &self.expected);
        let extra = collect_values(&self.extra, actual);
        let out_of_order = collect_values(&self.out_of_order, actual);

        format!(
            r"expected {expression} contains exactly in order {:?}
       but was: {actual:?}
      expected: {:?}
       missing: {missing:?}
         extra: {extra:?}
  out-of-order: {out_of_order:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsSequence<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let subject_length = subject.len();
        let sequence_length = self.expected.len();
        let possible_sequence_starts = if sequence_length >= subject_length {
            vec![0]
        } else {
            (0..=subject_length - sequence_length).collect()
        };
        let best_missing = &mut self.missing;
        let best_extra = &mut self.extra;
        let mut best_match_count = 0;
        let mut missing = HashSet::new();
        let mut extra = HashSet::new();
        let mut match_count = 0;
        for start_index in possible_sequence_starts {
            let mut expected_iter = self.expected.iter().enumerate();
            let mut subject_iter = subject.iter().enumerate().skip(start_index);
            loop {
                match (expected_iter.next(), subject_iter.next()) {
                    (
                        Some((expected_index, expected_value)),
                        Some((subject_index, actual_value)),
                    ) => {
                        if actual_value == expected_value {
                            match_count += 1;
                            continue;
                        }
                        missing.insert(expected_index);
                        extra.insert(subject_index);
                    },
                    (Some((expected_index, _)), None) => {
                        missing.insert(expected_index);
                    },
                    (None, _) => break,
                }
            }
            if missing.is_empty() && extra.is_empty() {
                *best_missing = HashSet::new();
                *best_extra = HashSet::new();
                return true;
            }
            match match_count.cmp(&best_match_count) {
                Ordering::Less => {
                    missing.clear();
                    extra.clear();
                },
                Ordering::Equal => {
                    best_missing.extend(mem::replace(&mut missing, HashSet::new()));
                    best_extra.extend(mem::replace(&mut extra, HashSet::new()));
                },
                Ordering::Greater => {
                    best_match_count = match_count;
                    *best_missing = mem::replace(&mut missing, HashSet::new());
                    *best_extra = mem::replace(&mut extra, HashSet::new());
                },
            }
            match_count = 0;
        }
        false
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing, &self.expected);
        let extra = collect_values(&self.extra, actual);

        format!(
            r"expected {expression} contains sequence {:?}
       but was: {actual:?}
      expected: {:?}
       missing: {missing:?}
         extra: {extra:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterContainsAllInOrder<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let missing = &mut self.missing;
        let mut last_match_index = 0;
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if let Some((subject_index, _)) = subject
                .iter()
                .enumerate()
                .skip(last_match_index)
                .find(|(_, actual)| *actual == expected)
            {
                last_match_index = subject_index + 1;
            } else {
                missing.insert(expected_index);
            }
        }
        missing.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing, &self.expected);

        format!(
            r"expected {expression} contains all of {:?} in order
       but was: {actual:?}
      expected: {:?}
       missing: {missing:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterStartsWith<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let missing = &mut self.missing;
        let extra = &mut self.extra;
        let mut expected_iter = self.expected.iter().enumerate();
        let mut subject_iter = subject.iter().enumerate();
        loop {
            match (expected_iter.next(), subject_iter.next()) {
                (Some((expected_index, expected)), Some((subject_index, actual))) => {
                    if actual == expected {
                        continue;
                    }
                    missing.insert(expected_index);
                    extra.insert(subject_index);
                },
                (Some((expected_index, _)), None) => {
                    missing.insert(expected_index);
                },
                (None, _) => break,
            }
        }
        extra.is_empty() && missing.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing, &self.expected);
        let extra = collect_values(&self.extra, actual);

        format!(
            r"expected {expression} starts with {:?}
   but was: {actual:?}
  expected: {:?}
   missing: {missing:?}
     extra: {extra:?}",
            &self.expected, &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IterEndsWith<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        let missing = &mut self.missing;
        let extra = &mut self.extra;
        let mut expected_iter = self.expected.iter().enumerate().rev();
        let mut subject_iter = subject.iter().enumerate().rev();
        loop {
            match (expected_iter.next(), subject_iter.next()) {
                (Some((expected_index, expected)), Some((subject_index, actual))) => {
                    if actual == expected {
                        continue;
                    }
                    missing.insert(expected_index);
                    extra.insert(subject_index);
                },
                (Some((expected_index, _)), None) => {
                    missing.insert(expected_index);
                },
                (None, _) => break,
            }
        }
        extra.is_empty() && missing.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &Vec<T>) -> String {
        let missing = collect_values(&self.missing, &self.expected);
        let extra = collect_values(&self.extra, actual);

        format!(
            r"expected {expression} ends with {:?}
   but was: {actual:?}
  expected: {:?}
   missing: {missing:?}
     extra: {extra:?}",
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
