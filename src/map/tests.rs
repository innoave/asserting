mod hashbrown {
    use crate::prelude::*;
    use crate::std::format;
    use crate::std::vec::Vec;
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

    #[test]
    fn hashmap_contains_keys() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).contains_keys([5, 4]);
    }

    #[test]
    fn verify_hashmap_contains_keys_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_keys([5, 3, 4])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [5, 3, 4]
   but was: {formatted_actual}
  expected: [5, 3, 4]
   missing: [3]
"
            )]
        );
    }

    #[test]
    fn verify_borrowed_hashmap_contains_keys_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(&subject)
            .named("foo_map")
            .contains_keys([5, 3, 4])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [5, 3, 4]
   but was: {formatted_actual}
  expected: [5, 3, 4]
   missing: [3]
"
            )]
        );
    }

    #[test]
    fn verify_mutable_borrowed_hashmap_contains_keys_fails() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(&mut subject)
            .named("foo_map")
            .contains_keys([5, 3, 4])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [5, 3, 4]
   but was: {formatted_actual}
  expected: [5, 3, 4]
   missing: [3]
"
            )]
        );
    }

    #[test]
    fn hashmap_contains_values() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).contains_values(["five", "four"]);
    }

    #[test]
    fn verify_hashmap_contains_values_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_values(["one", "two", "three"])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map contains values ["one", "two", "three"]
   but was: {formatted_actual}
  expected: ["one", "two", "three"]
   missing: ["two", "three"]
"#
            )]
        );
    }

    #[test]
    fn hashmap_does_not_contain_keys() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).does_not_contain_keys([7, 3]);
    }

    #[test]
    fn verify_hashmap_does_not_contain_keys_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);
        let formatted_extra = format!(
            "{:?}",
            subject
                .keys()
                .filter(|k| **k == 5 || **k == 4)
                .collect::<Vec<_>>()
        );

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_keys([5, 3, 4])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map does not contain keys [5, 3, 4]
   but was: {formatted_actual}
  expected: [5, 3, 4]
     extra: {formatted_extra}
"
            )]
        );
    }

    #[test]
    fn hashmap_does_not_contain_values() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).does_not_contain_values(["three", "seven"]);
    }

    #[test]
    fn verify_hashmap_does_not_contain_values_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .does_not_contain_values(["one", "two", "three"])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map does not contain values ["one", "two", "three"]
   but was: {formatted_actual}
  expected: ["one", "two", "three"]
     extra: ["one"]
"#
            )]
        );
    }
}

#[cfg(feature = "std")]
mod std_hash_map {
    use crate::prelude::*;
    use crate::std::collections::HashMap;
    use crate::std::format;

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

    #[test]
    fn hashmap_contains_keys() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).contains_keys([1, 6]);
    }

    #[test]
    fn verify_hashmap_contains_keys_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_keys([2, 3, 5])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [2, 3, 5]
   but was: {formatted_actual}
  expected: [2, 3, 5]
   missing: [2, 3]
"
            )]
        );
    }

    #[test]
    fn verify_borrowed_hashmap_contains_keys_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(&subject)
            .named("foo_map")
            .contains_keys([2, 3, 5])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [2, 3, 5]
   but was: {formatted_actual}
  expected: [2, 3, 5]
   missing: [2, 3]
"
            )]
        );
    }

    #[test]
    fn verify_mutable_borrowed_hashmap_contains_keys_fails() {
        let mut subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(&mut subject)
            .named("foo_map")
            .contains_keys([2, 3, 5])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [2, 3, 5]
   but was: {formatted_actual}
  expected: [2, 3, 5]
   missing: [2, 3]
"
            )]
        );
    }

    #[test]
    fn hashmap_contains_values() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).contains_values(["five", "four"]);
    }

    #[test]
    fn verify_hashmap_contains_values_fails() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_values(["one", "two", "three"])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map contains values ["one", "two", "three"]
   but was: {formatted_actual}
  expected: ["one", "two", "three"]
   missing: ["two", "three"]
"#
            )]
        );
    }
}

mod btree_map {
    use crate::prelude::*;
    use crate::std::collections::BTreeMap;
    use crate::std::format;

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

    #[test]
    fn btree_map_contains_keys() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).contains_keys([5, 1, 6]);
    }

    #[test]
    fn verify_btree_map_contains_keys_fails() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_keys([5, 3, 7])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [5, 3, 7]
   but was: {formatted_actual}
  expected: [5, 3, 7]
   missing: [3, 7]
"
            )]
        );
    }

    #[test]
    fn verify_borrowed_btree_map_contains_keys_fails() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(&subject)
            .named("foo_map")
            .contains_keys([5, 3, 7])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [5, 3, 7]
   but was: {formatted_actual}
  expected: [5, 3, 7]
   missing: [3, 7]
"
            )]
        );
    }

    #[test]
    fn verify_mutable_borrowed_btree_map_contains_keys_fails() {
        let mut subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(&mut subject)
            .named("foo_map")
            .contains_keys([5, 3, 7])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r"assertion failed: expected foo_map contains keys [5, 3, 7]
   but was: {formatted_actual}
  expected: [5, 3, 7]
   missing: [3, 7]
"
            )]
        );
    }

    #[test]
    fn btree_map_contains_values() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();

        assert_that(subject).contains_values(["five", "four"]);
    }

    #[test]
    fn verify_btree_map_contains_values_fails() {
        let subject: BTreeMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject);

        let failures = verify_that(subject)
            .named("foo_map")
            .contains_values(["one", "two", "three"])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                r#"assertion failed: expected foo_map contains values ["one", "two", "three"]
   but was: {formatted_actual}
  expected: ["one", "two", "three"]
   missing: ["two", "three"]
