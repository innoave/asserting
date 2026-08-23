use crate::assertions::{AssertMapContainsKey, AssertMapContainsValue};
use crate::colored::{
    mark_all_entries_in_map, mark_missing, mark_selected_entries_in_map,
    mark_selected_items_in_collection, mark_unexpected,
};
use crate::expectations::{
    MapContainsExactlyKeys, MapContainsKey, MapContainsKeys, MapContainsValue, MapContainsValues,
    MapDoesNotContainKeys, MapDoesNotContainValues, map_contains_exactly_keys, map_contains_key,
    map_contains_keys, map_contains_value, map_contains_values, map_does_not_contain_keys,
    map_does_not_contain_values, not,
};
use crate::iterator::{collect_selected_ref_values, collect_selected_values};
use crate::properties::MapProperties;
use crate::spec::{
    DiffFormat, Expectation, Expecting, Expression, FailingStrategy, Invertible, Represent,
    Represented, Spec,
};
use crate::std::format;
use crate::std::string::String;
use crate::std::vec::Vec;
use hashbrown::HashSet;

impl<S, E, D, R> AssertMapContainsKey<E> for Spec<'_, S, D, R>
where
    S: MapProperties,
    <S as MapProperties>::Key: PartialEq<E>,
    D: Represent<S>
        + Represent<E>
        + Represent<<S as MapProperties>::Key>
        + Represent<<S as MapProperties>::Value>,
    R: FailingStrategy,
{
    fn contains_key(self, expected_key: E) -> Self {
        self.expecting(map_contains_key(expected_key))
    }

    fn does_not_contain_key(self, expected_key: E) -> Self {
        self.expecting(not(map_contains_key(expected_key)))
    }

    fn contains_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_contains_keys(expected_keys))
    }

    fn does_not_contain_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_does_not_contain_keys(expected_keys))
    }

    fn contains_exactly_keys(self, expected_keys: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_contains_exactly_keys(expected_keys))
    }
}

