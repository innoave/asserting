//! Implementations of assertions for `Iterator` values.

use crate::assertions::{
    AssertIteratorContains, AssertIteratorContainsInAnyOrder, AssertIteratorContainsInOrder,
};
use crate::colored::{
    mark_all_items_in_collection, mark_missing, mark_selected_items_in_collection, mark_unexpected,
};
use crate::expectations::{
    iterator_contains, iterator_contains_all_in_order, iterator_contains_all_of,
    iterator_contains_any_of, iterator_contains_exactly, iterator_contains_exactly_in_any_order,
    iterator_contains_only, iterator_contains_only_once, iterator_contains_sequence,
    iterator_ends_with, iterator_starts_with, not, IteratorContains, IteratorContainsAllInOrder,
    IteratorContainsAllOf, IteratorContainsAnyOf, IteratorContainsExactly,
    IteratorContainsExactlyInAnyOrder, IteratorContainsOnly, IteratorContainsOnlyOnce,
    IteratorContainsSequence, IteratorEndsWith, IteratorStartsWith,
};
use crate::properties::DefinedOrderProperty;
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Invertible, Spec};
use crate::std::cmp::Ordering;
use crate::std::fmt::Debug;
use crate::std::mem;
use crate::std::{format, string::String, vec, vec::Vec};
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
            .expecting(iterator_contains(expected))
    }

    fn does_not_contain(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(not(iterator_contains(expected)))
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContains<E>
where
    T: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        subject.iter().any(|e| e == &self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual) = if inverted {
            let found_unexpected = actual
                .iter()
                .enumerate()
                .filter_map(|(idx, element)| {
                    if element == &self.expected {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect();
            let marked_actual = mark_selected_items_in_collection(
                actual,
                &found_unexpected,
                format,
                mark_unexpected,
            );
            ("not ", marked_actual)
        } else {
            let marked_actual = mark_all_items_in_collection(actual, format, mark_unexpected);
            ("", marked_actual)
        };
        let marked_expected = mark_missing(&self.expected, format);
        format!(
            "expected {expression} to {not}contain {:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
            &self.expected,
        )
    }
}

impl<E> Invertible for IteratorContains<E> {}

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
            .expecting(iterator_contains_exactly_in_any_order(expected))
    }

    fn contains_any_of(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_any_of(expected))
    }

    fn contains_all_of(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_all_of(expected))
    }

    fn contains_only(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_only(expected))
    }

    fn contains_only_once(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_only_once(expected))
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsExactlyInAnyOrder<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let missing = collect_selected_values(&self.missing, &self.expected);
        let extra = collect_selected_values(&self.extra, actual);
        let marked_actual =
            mark_selected_items_in_collection(actual, &self.extra, format, mark_unexpected);
        let marked_expected =
            mark_selected_items_in_collection(&self.expected, &self.missing, format, mark_missing);

        format!(
            r"expected {expression} to contain exactly in any order {:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
            &self.expected
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsAnyOf<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_all_items_in_collection(actual, format, mark_unexpected);
        let marked_expected = mark_all_items_in_collection(&self.expected, format, mark_missing);
        format!(
            r"expected {expression} to contain any of {:?}
   but was: {marked_actual}
  expected: {marked_expected}",
            &self.expected,
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsAllOf<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let mut extra = HashSet::new();
        for (actual_index, actual) in actual.iter().enumerate() {
            if !self.expected.iter().any(|expected| actual == expected) {
                extra.insert(actual_index);
            }
        }
        let marked_actual =
            mark_selected_items_in_collection(actual, &extra, format, mark_unexpected);
        let marked_expected =
            mark_selected_items_in_collection(&self.expected, &self.missing, format, mark_missing);
        let missing = collect_selected_values(&self.missing, &self.expected);

        format!(
            r"expected {expression} to contain all of {:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}",
            &self.expected,
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsOnly<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let mut missing = HashSet::new();
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if !actual.iter().any(|value| value == expected) {
                missing.insert(expected_index);
            }
        }
        let marked_actual =
            mark_selected_items_in_collection(actual, &self.extra, format, mark_unexpected);
        let marked_expected =
            mark_selected_items_in_collection(&self.expected, &missing, format, mark_missing);
        let extra = collect_selected_values(&self.extra, actual);

        format!(
            r"expected {expression} to contain only {:?}
   but was: {marked_actual}
  expected: {marked_expected}
     extra: {extra:?}",
            &self.expected,
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsOnlyOnce<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let actual_duplicates_and_extras = self.duplicates.union(&self.extra).copied().collect();
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &actual_duplicates_and_extras,
            format,
            mark_unexpected,
        );
        let duplicates = collect_selected_values(&self.duplicates, actual);
        let mut expected_duplicates_and_missing = HashSet::new();
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if duplicates.iter().any(|duplicate| *duplicate == expected)
                || !actual.iter().any(|actual| actual == expected)
            {
                expected_duplicates_and_missing.insert(expected_index);
            }
        }
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &expected_duplicates_and_missing,
            format,
            mark_missing,
        );
        let extra = collect_selected_values(&self.extra, actual);

        format!(
            r"expected {expression} to contain only once {:?}
     but was: {marked_actual}
    expected: {marked_expected}
       extra: {extra:?}
  duplicates: {duplicates:?}",
            &self.expected,
        )
    }
}

