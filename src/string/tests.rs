use crate::prelude::*;
use crate::std::string::{String, ToString};

#[test]
fn string_is_equal_to_string() {
    let subject: String = "stet invidunt gubergren iusto".to_string();

    assert_that(subject)
        .is_equal_to("stet invidunt gubergren iusto".to_string())
        .is_not_equal_to("bruno");
}

#[test]
fn string_is_equal_to_str() {
    let subject: String = "adipisici mollit hendrerit nostrud".to_string();

    assert_that(subject).is_equal_to("adipisici mollit hendrerit nostrud");
}

#[test]
fn string_ref_is_equal_to_str() {
    let subject: &String = &"duo exerci laborum doming".to_string();

    assert_that(subject).is_equal_to("duo exerci laborum doming");
}

#[test]
fn str_is_equal_to_str() {
    let subject: &str = "id elit vero praesent";

    assert_that(subject).is_equal_to("id elit vero praesent");
}

#[test]
fn str_is_equal_to_string() {
    let subject: &str = "ex tincidunt nam cupiditat";

    assert_that(subject).is_equal_to("ex tincidunt nam cupiditat");
}

#[test]
fn string_is_not_equal_to_string() {
    let subject: String = "volutpat voluptate nibh volutpat".to_string();

    assert_that(subject).is_not_equal_to("wisi nihil commodi ex".to_string());
}

#[test]
fn string_is_not_equal_to_str() {
    let subject: String = "consectetuer qui tincidunt adipiscing".to_string();

    assert_that(subject).is_not_equal_to("takimata wisi dolor vulputate");
}

#[test]
fn string_ref_is_not_equal_to_str() {
    let subject: String = "sunt facer clita delenit".to_string();

    assert_that(&subject).is_not_equal_to("tation zzril proident suscipit");
}

#[test]
fn str_is_not_equal_to_str() {
    let subject: &str = "cum consectetur sadipscing vulputate";

    assert_that(subject).is_not_equal_to("quod accumsan veniam doming");
}

#[test]
fn str_is_not_equal_to_string() {
    let subject: &str = "veniam mollit incidunt tincidunt";

    assert_that(subject).is_not_equal_to("est commodo eleifend imperdiet".to_string());
}

#[test]
fn verify_string_is_equal_to_str_fails() {
    let failures = verify_that(String::new())
        .named("my_thing")
        .is_equal_to("aute duis eleifend molestie")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing is equal to "aute duis eleifend molestie"
   but was: ""
  expected: "aute duis eleifend molestie"
"#
        ]
    );
}

#[test]
fn string_is_empty() {
    let subject: String = String::new();

    assert_that(subject).is_empty();
}

#[test]
fn string_is_not_empty() {
    let subject: String = "ABC".to_string();

    assert_that(subject).is_not_empty();
}

#[test]
fn borrowed_string_is_empty() {
    let subject: &String = &String::new();

    assert_that(subject).is_empty();
}

#[test]
fn mutable_borrowed_string_is_empty() {
    let subject: &mut String = &mut String::new();

    assert_that(subject).is_empty();
}

#[test]
fn str_is_empty() {
    let subject: &str = "";

    assert_that(subject).is_empty();
}

#[test]
fn str_is_not_empty() {
    let subject: &str = "ABC";

    assert_that(subject).is_not_empty();
}

#[test]
fn verify_str_is_empty_fails() {
    let subject: &str = "ABC";

    let failures = verify_that(subject)
        .named("my_thing")
        .is_empty()
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing is empty
   but was: "ABC"
  expected: <empty>
"#]
    );
}

#[test]
fn verify_string_is_not_empty_fails() {
    let subject: String = String::new();

    let failures = verify_that(subject)
        .named("my_thing")
        .is_not_empty()
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing is not empty
   but was: ""
  expected: <non-empty>
"#]
    );
}

#[test]
fn string_has_length() {
    let subject: String = "aute lobortis voluptua pariatur".to_string();

    assert_that(subject).has_length(31);
}

#[test]
fn mutable_borrowed_string_has_length() {
    let subject: &mut String = &mut "aute lobortis voluptua pariatur".to_string();

    assert_that(subject).has_length(31);
}