impl<M, E, D> Expectation<M, D> for MapContainsKey<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: PartialEq<E>,
    D: Represent<E> + Represent<<M as MapProperties>::Key> + Represent<<M as MapProperties>::Value>,
{
    fn test(&mut self, subject: &M) -> bool {
        subject.keys_property().any(|k| k == &self.expected_key)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &M,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let (not, marked_actual) = if inverted {
            let found: HashSet<usize> = actual_entries
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
            let selected_entries_marked = mark_selected_entries_in_map(
                &actual_entries,
                &found,
                representation,
                format,
                mark_unexpected,
            );
            ("not ", selected_entries_marked)
        } else {
            let all_entries_marked =
                mark_all_entries_in_map(&actual_entries, representation, format, mark_unexpected);
            ("", all_entries_marked)
        };
        let marked_expected = mark_missing(&self.expected_key, representation, format);
        let expected_key = Represented::from((&self.expected_key, representation));
        format!(
            "expected {expression} to {not}contain the key {expected_key:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl<E> Invertible for MapContainsKey<E> {}

impl<M, E, D> Expectation<M, D> for MapContainsKeys<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: PartialEq<E>,
    D: Represent<E> + Represent<<M as MapProperties>::Key> + Represent<<M as MapProperties>::Value>,
{
    fn test(&mut self, subject: &M) -> bool {
        let keys = subject.keys_property().collect::<Vec<_>>();
        let missing = &mut self.missing;
        for (expected_index, expected_key) in self.expected_keys.iter().enumerate() {
            if !keys.iter().any(|k| *k == expected_key) {
                missing.insert(expected_index);
            }
        }
        missing.is_empty()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &M,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let expected_keys = &self.expected_keys;
        let missing = &self.missing;
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
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            expected_keys,
            missing,
            representation,
            format,
            mark_missing,
        );
        let missing_keys = collect_selected_values(missing, expected_keys, representation);
        let expected_keys = self
            .expected_keys
            .iter()
            .map(|k| Represented::from((k, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain the keys {expected_keys:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing_keys:?}"
        )
    }
}

impl<M, E, D> Expectation<M, D> for MapDoesNotContainKeys<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: PartialEq<E>,
    D: Represent<E> + Represent<<M as MapProperties>::Key> + Represent<<M as MapProperties>::Value>,
{
    fn test(&mut self, subject: &M) -> bool {
        let keys = subject.keys_property().collect::<Vec<_>>();
        let extra = &mut self.extra;
        for (expected_index, expected_key) in self.expected_keys.iter().enumerate() {
            if keys.iter().any(|k| *k == expected_key) {
                extra.insert(expected_index);
            }
        }
        extra.is_empty()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &M,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let expected_keys = &self.expected_keys;
        let extra = &self.extra;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let actual_keys: Vec<_> = actual.keys_property().collect();
        let mut found = HashSet::new();
        for (actual_index, actual_key) in actual_keys.iter().enumerate() {
            if expected_keys.iter().any(|expected| *actual_key == expected) {
                found.insert(actual_index);
            }
        }
        let marked_actual = mark_selected_entries_in_map(
            &actual_entries,
            &found,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            expected_keys,
            extra,
            representation,
            format,
            mark_missing,
        );
        let extra_keys = collect_selected_ref_values(&found, &actual_keys, representation);
        let expected_keys = self
            .expected_keys
            .iter()
            .map(|k| Represented::from((k, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to not contain the keys {expected_keys:?}
   but was: {marked_actual}
  expected: {marked_expected}
     extra: {extra_keys:?}"
        )
    }
}

impl<M, E, D> Expectation<M, D> for MapContainsExactlyKeys<E>
where
    M: MapProperties,
    <M as MapProperties>::Key: PartialEq<E>,
    D: Represent<E> + Represent<<M as MapProperties>::Key> + Represent<<M as MapProperties>::Value>,
{
    fn test(&mut self, subject: &M) -> bool {
        let actual_keys = subject.keys_property().collect::<Vec<_>>();
        let expected_keys = &self.expected_keys;
        let missing = &mut self.missing;
        let extra = &mut self.extra;
        *extra = (0..actual_keys.len()).collect();
        for (expected_index, expected_key) in expected_keys.iter().enumerate() {
            if let Some(actual_index) = actual_keys.iter().position(|k| *k == expected_key) {
                extra.remove(&actual_index);
            } else {
                missing.insert(expected_index);
            }
        }
        missing.is_empty() && extra.is_empty()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &M,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let expected_keys = &self.expected_keys;
        let missing = &self.missing;
        let extra = &self.extra;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let actual_keys: Vec<_> = actual.keys_property().collect();

        let marked_actual = mark_selected_entries_in_map(
            &actual_entries,
            extra,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            expected_keys,
            missing,
            representation,
            format,
            mark_missing,
        );
        let missing_keys = collect_selected_values(missing, expected_keys, representation);
        let extra_keys = collect_selected_ref_values(extra, &actual_keys, representation);
        let expected_keys = self
            .expected_keys
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain exactly the keys {expected_keys:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing_keys:?}
     extra: {extra_keys:?}"
        )
    }
}

impl<S, E, D, R> AssertMapContainsValue<E> for Spec<'_, S, D, R>
where
    S: MapProperties,
    <S as MapProperties>::Value: PartialEq<E>,
    D: Represent<E> + Represent<<S as MapProperties>::Key> + Represent<<S as MapProperties>::Value>,
    R: FailingStrategy,
{
    fn contains_value(self, expected_value: E) -> Self {
        self.expecting(map_contains_value(expected_value))
    }

    fn does_not_contain_value(self, expected_value: E) -> Self {
        self.expecting(not(map_contains_value(expected_value)))
    }

    fn contains_values(self, expected_values: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_contains_values(expected_values))
    }

    fn does_not_contain_values(self, expected_values: impl IntoIterator<Item = E>) -> Self {
        self.expecting(map_does_not_contain_values(expected_values))
    }
}

impl<M, E, D> Expectation<M, D> for MapContainsValue<E>
where
    M: MapProperties,
    <M as MapProperties>::Value: PartialEq<E>,
    D: Represent<E> + Represent<<M as MapProperties>::Key> + Represent<<M as MapProperties>::Value>,
{
    fn test(&mut self, subject: &M) -> bool {
        subject.values_property().any(|v| v == &self.expected_value)
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &M,
        inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let (not, marked_actual) = if inverted {
            let found: HashSet<usize> = actual_entries
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
            let selected_entries_marked = mark_selected_entries_in_map(
                &actual_entries,
                &found,
                representation,
                format,
                mark_unexpected,
            );
            ("not ", selected_entries_marked)
        } else {
            let all_entries_marked =
                mark_all_entries_in_map(&actual_entries, representation, format, mark_unexpected);
            ("", all_entries_marked)
        };
        let marked_expected = mark_missing(&self.expected_value, representation, format);
        let expected_value = Represented::from((&self.expected_value, representation));

        format!(
            "expected {expression} to {not}contain the value {expected_value:?}\n   but was: {marked_actual}\n  expected: {not}{marked_expected}"
        )
    }
}

impl<E> Invertible for MapContainsValue<E> {}

impl<M, E, D> Expectation<M, D> for MapContainsValues<E>
where
    M: MapProperties,
    <M as MapProperties>::Value: PartialEq<E>,
    D: Represent<E> + Represent<<M as MapProperties>::Key> + Represent<<M as MapProperties>::Value>,
{
    fn test(&mut self, subject: &M) -> bool {
        let values = subject.values_property().collect::<Vec<_>>();
        let missing = &mut self.missing;
        for (expected_index, expected_value) in self.expected_values.iter().enumerate() {
            if !values.iter().any(|v| *v == expected_value) {
                missing.insert(expected_index);
            }
        }
        missing.is_empty()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &M,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let expected_values = &self.expected_values;
        let missing = &self.missing;
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
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            expected_values,
            missing,
            representation,
            format,
            mark_missing,
        );
        let missing_values = collect_selected_values(missing, expected_values, representation);
        let expected_values = self
            .expected_values
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to contain the values {expected_values:?}
   but was: {marked_actual}
  expected: {marked_expected}
   missing: {missing_values:?}"
        )
    }
}

impl<M, E, D> Expectation<M, D> for MapDoesNotContainValues<E>
where
    M: MapProperties,
    <M as MapProperties>::Value: PartialEq<E>,
    D: Represent<E> + Represent<<M as MapProperties>::Key> + Represent<<M as MapProperties>::Value>,
{
    fn test(&mut self, subject: &M) -> bool {
        let values = subject.values_property().collect::<Vec<_>>();
        let extra = &mut self.extra;
        for (expected_index, expected_value) in self.expected_values.iter().enumerate() {
            if values.iter().any(|v| *v == expected_value) {
                extra.insert(expected_index);
            }
        }
        extra.is_empty()
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &M,
        _inverted: bool,
        representation: &D,
        format: &DiffFormat,
    ) -> String {
        let expected_values = &self.expected_values;
        let extra = &self.extra;
        let actual_entries: Vec<_> = actual.entries_property().collect();
        let actual_values: Vec<_> = actual.values_property().collect();
        let mut found = HashSet::new();
        for (actual_index, actual_value) in actual_values.iter().enumerate() {
            if expected_values
                .iter()
                .any(|expected| *actual_value == expected)
            {
                found.insert(actual_index);
            }
        }
        let marked_actual = mark_selected_entries_in_map(
            &actual_entries,
            &found,
            representation,
            format,
            mark_unexpected,
        );
        let marked_expected = mark_selected_items_in_collection(
            expected_values,
            extra,
            representation,
            format,
            mark_missing,
        );
        let extra_values = collect_selected_ref_values(&found, &actual_values, representation);
        let expected_values = self
            .expected_values
            .iter()
            .map(|e| Represented::from((e, representation)))
            .collect::<Vec<_>>();

        format!(
            r"expected {expression} to not contain the values {expected_values:?}
   but was: {marked_actual}
  expected: {marked_expected}
     extra: {extra_values:?}"
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
    use crate::std::collections::HashMap;
    use crate::std::iter::Iterator;

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
