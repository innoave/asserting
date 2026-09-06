//! Implementations of assertions for `Iterator` values.

use crate::assertions::{
    AssertFilteredElements, AssertIteratorContains, AssertIteratorContainsInAnyOrder,
    AssertIteratorContainsInOrder, AssertOrderedElements, AssertOrderedElementsRef,
};
use crate::colored::{
    mark_all_items_in_collection, mark_missing, mark_selected_items_in_collection, mark_unexpected,
};
use crate::derived_spec::DerivedSpec;
use crate::expectations::{
    AllSatisfy, AnySatisfies, HasAtLeastNumberOfElements, HasSingleElement, IteratorContains,
    IteratorContainsAllInOrder, IteratorContainsAllOf, IteratorContainsAnyOf,
    IteratorContainsExactly, IteratorContainsExactlyInAnyOrder, IteratorContainsOnly,
    IteratorContainsOnlyOnce, IteratorContainsSequence, IteratorEndsWith, IteratorStartsWith,
    NoneSatisfies, all_satisfy, any_satisfies, has_at_least_number_of_elements, has_single_element,
    iterator_contains, iterator_contains_all_in_order, iterator_contains_all_of,
    iterator_contains_any_of, iterator_contains_exactly, iterator_contains_exactly_in_any_order,
    iterator_contains_only, iterator_contains_only_once, iterator_contains_sequence,
    iterator_ends_with, iterator_starts_with, none_satisfies, not,
};
use crate::spec::{
    DiffFormat, DisplayRepresentation, Expectation, Expecting, Expression, FailingStrategy,
    GetFailures, Invertible, PanicOnFail, Represent, Represented, RepresentedBy, Spec,
};
use crate::std::borrow::ToOwned;
use crate::std::cmp::Ordering;
use crate::std::mem;
use crate::std::{format, string::String, vec, vec::Vec};
use hashbrown::HashSet;

impl<'a, S, T, E, D, R> AssertIteratorContains<E> for Spec<'a, S, D, R>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<E>,
    D: Represent<T> + Represent<E> + Clone,
    R: FailingStrategy,
{
    type Sequence = Spec<'a, Vec<T>, D, R>;

    fn contains(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains(expected))
    }

    fn does_not_contain(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(not(iterator_contains(expected)))
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContains<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        subject.iter().any(|e| e == &self.expected)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        inverted: bool,
        representation: &D,
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
                representation,
                format,
                mark_unexpected,
            );
            ("not ", marked_actual)
        } else {
            let marked_actual =
                mark_all_items_in_collection(actual, representation, format, mark_unexpected);
            ("", marked_actual)
        };
        let marked_expected = mark_missing(&self.expected, representation, format);
        let represented_expected = Represented::from((&self.expected, representation));
        format!(
            "expected {expression} to {not}contain {represented_expected:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}",
        )
    }
}

impl<E> Invertible for IteratorContains<E> {}

impl<'a, S, T, E, D, R> AssertIteratorContainsInAnyOrder<E> for Spec<'a, S, D, R>
where
    S: IntoIterator<Item = T>,
    T: PartialEq<<E as IntoIterator>::Item>,
    E: IntoIterator,
    D: Represent<T> + Represent<<E as IntoIterator>::Item> + Clone,
    R: FailingStrategy,
{
    type Sequence = Spec<'a, Vec<T>, D, R>;

    fn contains_exactly_in_any_order(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_exactly_in_any_order(expected))
    }

    fn contains_any_of(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_any_of(expected))
    }

    fn does_not_contain_any_of(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(not(iterator_contains_any_of(expected)))
    }

    fn contains_all_of(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_all_of(expected))
    }

    fn contains_only(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_only(expected))
    }

    fn contains_only_once(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_only_once(expected))
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsExactlyInAnyOrder<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let missing = collect_selected_values(&self.missing, &self.expected, representation);
        let extra = collect_selected_values(&self.extra, actual, representation);
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &self.extra,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &self.missing,
            representation,
            format,
            mark_missing,
        );
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain exactly in any order {represented_expected:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
        )
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsAnyOf<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let (not, marked_actual, marked_expected) = if inverted {
            let mut found_in_actual = HashSet::new();
            let mut found_in_expected = HashSet::new();
            for (exp_idx, expected_item) in self.expected.iter().enumerate() {
                let found = actual
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, elem)| {
                        if elem == expected_item {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if !found.is_empty() {
                    found_in_actual.extend(found);
                    found_in_expected.insert(exp_idx);
                }
            }
            let marked_actual = mark_selected_items_in_collection(
                actual,
                &found_in_actual,
                representation,
                format,
                mark_unexpected,
            );
            let marked_expected = mark_selected_items_in_collection(
                &self.expected,
                &found_in_expected,
                representation,
                format,
                mark_missing,
            );
            ("not ", marked_actual, marked_expected)
        } else {
            let marked_actual =
                mark_all_items_in_collection(actual, representation, format, mark_unexpected);
            let marked_expected =
                mark_all_items_in_collection(&self.expected, representation, format, mark_missing);
            ("", marked_actual, marked_expected)
        };
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();
        format!(
            r"expected {expression} to {not}contain any of {represented_expected:?}
   but was: {marked_actual}
  expected: {not}{marked_expected}",
        )
    }
}

