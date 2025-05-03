use crate::assertions::{AssertMapContainsKey, AssertMapContainsValue};
use crate::colored::{
    mark_all_entries_in_map, mark_missing, mark_selected_entries_in_map,
    mark_selected_items_in_collection, mark_unexpected_substr,
};
use crate::expectations::{
    MapContainsKey, MapContainsKeys, MapContainsValue, MapContainsValues, Negatable, Not,
};
use crate::iterator::collect_selected_values;
use crate::properties::MapProperties;
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::format;
use crate::std::string::String;
use crate::std::vec::Vec;
use hashbrown::HashSet;

impl<S, E, R> AssertMapContainsKey<E> for Spec<'_, S, R>
where
    S: MapProperties + Debug,
    <S as MapProperties>::Key: PartialEq<E> + Debug,
    <S as MapProperties>::Value: Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn contains_key(self, expected_key: E) -> Self {
        self.expecting(MapContainsKey { expected_key })
    }

    fn does_not_contain_key(self, expected_key: E) -> Self {
        self.expecting(Not(MapContainsKey { expected_key }))
    }

    fn contains_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self {
        self.expecting(MapContainsKeys::new(expected_keys))
    }
}

impl<M, E> Expectation<M> for MapContainsKey<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: PartialEq<E> + Debug,
    <M as MapProperties>::Value: Debug,
    E: Debug,
{
    fn test(&mut self, subject: &M) -> bool {
        subject.keys_property().any(|k| k == &self.expected_key)
    }

    fn message(&self, expression: Expression<'_>, actual: &M, format: &DiffFormat) -> String {
        let expected_key = &self.expected_key;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let marked_actual =
            mark_all_entries_in_map(&actual_entries, format, mark_unexpected_substr);
        let marked_expected = mark_missing(&self.expected_key, format);

        format!("expected {expression} contains key {expected_key:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<M, E> Negatable<M> for MapContainsKey<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: PartialEq<E> + Debug,
    <M as MapProperties>::Value: Debug,
    E: Debug,
{
    fn negated_message(
        &self,
        expression: Expression<'_>,
        actual: &M,
        format: &DiffFormat,
    ) -> String {
        let expected_key = &self.expected_key;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let found_keys: HashSet<usize> = actual_entries
            .iter()
            .enumerate()
            .filter_map(|(index, (k, _))| {
                if *k == &self.expected_key {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let marked_actual = mark_selected_entries_in_map(
            &actual_entries,
            &found_keys,
            format,
            mark_unexpected_substr,
        );
        let marked_expected = mark_missing(&self.expected_key, format);

        format!("expected {expression} does not contain key {expected_key:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<M, E> Expectation<M> for MapContainsKeys<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: PartialEq<E> + Debug,
    <M as MapProperties>::Value: Debug,
    E: Debug,
{
    fn test(&mut self, subject: &M) -> bool {
        let keys = subject.keys_property().collect::<Vec<_>>();
        let missing_keys = &mut self.missing_keys;
        for (expected_index, expected_key) in self.expected_keys.iter().enumerate() {
            if !keys.iter().any(|k| *k == expected_key) {
                missing_keys.insert(expected_index);
            }
        }
        missing_keys.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &M, format: &DiffFormat) -> String {
        let expected_keys = &self.expected_keys;
        let missing = &self.missing_keys;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let mut extra_entries = HashSet::new();
        for (actual_index, actual_entry) in actual_entries.iter().enumerate() {
            if !expected_keys
                .iter()
                .any(|expected| actual_entry.0 == expected)
            {
                extra_entries.insert(actual_index);
            }
        }
        let marked_actual = mark_selected_entries_in_map(
            &actual_entries,
            &extra_entries,
            format,
            mark_unexpected_substr,
        );
        let marked_expected =
            mark_selected_items_in_collection(expected_keys, missing, format, mark_missing);
        let missing_keys = collect_selected_values(&self.missing_keys, expected_keys);

        format!(
            r"expected {expression} contains keys {expected_keys:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing_keys:?}"
        )
    }
}

impl<S, E, R> AssertMapContainsValue<E> for Spec<'_, S, R>
where
    S: MapProperties,
    <S as MapProperties>::Key: Debug,
    <S as MapProperties>::Value: PartialEq<E> + Debug,
    E: Debug,
    R: FailingStrategy,
{
    fn contains_value(self, expected_value: E) -> Self {
        self.expecting(MapContainsValue { expected_value })
    }

    fn does_not_contain_value(self, expected_value: E) -> Self {
        self.expecting(Not(MapContainsValue { expected_value }))
    }

    fn contains_values(self, expected_values: impl IntoIterator<Item = E>) -> Self {
        self.expecting(MapContainsValues::new(expected_values))
    }
}

impl<M, E> Expectation<M> for MapContainsValue<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: Debug,
    <M as MapProperties>::Value: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &M) -> bool {
        subject.values_property().any(|v| v == &self.expected_value)
    }

    fn message(&self, expression: Expression<'_>, actual: &M, format: &DiffFormat) -> String {
        let expected_value = &self.expected_value;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let marked_actual =
            mark_all_entries_in_map(&actual_entries, format, mark_unexpected_substr);
        let marked_expected = mark_missing(&self.expected_value, format);

        format!("expected {expression} contains value {expected_value:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<M, E> Negatable<M> for MapContainsValue<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: Debug,
    <M as MapProperties>::Value: PartialEq<E> + Debug,
    E: Debug,
{
    fn negated_message(
        &self,
        expression: Expression<'_>,
        actual: &M,
        format: &DiffFormat,
    ) -> String {
        let expected_value = &self.expected_value;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let found_keys: HashSet<usize> = actual_entries
            .iter()
            .enumerate()
            .filter_map(|(index, (_, v))| {
                if *v == &self.expected_value {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let marked_actual = mark_selected_entries_in_map(
            &actual_entries,
            &found_keys,
            format,
            mark_unexpected_substr,
        );
        let marked_expected = mark_missing(&self.expected_value, format);

        format!("expected {expression} does not contain value {expected_value:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<M, E> Expectation<M> for MapContainsValues<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: Debug,
    <M as MapProperties>::Value: PartialEq<E> + Debug,
    E: Debug,
{
    fn test(&mut self, subject: &M) -> bool {
        let values = subject.values_property().collect::<Vec<_>>();
        let missing_values = &mut self.missing_values;
        for (expected_index, expected_value) in self.expected_values.iter().enumerate() {
            if !values.iter().any(|v| *v == expected_value) {
                missing_values.insert(expected_index);
            }
        }
        missing_values.is_empty()
    }

    fn message(&self, expression: Expression<'_>, actual: &M, format: &DiffFormat) -> String {
        let expected_values = &self.expected_values;
        let missing = &self.missing_values;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let mut extra_entries = HashSet::new();
        for (actual_index, actual_entry) in actual_entries.iter().enumerate() {
            if !expected_values
                .iter()
                .any(|expected| actual_entry.1 == expected)
            {
                extra_entries.insert(actual_index);
            }
        }
        let marked_actual = mark_selected_entries_in_map(
            &actual_entries,
            &extra_entries,
            format,
            mark_unexpected_substr,
        );
        let marked_expected =
            mark_selected_items_in_collection(expected_values, missing, format, mark_missing);
        let missing_values = collect_selected_values(&self.missing_values, expected_values);

        format!(
            r"expected {expression} contains values {expected_values:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing_values:?}"
        )
    }
}

mod hashbrown_impls {
    use crate::properties::MapProperties;
    use crate::std::iter::Iterator;
    use hashbrown::HashMap;

    impl<K, V, H> MapProperties for HashMap<K, V, H> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }

    impl<K, V, H> MapProperties for &HashMap<K, V, H> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }

    impl<K, V, H> MapProperties for &mut HashMap<K, V, H> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }
}

#[cfg(feature = "std")]
mod std_hashmap_impls {
    use crate::properties::MapProperties;
    use crate::std::iter::Iterator;
    use std::collections::HashMap;

    impl<K, V, H> MapProperties for HashMap<K, V, H> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }

    impl<K, V, H> MapProperties for &HashMap<K, V, H> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }

    impl<K, V, H> MapProperties for &mut HashMap<K, V, H> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }
}

mod btree_map_impls {
    use crate::properties::MapProperties;
    use crate::std::collections::BTreeMap;
    use crate::std::iter::Iterator;

    impl<K, V> MapProperties for BTreeMap<K, V> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }

    impl<K, V> MapProperties for &BTreeMap<K, V> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }

    impl<K, V> MapProperties for &mut BTreeMap<K, V> {
        type Key = K;
        type Value = V;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as MapProperties>::Key> {
            self.keys()
        }

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }

        fn entries_property(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
            self.iter()
        }
    }
}

#[cfg(test)]
mod tests;
