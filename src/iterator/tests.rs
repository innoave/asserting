use crate::prelude::*;
use crate::std::{vec, vec::Vec};

#[derive(Debug)]
struct CustomCollection<T> {
    inner: Vec<T>,
}

struct CustomIter<T> {
    inner: vec::IntoIter<T>,
}

impl<T> Iterator for CustomIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> IntoIterator for CustomCollection<T> {
    type Item = T;
    type IntoIter = CustomIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        CustomIter {
            inner: self.inner.into_iter(),
        }
    }
}

impl<T> IsEmptyProperty for CustomCollection<T> {
    fn is_empty_property(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T> LengthProperty for CustomCollection<T> {
    fn length_property(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug)]
struct CustomOrderedCollection<T> {
    inner: Vec<T>,
}

struct CustomOrderedIter<T> {
    inner: vec::IntoIter<T>,
}

impl<T> Iterator for CustomOrderedIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> IntoIterator for CustomOrderedCollection<T> {
    type Item = T;
    type IntoIter = CustomOrderedIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        CustomOrderedIter {
            inner: self.inner.into_iter(),
        }
    }
}

impl<T> DefinedOrderProperty for CustomOrderedIter<T> {}

#[test]
fn custom_collection_is_empty() {
    let subject: CustomCollection<i32> = CustomCollection { inner: vec![] };

    assert_that(subject).is_empty();
}

#[test]
fn custom_collection_is_not_empty() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17],
    };

    assert_that(subject).is_not_empty();
}

#[test]
fn custom_collection_has_length() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13],
    };

    assert_that(subject).has_length(6);
}

#[test]
fn custom_collection_has_length_in_range() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19],
    };

    assert_that(subject).has_length_in_range(7..=8);
}

#[test]
fn custom_collection_contains() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    };

    assert_that(subject).contains(19).contains(43).contains(1);
}

#[test]
fn verify_custom_collection_contains_fails() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    };

    let failures = verify_that(subject)
        .named("my_thing")
        .contains(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to contain 42
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: 42
"]
    );
}

#[test]
fn custom_iterator_contains() {
    let subject: CustomIter<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    }
    .into_iter();

    assert_that(subject).contains(19).contains(43).contains(1);
}

#[test]
fn verify_custom_iterator_contains_fails() {
    let subject: CustomIter<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    }
    .into_iter();

    let failures = verify_that(subject)
        .named("my_thing")
        .contains(42)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to contain 42
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: 42
"]
    );
}

#[test]
fn custom_collection_does_not_contain() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    };

    assert_that(subject)
        .does_not_contain(2)
        .does_not_contain(4)
        .does_not_contain(6);
}

#[test]
fn verify_custom_collection_does_not_contain_fails() {
    let subject: CustomCollection<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    };

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain(19)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to not contain 19
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: not 19
"]
    );
}

#[test]
fn custom_iterator_does_not_contain() {
    let subject: CustomIter<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    }
    .into_iter();

    assert_that(subject)
        .does_not_contain(2)
        .does_not_contain(4)
        .does_not_contain(6);
}

#[test]
fn verify_custom_iterator_does_not_contain_fails() {
    let subject: CustomIter<i32> = CustomCollection {
        inner: vec![1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43],
    }
    .into_iter();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain(19)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to not contain 19
   but was: [1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]
  expected: not 19
"]
    );
}

mod element_filters {
    use super::*;

    #[test]
    fn single_element_of_iterator_with_one_element() {
        let subject = CustomCollection {
            inner: vec!["single"],
        };

        assert_that(subject)
            .single_element()
            .is_equal_to("single")
            .has_length(6)
            .starts_with('s');
    }

    #[cfg(feature = "panic")]
    #[test]
    fn single_element_of_empty_iterator_fails() {
        let subject: CustomCollection<i32> = CustomCollection { inner: vec![] };

        assert_that_code(|| {
            assert_that(subject)
                .named("my_custom_collection")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .single_element()
                .is_equal_to(42);
        })
        .panics_with_message(
            r"expected my_custom_collection to have exactly one element, but has no elements
  actual: []
",
        );
    }