impl<E> Invertible for IteratorContainsAnyOf<E> {}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsAllOf<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let mut extra = HashSet::new();
        for (actual_index, actual) in actual.iter().enumerate() {
            if !self.expected.iter().any(|expected| actual == expected) {
                extra.insert(actual_index);
            }
        }
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &extra,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &self.missing,
            representation,
            format,
            mark_missing,
        );
        let missing = collect_selected_values(&self.missing, &self.expected, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain all of {represented_expected:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}",
        )
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsOnly<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let mut missing = HashSet::new();
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if !actual.iter().any(|value| value == expected) {
                missing.insert(expected_index);
            }
        }
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &self.extra,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &missing,
            representation,
            format,
            mark_missing,
        );
        let extra = collect_selected_values(&self.extra, actual, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain only {represented_expected:?}
   but was: {marked_actual}
  expected: {marked_expected}
     extra: {extra:?}",
        )
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsOnlyOnce<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let actual_duplicates_and_extras = self.duplicates.union(&self.extra).copied().collect();
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &actual_duplicates_and_extras,
            representation,
            format,
            mark_unexpected,
        );
        let duplicates = collect_selected_values(&self.duplicates, actual, representation);
        let mut expected_duplicates_and_missing = HashSet::new();
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if duplicates
                .iter()
                .any(|duplicate| duplicate.value == expected)
                || !actual.iter().any(|actual| actual == expected)
            {
                expected_duplicates_and_missing.insert(expected_index);
            }
        }
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &expected_duplicates_and_missing,
            representation,
            format,
            mark_missing,
        );
        let extra = collect_selected_values(&self.extra, actual, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain only once {represented_expected:?}
     but was: {marked_actual}
    expected: {marked_expected}
       extra: {extra:?}
  duplicates: {duplicates:?}",
        )
    }
}

impl<'a, S, T, E, D, R> AssertIteratorContainsInOrder<E> for Spec<'a, S, D, R>
where
    S: IntoIterator<Item = T>,
    E: IntoIterator,
    T: PartialEq<<E as IntoIterator>::Item>,
    D: Represent<T> + Represent<<E as IntoIterator>::Item> + Clone,
    R: FailingStrategy,
{
    type Sequence = Spec<'a, Vec<T>, D, R>;

    fn contains_exactly(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_exactly(expected))
    }

    fn contains_sequence(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_sequence(expected))
    }

    fn contains_all_in_order(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_contains_all_in_order(expected))
    }

    fn starts_with(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_starts_with(expected))
    }

    fn ends_with(self, expected: E) -> Self::Sequence {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(iterator_ends_with(expected))
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsExactly<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let out_of_order = collect_selected_values(&self.out_of_order, actual, representation);
        let mut expected_indices = self.missing.clone();
        for (expected_index, expected) in self.expected.iter().enumerate() {
            if out_of_order.iter().any(|actual| actual.value == expected) {
                expected_indices.insert(expected_index);
            }
        }
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &expected_indices,
            representation,
            format,
            mark_missing,
        );
        let actual_indices = self.extra.union(&self.out_of_order).copied().collect();
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &actual_indices,
            representation,
            format,
            mark_unexpected,
        );

        let missing = collect_selected_values(&self.missing, &self.expected, representation);
        let extra = collect_selected_values(&self.extra, actual, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain exactly in order {represented_expected:?}
       but was: {marked_actual}
      expected: {marked_expected}
       missing: {missing:?}
         extra: {extra:?}
  out-of-order: {out_of_order:?}",
        )
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsSequence<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &self.extra,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &self.missing,
            representation,
            format,
            mark_missing,
        );
        let missing = collect_selected_values(&self.missing, &self.expected, representation);
        let extra = collect_selected_values(&self.extra, actual, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain the sequence {represented_expected:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
        )
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorContainsAllInOrder<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &self.missing,
            representation,
            format,
            mark_missing,
        );
        let missing = collect_selected_values(&self.missing, &self.expected, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();
        let represented_actual = actual
            .iter()
            .map(|s| Represented::from((s, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain all of {represented_expected:?} in order
   but was: {represented_actual:?}
  expected: {marked_expected}
   missing: {missing:?}",
        )
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorStartsWith<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &self.extra,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &self.missing,
            representation,
            format,
            mark_missing,
        );
        let missing = collect_selected_values(&self.missing, &self.expected, representation);
        let extra = collect_selected_values(&self.extra, actual, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to start with {represented_expected:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
        )
    }
}

impl<T, E, D> Expectation<Vec<T>, D> for IteratorEndsWith<E>
where
    T: PartialEq<E>,
    D: Represent<T> + Represent<E>,
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
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &self.extra,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            &self.expected,
            &self.missing,
            representation,
            format,
            mark_missing,
        );
        let missing = collect_selected_values(&self.missing, &self.expected, representation);
        let extra = collect_selected_values(&self.extra, actual, representation);
        let represented_expected = self
            .expected
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to end with {represented_expected:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing:?}
     extra: {extra:?}",
        )
    }
}

