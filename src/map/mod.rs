use crate::assertions::{AssertMapContainsKey, AssertMapContainsValue};
use crate::colored::{mark_missing, mark_unexpected};
use crate::expectations::{MapContainsKey, MapContainsValue, Negatable, Not};
use crate::properties::{KeysProperty, ValuesProperty};
use crate::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};
use crate::std::fmt::Debug;
use crate::std::format;
use crate::std::string::String;

impl<S, E, R> AssertMapContainsKey<E> for Spec<'_, S, R>
where
    S: KeysProperty + Debug,
    <S as KeysProperty>::Key: PartialEq<E>,
    E: Debug,
    R: FailingStrategy,
{
    fn contains_key(self, expected_key: E) -> Self {
        self.expecting(MapContainsKey { expected_key })
    }

    fn does_not_contain_key(self, expected_key: E) -> Self {
        self.expecting(Not(MapContainsKey { expected_key }))
    }
}

impl<M, E> Expectation<M> for MapContainsKey<E>
where
    M: KeysProperty + Debug,
    <M as KeysProperty>::Key: PartialEq<E>,
    E: Debug,
{
    fn test(&mut self, subject: &M) -> bool {
        subject.keys_property().any(|k| k == &self.expected_key)
    }

    fn message(&self, expression: Expression<'_>, actual: &M, format: &DiffFormat) -> String {
        let expected_key = &self.expected_key;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected_key, format);

        format!("expected {expression} contains key {expected_key:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S, E> Negatable<S> for MapContainsKey<E>
where
    S: Debug,
    E: Debug,
{
    fn negated_message(
        &self,
        expression: Expression<'_>,
        actual: &S,
        format: &DiffFormat,
    ) -> String {
        let expected_key = &self.expected_key;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected_key, format);

        format!("expected {expression} does not contain key {expected_key:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S, E, R> AssertMapContainsValue<E> for Spec<'_, S, R>
where
    S: ValuesProperty + Debug,
    <S as ValuesProperty>::Value: PartialEq<E>,
    E: Debug,
    R: FailingStrategy,
{
    fn contains_value(self, expected_value: E) -> Self {
        self.expecting(MapContainsValue { expected_value })
    }

    fn does_not_contain_value(self, expected_value: E) -> Self {
        self.expecting(Not(MapContainsValue { expected_value }))
    }
}

impl<M, E> Expectation<M> for MapContainsValue<E>
where
    M: ValuesProperty + Debug,
    <M as ValuesProperty>::Value: PartialEq<E>,
    E: Debug,
{
    fn test(&mut self, subject: &M) -> bool {
        subject.values_property().any(|v| v == &self.expected_value)
    }

    fn message(&self, expression: Expression<'_>, actual: &M, format: &DiffFormat) -> String {
        let expected_value = &self.expected_value;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected_value, format);

        format!("expected {expression} contains value {expected_value:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

impl<S, E> Negatable<S> for MapContainsValue<E>
where
    S: Debug,
    E: Debug,
{
    fn negated_message(
        &self,
        expression: Expression<'_>,
        actual: &S,
        format: &DiffFormat,
    ) -> String {
        let expected_value = &self.expected_value;
        let marked_actual = mark_unexpected(actual, format);
        let marked_expected = mark_missing(&self.expected_value, format);

        format!("expected {expression} does not contain value {expected_value:?}\n   but was: {marked_actual}\n  expected: {marked_expected}")
    }
}

mod hashbrown_impls {
    use crate::properties::{KeysProperty, ValuesProperty};
    use crate::std::iter::Iterator;
    use hashbrown::HashMap;

    impl<K, V, H> KeysProperty for HashMap<K, V, H> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as KeysProperty>::Key> {
            self.keys()
        }
    }

    impl<K, V, H> KeysProperty for &HashMap<K, V, H> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &Self::Key> {
            self.keys()
        }
    }

    impl<K, V, H> KeysProperty for &mut HashMap<K, V, H> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &Self::Key> {
            self.keys()
        }
    }

    impl<K, V, H> ValuesProperty for HashMap<K, V, H> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }

    impl<K, V, H> ValuesProperty for &HashMap<K, V, H> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }

    impl<K, V, H> ValuesProperty for &mut HashMap<K, V, H> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }
}

#[cfg(feature = "std")]
mod std_hashmap_impls {
    use crate::properties::{KeysProperty, ValuesProperty};
    use crate::std::iter::Iterator;
    use std::collections::HashMap;

    impl<K, V, H> KeysProperty for HashMap<K, V, H> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &<Self as KeysProperty>::Key> {
            self.keys()
        }
    }

    impl<K, V, H> KeysProperty for &HashMap<K, V, H> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &Self::Key> {
            self.keys()
        }
    }

    impl<K, V, H> KeysProperty for &mut HashMap<K, V, H> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &Self::Key> {
            self.keys()
        }
    }

    impl<K, V, H> ValuesProperty for HashMap<K, V, H> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }

    impl<K, V, H> ValuesProperty for &HashMap<K, V, H> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }

    impl<K, V, H> ValuesProperty for &mut HashMap<K, V, H> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }
}

mod btree_map_impls {
    use crate::properties::{KeysProperty, ValuesProperty};
    use crate::std::collections::BTreeMap;
    use crate::std::iter::Iterator;

    impl<K, V> KeysProperty for BTreeMap<K, V> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &Self::Key> {
            self.keys()
        }
    }

    impl<K, V> KeysProperty for &BTreeMap<K, V> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &Self::Key> {
            self.keys()
        }
    }

    impl<K, V> KeysProperty for &mut BTreeMap<K, V> {
        type Key = K;

        fn keys_property(&self) -> impl Iterator<Item = &Self::Key> {
            self.keys()
        }
    }

    impl<K, V> ValuesProperty for BTreeMap<K, V> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }

    impl<K, V> ValuesProperty for &BTreeMap<K, V> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }

    impl<K, V> ValuesProperty for &mut BTreeMap<K, V> {
        type Value = V;

        fn values_property(&self) -> impl Iterator<Item = &Self::Value> {
            self.values()
        }
    }
}

#[cfg(test)]
mod tests;
