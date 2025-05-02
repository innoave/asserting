mod hashbrown {
    use crate::prelude::*;
    use crate::std::format;
    use hashbrown::HashMap;

    #[test]
    fn hashmap_is_empty() {
        let subject: HashMap<usize, &str> = HashMap::new();

        assert_that(subject).is_empty();
    }

    #[test]
    fn hashmap_is_not_empty() {
        let subject: HashMap<_, _> = [(5, "five")].into();

        assert_that(subject).is_not_empty();
    }

    #[test]
    fn hashmap_has_length() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).has_length(3);
    }

    #[test]
    fn hashmap_has_length_in_range() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).has_length_in_range(3..4);
        assert_that(&subject).has_length_in_range(3..=4);
    }

    #[test]
    fn hashmap_contains_key_value_pair() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains((5, "five"));
    }

    #[test]
    fn borrowed_hashmap_contains_key_value_pair() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains((&5, &"five"));
    }

    #[test]
    fn mutable_borrowed_hashmap_contains_key_value_pair() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains((&5, &mut "five"));
    }

    #[test]
    fn hashmap_contains_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains_key(5);
    }

    #[test]
    fn borrowed_hashmap_contains_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains_key(5);
    }

    #[test]
    fn mutable_borrowed_hashmap_contains_key() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains_key(5);
    }

    #[test]
    fn verify_hashmap_contains_key_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_key(7)
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains key 7
   but was: {formatted_actual}
  expected: 7
"
            )]
        );
    }

    #[test]
    fn hashmap_does_not_contain_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).does_not_contain_key(6);
    }

    #[test]
    fn verify_hashmap_does_not_contain_key_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_key(5)
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map does not contain key 5
   but was: {formatted_actual}
  expected: 5
"
            )]
        );
    }

    #[test]
    fn hashmap_map_contains_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains_value("four");
    }

    #[test]
    fn borrowed_hashmap_map_contains_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains_value("four");
    }

    #[test]
    fn mutable_borrowed_hashmap_map_contains_value() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains_value("four");
    }

    #[test]
    fn verify_hashmap_contains_value_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_value("six")
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map contains value "six"
   but was: {formatted_actual}
  expected: "six"
"#
            )]
        );
    }

    #[test]
    fn hashmap_map_does_not_contain_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).does_not_contain_value("six");
    }

    #[test]
    fn verify_hashmap_does_not_contain_value_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_value("five")
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map does not contain value "five"
   but was: {formatted_actual}
  expected: "five"
"#
            )]
        );
    }
}

#[cfg(feature = "std")]
mod std_hash_map {
    use crate::prelude::*;
    use crate::std::collections::HashMap;

    #[test]
    fn hashmap_is_empty() {
        let subject: HashMap<usize, &str> = HashMap::new();

        assert_that(subject).is_empty();
    }

    #[test]
    fn hashmap_is_not_empty() {
        let subject: HashMap<_, _> = [(5, "five")].into();

        assert_that(subject).is_not_empty();
    }

    #[test]
    fn hashmap_has_length() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).has_length(3);
    }

    #[test]
    fn hashmap_has_length_in_range() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).has_length_in_range(3..4);
        assert_that(&subject).has_length_in_range(3..=4);
    }

    #[test]
    fn hashmap_contains_key_value_pair() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains((4, "four"));
    }

    #[test]
    fn borrowed_hashmap_contains_key_value_pair() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains((&4, &"four"));
    }

    #[test]
    fn mutable_borrowed_hashmap_contains_key_value_pair() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains((&4, &mut "four"));
    }

    #[test]
    fn hashmap_contains_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains_key(5);
    }

    #[test]
    fn borrowed_hashmap_contains_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains_key(5);
    }

    #[test]
    fn mutably_borrowed_hashmap_contains_key() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains_key(5);
    }

    #[test]
    fn verify_hashmap_contains_key_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_key(7)
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains key 7
   but was: {formatted_actual}
  expected: 7