"#
            )]
        );
    }
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;
    use crate::std::format;
    use crate::std::vec::Vec;
    use hashbrown::HashMap;

    #[test]
    fn highlight_diffs_hashmap_contains_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject)
            .replace("5: \"five\"", "\u{1b}[31m5: \"five\"\u{1b}[0m")
            .replace("1: \"one\"", "\u{1b}[31m1: \"one\"\u{1b}[0m")
            .replace("4: \"four\"", "\u{1b}[31m4: \"four\"\u{1b}[0m")
            .replace("6: \"six\"", "\u{1b}[31m6: \"six\"\u{1b}[0m");

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .contains_key(2)
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map contains key 2\n   \
                but was: {formatted_actual}\n  \
               expected: \u{1b}[32m2\u{1b}[0m\n\
            "
            )]
        );
    }

    #[test]
    fn highlight_diffs_hashmap_does_not_contain_key() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual =
            format!("{:?}", &subject).replace("1: \"one\"", "\u{1b}[31m1: \"one\"\u{1b}[0m");

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_contain_key(1)
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map does not contain key 1\n   \
                but was: {formatted_actual}\n  \
               expected: \u{1b}[32m1\u{1b}[0m\n\
            "
            )]
        );
    }

    #[test]
    fn highlight_diffs_hashmap_contains_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject)
            .replace("5: \"five\"", "\u{1b}[31m5: \"five\"\u{1b}[0m")
            .replace("1: \"one\"", "\u{1b}[31m1: \"one\"\u{1b}[0m")
            .replace("4: \"four\"", "\u{1b}[31m4: \"four\"\u{1b}[0m")
            .replace("6: \"six\"", "\u{1b}[31m6: \"six\"\u{1b}[0m");

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .contains_value("three")
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map contains value \"three\"\n   \
                but was: {formatted_actual}\n  \
               expected: \u{1b}[32m\"three\"\u{1b}[0m\n\
            "
            )]
        );
    }

    #[test]
    fn highlight_diffs_hashmap_does_not_contain_value() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual =
            format!("{:?}", &subject).replace("4: \"four\"", "\u{1b}[31m4: \"four\"\u{1b}[0m");

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_contain_value("four")
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map does not contain value \"four\"\n   \
                but was: {formatted_actual}\n  \
               expected: \u{1b}[32m\"four\"\u{1b}[0m\n\
            "
            )]
        );
    }

    #[test]
    fn highlight_diffs_hashmap_contains_keys() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject)
            .replace("1: \"one\"", "\u{1b}[31m1: \"one\"\u{1b}[0m")
            .replace("6: \"six\"", "\u{1b}[31m6: \"six\"\u{1b}[0m");

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .contains_keys([5, 2, 4, 7])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map contains keys [5, 2, 4, 7]\n   \
                but was: {formatted_actual}\n  \
               expected: [5, \u{1b}[32m2\u{1b}[0m, 4, \u{1b}[32m7\u{1b}[0m]\n   \
                missing: [2, 7]\n\
            "
            )]
        );
    }

    #[test]
    fn highlight_diffs_hashmap_does_not_contain_keys() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject)
            .replace("5: \"five\"", "\u{1b}[31m5: \"five\"\u{1b}[0m")
            .replace("4: \"four\"", "\u{1b}[31m4: \"four\"\u{1b}[0m");
        let formatted_extra = format!(
            "{:?}",
            subject
                .keys()
                .filter(|k| **k == 5 || **k == 4)
                .collect::<Vec<_>>()
        );

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_contain_keys([5, 2, 4, 7])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map does not contain keys [5, 2, 4, 7]\n   \
                but was: {formatted_actual}\n  \
               expected: [\u{1b}[32m5\u{1b}[0m, 2, \u{1b}[32m4\u{1b}[0m, 7]\n     \
                  extra: {formatted_extra}\n\
            "
            )]
        );
    }

    #[test]
    fn highlight_diffs_hashmap_contains_values() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject)
            .replace("1: \"one\"", "\u{1b}[31m1: \"one\"\u{1b}[0m")
            .replace("6: \"six\"", "\u{1b}[31m6: \"six\"\u{1b}[0m");

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .contains_values(["five", "two", "four", "seven"])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map contains values [\"five\", \"two\", \"four\", \"seven\"]\n   \
                    but was: {formatted_actual}\n  \
                   expected: [\"five\", \u{1b}[32m\"two\"\u{1b}[0m, \"four\", \u{1b}[32m\"seven\"\u{1b}[0m]\n   \
                    missing: [\"two\", \"seven\"]\n\
                "
            )]
        );
    }

    #[test]
    fn highlight_diffs_hashmap_does_not_contain_values() {
        let subject: HashMap<_, _> = [(5, "five"), (1, "one"), (4, "four"), (6, "six")].into();
        let formatted_actual = format!("{:?}", &subject)
            .replace("5: \"five\"", "\u{1b}[31m5: \"five\"\u{1b}[0m")
            .replace("4: \"four\"", "\u{1b}[31m4: \"four\"\u{1b}[0m");
        let formatted_extra = format!(
            "{:?}",
            subject
                .values()
                .filter(|v| **v == "five" || **v == "four")
                .collect::<Vec<_>>()
        );

        let failures = verify_that(subject)
            .named("foo_map")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_contain_values(["five", "two", "four", "seven"])
            .display_failures();

        assert_eq!(
            failures,
            &[format!(
                "assertion failed: expected foo_map does not contain values [\"five\", \"two\", \"four\", \"seven\"]\n   \
                    but was: {formatted_actual}\n  \
                   expected: [\u{1b}[32m\"five\"\u{1b}[0m, \"two\", \u{1b}[32m\"four\"\u{1b}[0m, \"seven\"]\n     \
                      extra: {formatted_extra}\n\
                "
            )]
        );
    }
}