impl<'a, S, T, D, R> AssertFilteredElements<T> for Spec<'a, S, D, R>
where
    S: IntoIterator<Item = T>,
    D: Represent<T> + Clone,
    R: FailingStrategy,
{
    type SingleElement = Spec<'a, T, D, R>;
    type MultipleElements = Spec<'a, Vec<T>, D, R>;

    fn single_element(self) -> Self::SingleElement {
        let representation = self.representation().clone();
        let spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone())
            .expecting(has_single_element());
        if spec.has_failures() {
            PanicOnFail.do_fail_with(&spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let original_expression = spec.expression();
        let new_expression = format!("{original_expression}'s only element");
        spec.extracting("[0]", |mut collection| {
            collection.pop().unwrap_or_else(|| {
                unreachable!("Assertion failed and should have panicked! Please report a bug.")
            })
        })
        .named(new_expression)
        .represented_by(representation)
    }

    fn filtered_on<C>(self, condition: C) -> Self::MultipleElements
    where
        C: FnMut(&T) -> bool,
    {
        let representation = self.representation().clone();
        self.mapping(|subject| subject.into_iter().filter(condition).collect())
            .represented_by(representation)
    }

    fn any_satisfies<P>(self, predicate: P) -> Self::MultipleElements
    where
        P: FnMut(&T) -> bool,
    {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(any_satisfies(predicate))
    }

    fn all_satisfy<P>(self, predicate: P) -> Self::MultipleElements
    where
        P: FnMut(&T) -> bool,
    {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(all_satisfy(predicate))
    }

    fn none_satisfies<P>(self, predicate: P) -> Self::MultipleElements
    where
        P: FnMut(&T) -> bool,
    {
        let representation = self.representation().clone();
        self.mapping(Vec::from_iter)
            .represented_by(representation)
            .expecting(none_satisfies(predicate))
    }
}

impl<T, D> Expectation<Vec<T>, D> for HasSingleElement
where
    D: Represent<T>,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        subject.len() == 1
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let actual_length = actual.len();
        let actual_elements = match actual_length {
            0 => mark_unexpected("no elements", &DisplayRepresentation, format),
            1 => mark_unexpected("exactly one element", &DisplayRepresentation, format),
            _ => mark_unexpected(
                &format!("{actual_length} elements"),
                &DisplayRepresentation,
                format,
            ),
        };
        let expected_elements = mark_missing("exactly one element", &DisplayRepresentation, format);
        let represented_actual = actual
            .iter()
            .map(|s| Represented::from((s, representation)))
            .collect::<Vec<_>>();
        format!(
            r"expected {expression} to have {expected_elements}, but has {actual_elements}
  actual: {represented_actual:?}"
        )
    }
}

impl<T, P, D> Expectation<Vec<T>, D> for AnySatisfies<P>
where
    P: FnMut(&T) -> bool,
    D: Represent<T>,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        subject.iter().any(|e| (self.predicate)(e))
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        representation: &D,
        _format: &DiffFormat,
    ) -> String {
        let represented_actual = actual
            .iter()
            .map(|s| Represented::from((s, representation)))
            .collect::<Vec<_>>();
        format!(
            r"expected any element of {expression} to satisfy the predicate, but none did
  actual: {represented_actual:?}"
        )
    }
}

