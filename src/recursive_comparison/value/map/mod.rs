use super::Value;
use crate::std::borrow::{Borrow, Cow};
use crate::std::cmp::Ordering;
use crate::std::fmt::{self, Debug};
use crate::std::hash::{Hash, Hasher};
use indexmap::IndexMap;

#[derive(Default, Clone)]
pub struct Map(IndexMap<Value, Value>);

impl Map {
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity(capacity))
    }

    pub fn type_name(&self) -> Cow<'static, str> {
        if let Some((key, value)) = self.0.iter().next() {
            let key_type = key.type_name();
            let value_type = value.type_name();
            Cow::Owned(format!("Map<{key_type}, {value_type}>"))
        } else {
            Cow::Borrowed("Map<Value, Value>")
        }
    }

    pub fn insert(&mut self, key: Value, value: Value) -> Option<Value> {
        self.0.insert(key, value)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        Value: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.0.get(key)
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.0.iter(),
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|x| x.hash(state));
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Map {}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.iter().cmp(other.0.iter())
    }
}

impl FromIterator<(Value, Value)> for Map {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Value, Value)>,
    {
        Self(IndexMap::from_iter(iter))
    }
}

impl IntoIterator for Map {
    type Item = (Value, Value);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.0.into_iter(),
        }
    }
}

pub struct IntoIter {
    inner: indexmap::map::IntoIter<Value, Value>,
}

impl Iterator for IntoIter {
    type Item = (Value, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (&'a Value, &'a Value);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a> {
    inner: indexmap::map::Iter<'a, Value, Value>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a Value, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

#[cfg(test)]
mod tests;