    #[cfg(feature = "panic")]
    #[test]
    fn single_element_of_iterator_with_2_elements_fails() {
        let subject: CustomCollection<i32> = CustomCollection {
            inner: vec![42, -1],
        };

        assert_that_code(|| {
            assert_that(subject)
                .named("my_custom_collection")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .single_element()
                .is_equal_to(42);
        })
        .panics_with_message(
            r"expected my_custom_collection to have exactly one element, but has 2 elements
  actual: [42, -1]
",
        );
    }

    #[test]
    fn first_element_of_iterator_with_one_element() {
        let subject = CustomOrderedCollection {
            inner: vec!["single"],
        };

        assert_that(subject)
            .first_element()
            .is_equal_to("single")
            .has_length(6)
            .starts_with("si");
    }

    #[test]
    fn first_element_of_iterator_with_several_elements() {
        let subject = CustomOrderedCollection {
            inner: vec!["one", "two", "three", "four", "five"],
        };

        assert_that(subject)
            .first_element()
            .is_equal_to("one")
            .has_length(3)
            .starts_with('o');
    }

    #[cfg(feature = "panic")]
    #[test]
    fn first_element_of_iterator_with_no_elements_fails() {
        let subject: CustomOrderedCollection<i32> = CustomOrderedCollection { inner: vec![] };

        assert_that_code(|| {
            assert_that(subject)
                .named("my_custom_collection")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .first_element()
                .is_equal_to(42);
        })
        .panics_with_message(
            r"expected my_custom_collection to have at least one element, but has no elements
  actual: []
",
        );
    }

    #[test]
    fn last_element_of_iterator_with_one_element() {
        let subject = CustomOrderedCollection {
            inner: vec!["single"],
        };

        assert_that(subject)
            .last_element()
            .is_equal_to("single")
            .has_length(6)
            .starts_with("si");
    }

    #[test]
    fn last_element_of_iterator_with_several_elements() {
        let subject = CustomOrderedCollection {
            inner: vec!["one", "two", "three", "four", "five"],
        };

        assert_that(subject)
            .last_element()
            .is_equal_to("five")
            .has_length(4)
            .starts_with("fi");
    }

    #[cfg(feature = "panic")]
    #[test]
    fn last_element_of_iterator_with_no_elements_fails() {
        let subject: CustomOrderedCollection<i32> = CustomOrderedCollection { inner: vec![] };

        assert_that_code(|| {
            assert_that(subject)
                .named("my_custom_collection")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .last_element()
                .is_equal_to(42);
        })
        .panics_with_message(
            r"expected my_custom_collection to have at least one element, but has no elements
  actual: []
",
        );
    }

    #[test]
    fn nth_element_of_iterator_with_one_element() {
        let subject = CustomOrderedCollection {
            inner: vec!["single"],
        };

        assert_that(subject)
            .nth_element(0)
            .is_equal_to("single")
            .has_length(6)
            .starts_with("si");
    }

    #[test]
    fn nth_element_of_iterator_with_several_elements_second_element() {
        let subject = CustomOrderedCollection {
            inner: vec!["one", "two", "three", "four", "five"],
        };

        assert_that(subject)
            .nth_element(1)
            .is_equal_to("two")
            .has_length(3)
            .starts_with("tw");
    }

    #[test]
    fn nth_element_of_iterator_with_several_elements_fifth_element() {
        let subject = CustomOrderedCollection {
            inner: vec!["one", "two", "three", "four", "five"],
        };

        assert_that(subject)
            .nth_element(4)
            .is_equal_to("five")
            .has_length(4)
            .starts_with("fi");
    }

    #[cfg(feature = "panic")]
    #[test]
    fn nth_element_of_iterator_with_five_elements_6th_element_fails() {
        let subject = CustomOrderedCollection {
            inner: vec!["one", "two", "three", "four", "five"],
        };

        assert_that_code(|| {
            assert_that(subject)
                .named("my_custom_collection")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .nth_element(5)
                .is_equal_to("five");
        })
        .panics_with_message(
            r#"expected my_custom_collection to have at least 6 elements, but has 5 elements
  actual: ["one", "two", "three", "four", "five"]
"#,
        );
    }

    #[test]
    fn filtered_on_elements_of_iterator_even_elements() {
        let subject = CustomCollection {
            inner: vec![1, 2, 3, 4, 5],
        };

        assert_that(subject)
            .filtered_on(|e| e & 1 == 0)
            .contains_exactly_in_any_order([2, 4]);
    }