impl<T, P, D> Expectation<Vec<T>, D> for AllSatisfy<P>
where
    P: FnMut(&T) -> bool,
    D: Represent<T>,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        for (i, e) in subject.iter().enumerate() {
            if !(self.predicate)(e) {
                self.failing.insert(i);
            }
        }
        self.failing.is_empty()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let number_of_failing = self.failing.len();
        let failing = collect_selected_values(&self.failing, actual, representation);
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &self.failing,
            representation,
            format,
            mark_unexpected,
        );
        format!(
            r"expected all elements of {expression} to satisfy the predicate, but {number_of_failing} did not
   actual: {marked_actual}
  failing: {failing:?}"
        )
    }
}

impl<T, P, D> Expectation<Vec<T>, D> for NoneSatisfies<P>
where
    P: FnMut(&T) -> bool,
    D: Represent<T>,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        for (i, e) in subject.iter().enumerate() {
            if (self.predicate)(e) {
                self.failing.insert(i);
            }
        }
        self.failing.is_empty()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let number_of_failing = self.failing.len();
        let failing = collect_selected_values(&self.failing, actual, representation);
        let marked_actual = mark_selected_items_in_collection(
            actual,
            &self.failing,
            representation,
            format,
            mark_unexpected,
        );
        format!(
            r"expected none of the elements of {expression} to satisfy the predicate, but {number_of_failing} did
   actual: {marked_actual}
  failing: {failing:?}"
        )
    }
}