#[test]
fn str_has_length() {
    let subject: &str = "ad fugiat duo erat";

    assert_that(subject).has_length(18);
}

#[test]
fn verify_str_has_length_fails() {
    let subject: &str = "officia volutpat duis iriure";

    let failures = verify_that(subject)
        .named("my_thing")
        .has_length(29)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing has length 29
   but was: 28
  expected: 29
"]
    );
}

#[test]
fn string_has_length_in_range() {
    let subject: String = "fugiat vero cillum dolore".to_string();

    assert_that(subject).has_length_in_range(1..=25);
}

#[test]
fn verify_has_length_in_range_fails() {
    let subject: String = "fugiat vero cillum dolore".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_length_in_range(1..=24)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing has length in range 1..=24
   but was: 25
  expected: 1..=24
"
        ]
    );
}

#[test]
fn string_has_length_less_than() {
    let subject: String = "congue veniam et proident".to_string();

    assert_that(subject).has_length_less_than(26);
}

#[test]
fn verify_string_has_length_less_than_fails() {
    let subject: String = "congue veniam et proident".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_length_less_than(25)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing has a length less than 25
   but was: 25
  expected: < 25
"
        ]
    );
}

#[test]
fn string_has_length_greater_than() {
    let subject: String = "deserunt elit aliquip eirmod".to_string();

    assert_that(subject).has_length_greater_than(27);
}

#[test]
fn verify_string_has_length_greater_than_fails() {
    let subject: String = "deserunt elit aliquip eirmod".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_length_greater_than(28)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing has a length greater than 28
   but was: 28
  expected: > 28
"
        ]
    );
}

#[test]
fn string_has_at_most_length() {
    let subject: String = "facilisi euismod veniam labore".to_string();

    assert_that(&subject).has_at_most_length(30);
    assert_that(subject).has_at_most_length(31);
}

#[test]
fn verify_string_has_at_most_length_fails() {
    let subject: String = "facilisi euismod veniam labore".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_at_most_length(29)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing has at most a length of 29
   but was: 30
  expected: <= 29
"
        ]
    );
}

#[test]
fn string_has_at_least_length() {
    let subject: String = "autem in option zzril".to_string();

    assert_that(&subject).has_at_least_length(21);
    assert_that(subject).has_at_least_length(20);
}

#[test]
fn verify_string_has_at_least_length_fails() {
    let subject: String = "autem in option zzril".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_at_least_length(22)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing has at least a length of 22
   but was: 21
  expected: >= 22
"
        ]
    );
}

#[test]
fn string_has_char_count() {
    let subject: String = "option\u{0074}\u{02B0} sadipscing accusam augue".to_string();

    assert_that(&subject).has_length(34);
    assert_that(subject).has_char_count(33);
}

#[test]
fn borrowed_string_has_char_count() {
    let subject: &String = &"option\u{0074}\u{02B0} sadipscing accusam augue".to_string();

    assert_that(subject).has_length(34);
    assert_that(subject).has_char_count(33);
}

#[test]
fn mutable_borrowed_string_has_char_count() {
    let subject: &mut String = &mut "option\u{0074}\u{02B0} sadipscing accusam augue".to_string();

    assert_that(&subject).has_length(34);
    assert_that(subject).has_char_count(33);
}

#[test]
fn str_has_char_count() {
    let subject: &str = "imper\u{0180}diet al\u{02AA}iquyam \u{01AF} zzril aliquip";

    assert_that(subject).has_length(39);
    assert_that(subject).has_char_count(36);
}

#[test]
fn verify_str_has_char_count_fails() {
    let subject: &str = "\u{0112} \u{0034} \u{0200}";

    let failures = verify_that(subject)
        .named("my_thing")
        .has_char_count(7)
        .display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected my_thing has a char count of 7
   but was: 5
  expected: 7
"]
    );
}

#[test]
fn string_has_char_count_in_range() {
    let subject: String = "\u{0112} \u{0034} \u{0200}".to_string();

    assert_that(subject).has_char_count_in_range(5..=5);
}

#[test]
fn borrowed_string_has_char_count_in_range() {
    let subject: &String = &"\u{0112} \u{0034} \u{0200}".to_string();

    assert_that(subject).has_char_count_in_range(5..=5);
}