    #[test]
    fn elements_at_positions_of_iterator() {
        let subject = CustomOrderedCollection {
            inner: vec!["one", "two", "three", "four", "five"],
        };

        assert_that(subject)
            .elements_at([0, 2, 4])
            .contains_exactly(["one", "three", "five"]);
    }

    #[test]
    fn any_satisfies_on_elements_of_iterator_value_is_equal_to_42() {
        let subject = CustomCollection {
            inner: vec![1, 41, 43, 42, 5],
        };

        assert_that(subject).any_satisfies(|e| *e == 42);
    }

    #[test]
    fn verify_any_satisfies_on_elements_of_iterator_value_is_equal_to_42_fails() {
        let subject = CustomCollection {
            inner: vec![1, 2, 43, 41, 5],
        };

        let failures = verify_that(subject)
            .named("my_numbers")
            .any_satisfies(|e| *e == 42)
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected any element of my_numbers to satisfy the predicate, but none did
  actual: [1, 2, 43, 41, 5]
"
            ]
        );
    }

    #[test]
    fn all_satisfy_on_elements_of_iterator_value_is_greater_than_42() {
        let subject = CustomCollection {
            inner: vec![47, 46, 45, 44, 43],
        };

        assert_that(subject).all_satisfy(|e| *e > 42);
    }

    #[test]
    fn verify_all_satisfy_on_elements_of_iterator_value_is_greater_than_42_fails() {
        let subject = CustomCollection {
            inner: vec![43, 44, 45, 42, 47],
        };

        let failures = verify_that(subject)
            .named("my_numbers")
            .all_satisfy(|e| *e > 42)
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected all elements of my_numbers to satisfy the predicate, but 1 did not
   actual: [43, 44, 45, 42, 47]
  failing: [42]
"
            ]
        );
    }

    #[test]
    fn none_satisfies_on_elements_of_iterator_value_is_greater_than_42() {
        let subject = CustomCollection {
            inner: vec![42, 41, 40, 39, 38],
        };

        assert_that(subject).none_satisfies(|e| *e > 42);
    }

    #[test]
    fn verify_none_satisfies_on_elements_of_iterator_value_is_greater_than_42_fails() {
        let subject = CustomCollection {
            inner: vec![41, 43, 45, 42, 47],
        };

        let failures = verify_that(subject)
            .named("my_numbers")
            .none_satisfies(|e| *e > 42)
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected none of the elements of my_numbers to satisfy the predicate, but 3 did
   actual: [41, 43, 45, 42, 47]
  failing: [43, 45, 47]
"
            ]
        );
    }

    #[cfg(feature = "colored")]
    mod colored {
        use super::*;

        #[test]
        fn highlight_all_satisfy_on_elements_of_iterator() {
            let subject = CustomCollection {
                inner: vec![43, 44, 45, 42, 47],
            };

            let failures = verify_that(subject)
                .named("my_numbers")
                .with_diff_format(DIFF_FORMAT_RED_YELLOW)
                .all_satisfy(|e| *e > 42)
                .display_failures();

            assert_eq!(
                failures,
                &[
                    "expected all elements of my_numbers to satisfy the predicate, but 1 did not\n   \
                        actual: [43, 44, 45, \u{1b}[31m42\u{1b}[0m, 47]\n  \
                       failing: [42]\n"
                ]
            );
        }

        #[test]
        fn highlight_none_satisfies_on_elements_of_iterator() {
            let subject = CustomCollection {
                inner: vec![41, 43, 45, 42, 47],
            };

            let failures = verify_that(subject)
                .named("my_numbers")
                .with_diff_format(DIFF_FORMAT_RED_YELLOW)
                .none_satisfies(|e| *e > 42)
                .display_failures();

            assert_eq!(
                failures,
                &[
                    "expected none of the elements of my_numbers to satisfy the predicate, but 3 did\n   \
                        actual: [41, \u{1b}[31m43\u{1b}[0m, \u{1b}[31m45\u{1b}[0m, 42, \u{1b}[31m47\u{1b}[0m]\n  \
                       failing: [43, 45, 47]\n"
                ]
            );
        }
    }
}