impl<'a, S, T, E, R> AssertIteratorContainsInOrder<'a, Vec<T>, E, R> for Spec<'a, S, R>
where
    S: IntoIterator<Item = T>,
    <S as IntoIterator>::IntoIter: DefinedOrderProperty,
    E: IntoIterator,
    <E as IntoIterator>::IntoIter: DefinedOrderProperty,
    <E as IntoIterator>::Item: Debug,
    T: PartialEq<<E as IntoIterator>::Item> + Debug,
    R: FailingStrategy,
{
    fn contains_exactly(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_exactly(expected))
    }

    fn contains_sequence(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_sequence(expected))
    }

    fn contains_all_in_order(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_contains_all_in_order(expected))
    }

    fn starts_with(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_starts_with(expected))
    }

    fn ends_with(self, expected: E) -> Spec<'a, Vec<T>, R> {
        self.mapping(Vec::from_iter)
            .expecting(iterator_ends_with(expected))
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsExactly<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let out_of_order = collect_selected_values(&self.out_of_order, actual);
        let mut expected_indices = self.missing.clone();
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if out_of_order.iter().any(|actual| *actual == expected) {
                expected_indices.insert(expected_index);
            }
        }
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &expected_indices,
            format,
            mark_missing,
        );
        let actual_indices = self.extra.union(&self.out_of_order).copied().collect();
        let marked_actual =
            mark_selected_items_in_collection(actual, &actual_indices, format, mark_unexpected);

        let missing = collect_selected_values(&self.missing, &self.expected);
        let extra = collect_selected_values(&self.extra, actual);

        format!(
            r"expected {expression} to contain exactly in order {:?}
       but was: {marked_actual}
      expected: {marked_expected}
       missing: {missing:?}
         extra: {extra:?}
  out-of-order: {out_of_order:?}",
            &self.expected,
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsSequence<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_actual =
            mark_selected_items_in_collection(actual, &self.extra, format, mark_unexpected);
        let marked_expected =
            mark_selected_items_in_collection(&self.expected, &self.missing, format, mark_missing);
        let missing = collect_selected_values(&self.missing, &self.expected);
        let extra = collect_selected_values(&self.extra, actual);

        format!(
            r"expected {expression} to contain the sequence {:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
            &self.expected,
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorContainsAllInOrder<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_expected =
            mark_selected_items_in_collection(&self.expected, &self.missing, format, mark_missing);
        let missing = collect_selected_values(&self.missing, &self.expected);

        format!(
            r"expected {expression} to contain all of {:?} in order
   but was: {actual:?}
  expected: {marked_expected}
   missing: {missing:?}",
            &self.expected,
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorStartsWith<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_actual =
            mark_selected_items_in_collection(actual, &self.extra, format, mark_unexpected);
        let marked_expected =
            mark_selected_items_in_collection(&self.expected, &self.missing, format, mark_missing);
        let missing = collect_selected_values(&self.missing, &self.expected);
        let extra = collect_selected_values(&self.extra, actual);

        format!(
            r"expected {expression} to start with {:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
            &self.expected,
        )
    }
}

impl<T, E> Expectation<Vec<T>> for IteratorEndsWith<E>
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

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let marked_actual =
            mark_selected_items_in_collection(actual, &self.extra, format, mark_unexpected);
        let marked_expected =
            mark_selected_items_in_collection(&self.expected, &self.missing, format, mark_missing);
        let missing = collect_selected_values(&self.missing, &self.expected);
        let extra = collect_selected_values(&self.extra, actual);

        format!(
            r"expected {expression} to end with {:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
            &self.expected,
        )
    }
}

pub fn collect_selected_values<'a, T>(indices: &HashSet<usize>, collection: &'a [T]) -> Vec<&'a T> {
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
