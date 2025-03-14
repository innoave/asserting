//! Implementation of assertions for collections and iterators.

use crate::prelude::{DefinedOrder, IsEmptyProperty, LengthProperty};
use crate::std::{array, slice};
use hashbrown::{HashMap, HashSet};

impl<T> DefinedOrder for [T] {}
impl<T, const N: usize> DefinedOrder for [T; N] {}
impl<T, const N: usize> DefinedOrder for array::IntoIter<T, N> {}
impl<T> DefinedOrder for slice::Iter<'_, T> {}
impl<T> DefinedOrder for slice::IterMut<'_, T> {}

#[cfg(feature = "std")]
mod std {
    use crate::prelude::{IsEmptyProperty, LengthProperty};
    use crate::properties::DefinedOrder;
    use std::{
        collections::{
            btree_map, btree_set, linked_list, vec_deque, BTreeMap, BTreeSet, BinaryHeap, HashMap,
            HashSet, LinkedList, VecDeque,
        },
        vec,
    };

    impl<T> DefinedOrder for Vec<T> {}
    impl<T> DefinedOrder for vec::IntoIter<T> {}
    impl<T> DefinedOrder for BTreeSet<T> {}
    impl<T> DefinedOrder for btree_set::IntoIter<T> {}
    impl<T> DefinedOrder for btree_set::Iter<'_, T> {}
    impl<K, V> DefinedOrder for BTreeMap<K, V> {}
    impl<K, V> DefinedOrder for btree_map::IntoIter<K, V> {}
    impl<K, V> DefinedOrder for btree_map::Iter<'_, K, V> {}
    impl<T> DefinedOrder for LinkedList<T> {}
    impl<T> DefinedOrder for linked_list::IntoIter<T> {}
    impl<T> DefinedOrder for linked_list::Iter<'_, T> {}
    impl<T> DefinedOrder for linked_list::IterMut<'_, T> {}
    impl<T> DefinedOrder for VecDeque<T> {}
    impl<T> DefinedOrder for vec_deque::IntoIter<T> {}
    impl<T> DefinedOrder for vec_deque::Iter<'_, T> {}
    impl<T> DefinedOrder for vec_deque::IterMut<'_, T> {}

    impl<T> IsEmptyProperty for BinaryHeap<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<K, V, S> IsEmptyProperty for HashMap<K, V, S> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T, S> IsEmptyProperty for HashSet<T, S> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<K, V> IsEmptyProperty for BTreeMap<K, V> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> IsEmptyProperty for BTreeSet<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> IsEmptyProperty for LinkedList<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> IsEmptyProperty for VecDeque<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> LengthProperty for BinaryHeap<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<K, V> LengthProperty for BTreeMap<K, V> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<K, V, S> LengthProperty for HashMap<K, V, S> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<T, S> LengthProperty for HashSet<T, S> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<T> LengthProperty for BTreeSet<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<T> LengthProperty for LinkedList<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<T> LengthProperty for VecDeque<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }
}

#[cfg(not(feature = "std"))]
mod no_std {
    use crate::prelude::{IsEmptyProperty, LengthProperty};
    use crate::properties::DefinedOrder;
    use alloc::{
        collections::{
            btree_set, linked_list, vec_deque, BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque,
        },
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

    impl<T> IsEmptyProperty for BinaryHeap<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<K, V> IsEmptyProperty for BTreeMap<K, V> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> IsEmptyProperty for BTreeSet<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> IsEmptyProperty for LinkedList<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> IsEmptyProperty for VecDeque<T> {
        fn is_empty_property(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> LengthProperty for BinaryHeap<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<K, V> LengthProperty for BTreeMap<K, V> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<T> LengthProperty for BTreeSet<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<T> LengthProperty for LinkedList<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }

    impl<T> LengthProperty for VecDeque<T> {
        fn length_property(&self) -> usize {
            self.len()
        }
    }
}

impl<T, const N: usize> IsEmptyProperty for [T; N] {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V, S> IsEmptyProperty for HashMap<K, V, S> {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl<T, S> IsEmptyProperty for HashSet<T, S> {
    fn is_empty_property(&self) -> bool {
        self.is_empty()
    }
}

impl<T, const N: usize> LengthProperty for [T; N] {
    fn length_property(&self) -> usize {
        self.len()
    }
}

impl<K, V, S> LengthProperty for HashMap<K, V, S> {
    fn length_property(&self) -> usize {
        self.len()
    }
}

impl<T, S> LengthProperty for HashSet<T, S> {
    fn length_property(&self) -> usize {
        self.len()
    }
}