#[test]
fn str_has_char_count_in_range() {
    let subject: &str = "\u{0112} \u{0034} \u{0200}";

    assert_that(subject).has_char_count_in_range(5..=5);
}

#[test]
fn verify_str_has_char_count_in_range_fails() {
    let subject: &str = "\u{0112} \u{0034} \u{0200}";

    let failures = verify_that(subject)
        .named("my_thing")
        .has_char_count_in_range(6..=12)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected my_thing has a char count of 6..=12
   but was: 5
  expected: 6..=12
"
        ]
    );
}

#[test]
fn string_contains_other_str() {
    let subject: String = "illum kasd nostrud possim".to_string();

    assert_that(subject).contains("nostrud");
}

#[test]
fn string_contains_other_string() {
    let subject: String = "consectetuer nulla anim nihil".to_string();

    assert_that(subject).contains(" nulla ".to_string());
}

#[test]
fn str_contains_other_str() {
    let subject: &str = "consectetuer duis quis veniam";

    assert_that(subject).contains("quis veniam");
}

#[test]
fn str_contains_other_string() {
    let subject: &str = "voluptua liber assum facilisis";

    assert_that(subject).contains("voluptua liber assum facilisis".to_string());
}

#[test]
fn str_contains_a_char() {
    let subject: &str = "praesent doming liber accusam";

    assert_that(subject).contains(' ');
}

#[test]
fn verify_string_contains_other_str_fails() {
    let subject: String = "invidunt eos hendrerit commodo".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .contains("not a substring")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to contain "not a substring"
   but was: "invidunt eos hendrerit commodo"
  expected: "not a substring"
"#
        ]
    );
}

#[test]
fn verify_string_contains_other_string_fails() {
    let subject: String = "invidunt eos hendrerit commodo".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .contains("not a substring".to_string())
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to contain "not a substring"
   but was: "invidunt eos hendrerit commodo"
  expected: "not a substring"
"#
        ]
    );
}

#[test]
fn verify_string_contains_char_fails() {
    let subject: String = "consectetur ex hendrerit officia".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .contains('Q')
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing to contain 'Q'
   but was: "consectetur ex hendrerit officia"
  expected: 'Q'
"#]
    );
}

#[test]
fn string_contains_any_char_of_a_slice_of_chars() {
    let subject: String = "dolore reprehenderit erat duis".to_string();

    assert_that(subject).contains_any_of(&['o', 'e', 'r', 't'][..]);
}

#[test]
fn str_contains_any_char_of_an_array_of_chars() {
    let subject: &str = "duo excepteur invidunt nonumy";

    assert_that(subject).contains_any_of(['x', 'v', 'y']);
}

#[test]
fn string_contains_any_char_of_a_borrowed_array_of_chars() {
    let subject: String = "sadipscing nibh nisi voluptua".to_string();

    assert_that(subject).contains_any_of(&['a', 'e', 'i', 'o', 'u']);
}

#[test]
fn verify_str_contains_any_char_of_a_slice_of_chars_fails() {
    let subject: &str = "luptatum in nihil laoreet";

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_any_of(&['x', 'y', 'z'][..])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#
        ]
    );
}

#[test]
fn verify_string_contains_any_char_of_an_array_of_chars_fails() {
    let subject: String = "luptatum in nihil laoreet".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_any_of(['x', 'y', 'z'])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#
        ]
    );
}

#[test]
fn verify_str_contains_any_char_of_a_borrowed_array_of_chars_fails() {
    let subject: &str = "luptatum in nihil laoreet";

    let failures = verify_that(subject)
        .named("my_thing")
        .contains_any_of(&['x', 'y', 'z'])
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#
        ]
    );
}

#[test]
fn string_starts_with_str() {
    let subject: String = "wisi option excepteur labore".to_string();

    assert_that(subject).starts_with("wisi");
}

#[test]
fn string_starts_with_string() {
    let subject: String = "sanctus stet eirmod voluptate".to_string();

    assert_that(subject).starts_with("sanctus ".to_string());
}

#[test]
fn string_starts_with_char() {
    let subject: String = "odio gubergren aliquip blandit".to_string();

    assert_that(subject).starts_with('o');
}