"
            )]
        );
    }

    #[test]
    fn hashmap_does_not_contain_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).does_not_contain_key(6);
    }

    #[test]
    fn verify_hashmap_does_not_contain_key_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_key(4)
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map does not contain key 4
   but was: {formatted_actual}
  expected: 4
"
            )]
        );
    }

    #[test]
    fn hashmap_map_contains_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains_value("four");
    }

    #[test]
    fn borrowed_hashmap_map_contains_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains_value("four");
    }

    #[test]
    fn mutable_borrowed_hashmap_map_contains_value() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains_value("four");
    }

    #[test]
    fn verify_hashmap_contains_value_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_value("six")
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map contains value "six"
   but was: {formatted_actual}
  expected: "six"
"#
            )]
        );
    }

    #[test]
    fn hashmap_map_does_not_contain_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).does_not_contain_value("six");
    }

    #[test]
    fn verify_hashmap_does_not_contain_value_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_value("four")
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map does not contain value "four"
   but was: {formatted_actual}
  expected: "four"
"#
            )]
        );
    }
}

mod btree_map {
    use crate::prelude::*;
    use crate::std::collections::BTreeMap;

    #[test]
    fn btree_map_is_empty() {
        let subject: BTreeMap<usize, &str> = BTreeMap::new();

        assert_that(subject).is_empty();
    }

    #[test]
    fn hashmap_is_not_empty() {
        let subject: BTreeMap<_, _> = [(5, "five")].into();

        assert_that(subject).is_not_empty();
    }

    #[test]
    fn btree_map_has_length() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).has_length(3);
    }

    #[test]
    fn btree_map_has_length_in_range() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).has_length_in_range(3..4);
        assert_that(&subject).has_length_in_range(3..=4);
    }

    #[test]
    fn btree_map_contains_key_value_pair() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains((1, "one"));
    }

    #[test]
    fn borrowed_btree_map_contains_key_value_pair() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains((&1, &"one"));
    }

    #[test]
    fn mutable_borrowed_btree_map_contains_key_value_pair() {
        let mut subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains((&1, &mut "one"));
    }

    #[test]
    fn btree_map_contains_key() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains_key(5);
    }

    #[test]
    fn borrowed_btree_map_contains_key() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains_key(5);
    }

    #[test]
    fn mutable_borrowed_btree_map_contains_key() {
        let mut subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains_key(5);
    }

    #[test]
    fn verify_btree_map_contains_key_fails() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_key(7)
            .display_failures();

        assert_eq!(
            failures,
            &[r#"assertion failed: expected foo_map contains key 7
   but was: {1: "one", 4: "four", 5: "five"}
  expected: 7
"#]
        );
    }

    #[test]
    fn btree_map_does_not_contain_key() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).does_not_contain_key(6);
    }

    #[test]
    fn verify_btree_map_does_not_contain_key_fails() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_key(1)
            .display_failures();

        assert_eq!(
            failures,
            &[r#"assertion failed: expected foo_map does not contain key 1
   but was: {1: "one", 4: "four", 5: "five"}
  expected: 1
"#]
        );
    }

    #[test]
    fn btree_map_contains_value() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).contains_value("four");
    }

    #[test]
    fn borrowed_btree_map_contains_value() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&subject).contains_value("four");
    }

    #[test]
    fn mutable_borrowed_btree_map_contains_value() {
        let mut subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(&mut subject).contains_value("four");
    }

    #[test]
    fn verify_btree_map_contains_value_fails() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_value("six")
            .display_failures();

        assert_eq!(
            failures,
            &[r#"assertion failed: expected foo_map contains value "six"
   but was: {1: "one", 4: "four", 5: "five"}
  expected: "six"
"#]
        );
    }

    #[test]
    fn btree_map_does_not_contain_value() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        assert_that(subject).does_not_contain_value("six");
    }

    #[test]
    fn verify_btree_map_does_not_contain_value_fails() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four")].into();

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_value("one")
            .display_failures();

        assert_eq!(
            failures,
            &[
                r#"assertion failed: expected foo_map does not contain value "one"
   but was: {1: "one", 4: "four", 5: "five"}
  expected: "one"
"#
            ]
        );
    }
}
