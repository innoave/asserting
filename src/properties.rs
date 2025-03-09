use crate::std::array;
use crate::std::slice;

pub trait IsEmptyProperty {
    fn is_empty_property(&self) -> bool;
}

pub trait LengthProperty {
    fn length_property(&self) -> usize;
}

/// Marker trait to specify whether a collection or iterator iterates over its
/// elements in a well-defined order.
pub trait DefinedOrder {}

impl<C> DefinedOrder for &C where C: DefinedOrder + ?Sized {}
impl<C> DefinedOrder for &mut C where C: DefinedOrder + ?Sized {}

impl<T> DefinedOrder for [T] {}
impl<T, const N: usize> DefinedOrder for [T; N] {}
impl<T, const N: usize> DefinedOrder for array::IntoIter<T, N> {}
impl<T> DefinedOrder for slice::Iter<'_, T> {}
impl<T> DefinedOrder for slice::IterMut<'_, T> {}

#[cfg(any(feature = "std", test))]
mod std {
    use crate::properties::DefinedOrder;
    use std::{
        collections::{btree_set, linked_list, vec_deque, BTreeSet, LinkedList, VecDeque},
        vec,
    };
    impl<T> DefinedOrder for Vec<T> {}
    impl<T> DefinedOrder for vec::IntoIter<T> {}
    impl<T> DefinedOrder for BTreeSet<T> {}
    impl<T> DefinedOrder for btree_set::IntoIter<T> {}
    impl<T> DefinedOrder for btree_set::Iter<'_, T> {}
    impl<T> DefinedOrder for LinkedList<T> {}
    impl<T> DefinedOrder for linked_list::IntoIter<T> {}
    impl<T> DefinedOrder for linked_list::Iter<'_, T> {}
    impl<T> DefinedOrder for linked_list::IterMut<'_, T> {}
    impl<T> DefinedOrder for VecDeque<T> {}
    impl<T> DefinedOrder for vec_deque::IntoIter<T> {}
    impl<T> DefinedOrder for vec_deque::Iter<'_, T> {}
    impl<T> DefinedOrder for vec_deque::IterMut<'_, T> {}
}

#[cfg(not(any(feature = "std", test)))]
mod no_std {
    use crate::properties::DefinedOrder;
    use alloc::{
        collections::{btree_set, linked_list, vec_deque, BTreeSet, LinkedList, VecDeque},
        vec,
        vec::Vec,
    };

    impl<T> DefinedOrder for Vec<T> {}
    impl<T> DefinedOrder for vec::IntoIter<T> {}
    impl<T> DefinedOrder for BTreeSet<T> {}
    impl<T> DefinedOrder for btree_set::IntoIter<T> {}
    impl<T> DefinedOrder for btree_set::Iter<'_, T> {}
    impl<T> DefinedOrder for LinkedList<T> {}
    impl<T> DefinedOrder for linked_list::IntoIter<T> {}
    impl<T> DefinedOrder for linked_list::Iter<'_, T> {}
    impl<T> DefinedOrder for linked_list::IterMut<'_, T> {}
    impl<T> DefinedOrder for VecDeque<T> {}
    impl<T> DefinedOrder for vec_deque::IntoIter<T> {}
    impl<T> DefinedOrder for vec_deque::Iter<'_, T> {}
    impl<T> DefinedOrder for vec_deque::IterMut<'_, T> {}
}