#[test]
fn str_starts_with_str() {
    let subject: &str = "stet nam consetetur placerat";

    assert_that(subject).starts_with("stet na");
}

#[test]
fn str_starts_with_string() {
    let subject: &str = "dolores invidunt exerci nostrud";

    assert_that(subject).starts_with("dolor".to_string());
}

#[test]
fn str_starts_with_char() {
    let subject: &str = "odio gubergren aliquip blandit";

    assert_that(subject).starts_with('o');
}

#[test]
fn verify_string_starts_with_str_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .starts_with("false start")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to start with "false start"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "false start"
"#
        ]
    );
}

#[test]
fn verify_string_starts_with_string_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .starts_with("false start".to_string())
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to start with "false start"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "false start"
"#
        ]
    );
}

#[test]
fn verify_string_starts_with_char_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .starts_with('X')
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing to start with 'X'
   but was: "possim deserunt obcaecat hendrerit"
  expected: 'X'
"#]
    );
}

#[test]
fn string_ends_with_str() {
    let subject: String = "wisi option excepteur labore".to_string();

    assert_that(subject).ends_with("labore");
}

#[test]
fn string_ends_with_string() {
    let subject: String = "sanctus stet eirmod voluptate".to_string();

    assert_that(subject).ends_with(" voluptate".to_string());
}

#[test]
fn string_ends_with_char() {
    let subject: String = "odio gubergren aliquip blandit".to_string();

    assert_that(subject).ends_with('t');
}

#[test]
fn str_ends_with_str() {
    let subject: &str = "stet nam consetetur placerat";

    assert_that(subject).ends_with("etur placerat");
}

#[test]
fn str_ends_with_string() {
    let subject: &str = "dolores invidunt exerci nostrud";

    assert_that(subject).ends_with("rud".to_string());
}

#[test]
fn str_ends_with_char() {
    let subject: &str = "odio gubergren aliquip blandit";

    assert_that(subject).ends_with('t');
}

#[test]
fn verify_string_ends_with_str_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .ends_with("abrupt end")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to end with "abrupt end"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "abrupt end"
"#
        ]
    );
}

#[test]
fn verify_string_ends_with_string_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .ends_with("abrupt end".to_string())
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"assertion failed: expected my_thing to end with "abrupt end"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "abrupt end"
"#
        ]
    );
}

#[test]
fn verify_string_ends_with_char_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .ends_with('Z')
        .display_failures();

    assert_eq!(
        failures,
        &[r#"assertion failed: expected my_thing to end with 'Z'
   but was: "possim deserunt obcaecat hendrerit"
  expected: 'Z'
"#]
    );
}

#[cfg(feature = "colored")]
mod colored {
    use crate::prelude::*;
    use crate::std::string::ToString;