impl<'a, S, T, D, R> AssertOrderedElements for Spec<'a, S, D, R>
where
    S: IntoIterator<Item = T>,
    D: Represent<T> + Clone,
    R: FailingStrategy,
{
    type SingleElement = Spec<'a, T, D, R>;
    type MultipleElements = Spec<'a, Vec<T>, D, R>;

    fn first_element(self) -> Self::SingleElement {
        let representation = self.representation().clone();
        let spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone())
            .expecting(has_at_least_number_of_elements(1));
        if spec.has_failures() {
            PanicOnFail.do_fail_with(&spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = spec.expression();
        let new_subject_name = format!("the first element of {orig_subject_name}");
        spec.extracting("[first]", |mut collection| collection.remove(0))
            .named(new_subject_name)
            .represented_by(representation)
    }

    fn last_element(self) -> Self::SingleElement {
        let representation = self.representation().clone();
        let spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone())
            .expecting(has_at_least_number_of_elements(1));
        if spec.has_failures() {
            PanicOnFail.do_fail_with(&spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = spec.expression();
        let new_subject_name = format!("the last element of {orig_subject_name}");
        spec.extracting("[last]", |mut collection| {
            collection.pop().unwrap_or_else(|| {
                unreachable!("Assertion failed and should have panicked! Please report a bug.")
            })
        })
        .named(new_subject_name)
        .represented_by(representation)
    }

    fn nth_element(self, n: usize) -> Self::SingleElement {
        let representation = self.representation().clone();
        let min_len = n + 1;
        let spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone())
            .expecting(has_at_least_number_of_elements(min_len));
        if spec.has_failures() {
            PanicOnFail.do_fail_with(&spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = spec.expression();
        let new_subject_name = format!("{orig_subject_name}[{n}]");
        spec.extracting("[nth]", |mut collection| collection.remove(n))
            .named(new_subject_name)
            .represented_by(representation)
    }

    fn elements_at(self, indices: impl IntoIterator<Item = usize>) -> Self::MultipleElements {
        let representation = self.representation().clone();
        let indices = Vec::from_iter(indices);
        let orig_subject_name = self.expression();
        let new_subject_name = format!("{orig_subject_name} at positions {indices:?}");
        let indices = HashSet::<_>::from_iter(indices);
        self.mapping(|subject| {
            subject
                .into_iter()
                .enumerate()
                .filter_map(|(i, v)| if indices.contains(&i) { Some(v) } else { None })
                .collect()
        })
        .named(new_subject_name)
        .represented_by(representation)
    }
}

impl<'a, S, T, U, D, R> AssertOrderedElementsRef for Spec<'a, S, D, R>
where
    S: IntoIterator<Item = T>,
    T: 'a + ToOwned<Owned = U>,
    U: Clone,
    D: Represent<T> + Clone,
    R: FailingStrategy,
{
    type SingleElement = DerivedSpec<'a, Spec<'a, Vec<T>, D, R>, U, D>;
    type MultipleElements = DerivedSpec<'a, Spec<'a, Vec<T>, D, R>, Vec<U>, D>;

    fn first_element_ref(self) -> Self::SingleElement {
        let representation = self.representation().clone();
        let original_spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone())
            .expecting(has_at_least_number_of_elements(1));
        if original_spec.has_failures() {
            PanicOnFail.do_fail_with(&original_spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = original_spec.expression();
        let new_subject_name = format!("the first element of {orig_subject_name}");
        original_spec.extracting_ref("[first]", |collection|
            collection.first()
                .unwrap_or_else(||
                    unreachable!("We should have asserted before, that there is at least one element in the collection/iterator. Please file a bug.")
                )
        ).named(new_subject_name).represented_by(representation)
    }

    fn last_element_ref(self) -> Self::SingleElement {
        let representation = self.representation().clone();
        let original_spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone())
            .expecting(has_at_least_number_of_elements(1));
        if original_spec.has_failures() {
            PanicOnFail.do_fail_with(&original_spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = original_spec.expression();
        let new_subject_name = format!("the last element of {orig_subject_name}");
        original_spec.extracting_ref("[last]", |collection|
            collection.last()
                .unwrap_or_else(||
                    unreachable!("We should have asserted before, that there is at least one element in the collection/iterator. Please file a bug.")
                )
        ).named(new_subject_name).represented_by(representation)
    }

    fn nth_element_ref(self, n: usize) -> Self::SingleElement {
        let representation = self.representation().clone();
        let min_len = n + 1;
        let original_spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone())
            .expecting(has_at_least_number_of_elements(min_len));
        if original_spec.has_failures() {
            PanicOnFail.do_fail_with(&original_spec.failures());
            unreachable!("Assertion failed and should have panicked! Please report a bug.")
        }
        let orig_subject_name = original_spec.expression();
        let new_subject_name = format!("{orig_subject_name}[{n}]");
        original_spec.extracting_ref("[nth]", |collection|
            collection.get(n)
                .unwrap_or_else(||
                    unreachable!("We should have asserted before, that there is at least one element in the collection/iterator. Please file a bug.")
                )
        ).named(new_subject_name).represented_by(representation)
    }

    fn elements_ref_at(self, indices: impl IntoIterator<Item = usize>) -> Self::MultipleElements {
        let representation = self.representation().clone();
        let indices = Vec::from_iter(indices);
        let orig_subject_name = self.expression();
        let new_subject_name = format!("{orig_subject_name} at positions {indices:?}");
        let indices = HashSet::<_>::from_iter(indices);
        let original_spec = self
            .mapping(Vec::from_iter)
            .represented_by(representation.clone());
        original_spec
            .extracting_ref_iter("", |collection| {
                collection
                    .enumerate()
                    .filter_map(|(i, e)| {
                        if indices.contains(&i) {
                            Some(e.to_owned())
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .named(new_subject_name)
            .represented_by(representation)
    }
}

impl<T, D> Expectation<Vec<T>, D> for HasAtLeastNumberOfElements
where
    D: Represent<T>,
{
    fn test(&mut self, subject: &Vec<T>) -> bool {
        subject.len() >= self.expected_number_of_elements
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Vec<T>,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let actual_length = actual.len();
        let actual_elements = match actual_length {
            0 => mark_unexpected("no elements", &DisplayRepresentation, format),
            1 => mark_unexpected("one element", &DisplayRepresentation, format),
            _ => mark_unexpected(
                &format!("{actual_length} elements"),
                &DisplayRepresentation,
                format,
            ),
        };
        let expected_elements = match self.expected_number_of_elements {
            0 => mark_missing("no elements", &DisplayRepresentation, format),
            1 => mark_missing("at least one element", &DisplayRepresentation, format),
            _ => mark_missing(
                &format!("at least {} elements", self.expected_number_of_elements),
                &DisplayRepresentation,
                format,
            ),
        };
        let represented_actual = actual
            .iter()
            .map(|s| Represented::from((s, representation)))
            .collect::<Vec<_>>();
        format!(
            r"expected {expression} to have {expected_elements}, but has {actual_elements}
  actual: {represented_actual:?}"
        )
    }
}

pub fn collect_selected_values<'t, 'd, T, D>(
    indices: &HashSet<usize>,
    collection: &'t [T],
    representation: &'d D,
) -> Vec<Represented<'t, 'd, T, D>> {
    collection
        .iter()
        .enumerate()
        .filter_map(|(idx, value)| {
            if indices.contains(&idx) {
                Some(Represented::from((value, representation)))
            } else {
                None
            }
        })
        .collect()
}

pub fn collect_selected_ref_values<'t, 'd, T, D>(
    indices: &HashSet<usize>,
    collection: &[&'t T],
    representation: &'d D,
) -> Vec<Represented<'t, 'd, T, D>> {
    collection
        .iter()
        .enumerate()
        .filter_map(|(idx, value)| {
            if indices.contains(&idx) {
                Some(Represented::from((*value, representation)))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