    #[test]
    fn highlight_diffs_is_equal_to_for_strings() {
        let failures = verify_that("invidunt wisi facilisis exercitation")
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_equal_to("invi wisi exercitation anim placerat")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is equal to \"invi wisi exercitation anim placerat\"\n   \
                    but was: \"invi\u{1b}[31mdunt\u{1b}[0m wisi \u{1b}[31mfacilisis \u{1b}[0mexercitation\"\n  \
                   expected: \"invi wisi exercitation\u{1b}[34m anim placerat\u{1b}[0m\"\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_is_not_equal_to_for_strings() {
        let failures = verify_that("aute aliquip culpa blandit")
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_not_equal_to("aute aliquip culpa blandit")
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is not equal to \"aute aliquip culpa blandit\"\n   \
               but was: \"aute aliquip culpa blandit\"\n  \
              expected: \"aute aliquip culpa blandit\"\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_is_empty() {
        let subject = "voluptua quod quis dignissim";

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_empty()
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is empty\n   \
               but was: \u{1b}[31m\"voluptua quod quis dignissim\"\u{1b}[0m\n  \
              expected: <empty>\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_is_not_empty() {
        let subject = "";

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_not_empty()
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is not empty\n   \
               but was: \u{1b}[31m\"\"\u{1b}[0m\n  \
              expected: <non-empty>\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_has_length() {
        let subject = "feugiat mazim vero vero";

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .has_length(29)
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject has length 29\n   \
               but was: \u{1b}[31m23\u{1b}[0m\n  \
              expected: \u{1b}[32m29\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_has_length_in_range() {
        let subject = "dignissim nisl erat possim";

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .has_length_in_range(8..=20)
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject has length in range 8..=20\n   \
                   but was: \u{1b}[31m26\u{1b}[0m\n  \
                  expected: \u{1b}[32m8..=20\u{1b}[0m\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_contains_str() {
        let subject = "sanctus stet eiusmod odio".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .contains("status")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to contain \"status\"\n   \
                    but was: \"\u{1b}[31msanctus stet eiusmod odio\u{1b}[0m\"\n  \
                   expected: \"\u{1b}[33mstatus\u{1b}[0m\"\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_contains_string() {
        let subject = "sanctus stet eiusmod odio".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .contains("status".to_string())
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to contain \"status\"\n   \
                    but was: \"\u{1b}[31msanctus stet eiusmod odio\u{1b}[0m\"\n  \
                   expected: \"\u{1b}[33mstatus\u{1b}[0m\"\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_contains_char() {
        let subject = "sanctus stet eiusmod odio".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .contains('E')
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject to contain 'E'\n   \
                 but was: \"\u{1b}[31msanctus stet eiusmod odio\u{1b}[0m\"\n  \
                expected: '\u{1b}[33mE\u{1b}[0m'\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_starts_with_str() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .starts_with("una")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to start with \"una\"\n   \
                   but was: \"\u{1b}[31mnul\u{1b}[0mla feugiat illum culpa\"\n  \
                  expected: \"\u{1b}[32muna\u{1b}[0m\"\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_starts_with_string() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .starts_with("una".to_string())
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to start with \"una\"\n   \
                    but was: \"\u{1b}[31mnul\u{1b}[0mla feugiat illum culpa\"\n  \
                   expected: \"\u{1b}[32muna\u{1b}[0m\"\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_starts_with_char() {
        let subject = "commodo sadipscing id imperdiet".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .starts_with('o')
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject to start with 'o'\n   \
                   but was: \"\u{1b}[31mc\u{1b}[0mommodo sadipscing id imperdiet\"\n  \
                  expected: '\u{1b}[32mo\u{1b}[0m'\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_ends_with_str() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .ends_with("innocence")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to end with \"innocence\"\n   \
                   but was: \"nulla feugiat il\u{1b}[31mlum culpa\u{1b}[0m\"\n  \
                  expected: \"\u{1b}[32minnocence\u{1b}[0m\"\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_ends_with_string() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .ends_with("innocence".to_string())
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to end with \"innocence\"\n   \
                    but was: \"nulla feugiat il\u{1b}[31mlum culpa\u{1b}[0m\"\n  \
                   expected: \"\u{1b}[32minnocence\u{1b}[0m\"\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_ends_with_char() {
        let subject = "commodo sadipscing id imperdiet".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .ends_with('e')
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject to end with 'e'\n   \
                   but was: \"commodo sadipscing id imperdie\u{1b}[31mt\u{1b}[0m\"\n  \
                  expected: '\u{1b}[32me\u{1b}[0m'\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_contains_any_of_a_char_slice() {
        let subject = "proident tempor est sed".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .contains_any_of(&['a', 'b', 'c'][..])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to contain any of ['a', 'b', 'c']\n   \
                    but was: \u{1b}[31m\"proident tempor est sed\"\u{1b}[0m\n  \
                   expected: \u{1b}[34m['a', 'b', 'c']\u{1b}[0m\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_contains_any_of_a_char_array() {
        let subject = "proident tempor est sed".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .contains_any_of(['a', 'b', 'c'])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to contain any of ['a', 'b', 'c']\n   \
                    but was: \u{1b}[31m\"proident tempor est sed\"\u{1b}[0m\n  \
                   expected: \u{1b}[34m['a', 'b', 'c']\u{1b}[0m\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_contains_any_of_a_borrowed_char_array() {
        let subject = "proident tempor est sed".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .contains_any_of(&['a', 'b', 'c'])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject to contain any of ['a', 'b', 'c']\n   \
                    but was: \u{1b}[31m\"proident tempor est sed\"\u{1b}[0m\n  \
                   expected: \u{1b}[34m['a', 'b', 'c']\u{1b}[0m\n\
                "
            ]
        );
    }
}
