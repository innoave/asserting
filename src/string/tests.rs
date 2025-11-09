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
            r#"expected my_thing to be equal to "aute duis eleifend molestie"
   but was: ""
  expected: "aute duis eleifend molestie"
"#
        ]
    );
}

#[test]
fn string_is_same_as_string() {
    let subject: String = "aliqua esse consectetur ullamcorper".to_string();

    assert_that(subject).is_same_as("aliqua esse consectetur ullamcorper".to_string());
}

#[test]
fn string_ref_is_same_as_string_ref() {
    let subject: &String = &"adipiscing liber esse anim".to_string();

    assert_that(subject).is_same_as(&"adipiscing liber esse anim".to_string());
}

#[test]
fn str_is_same_as_str() {
    let subject: &str = "diam accusam tation luptatum";

    assert_that(subject).is_same_as("diam accusam tation luptatum");
}

#[test]
fn verify_string_is_same_as_string_fails() {
    let failures = verify_that("diam accusam tation luptatum".to_string())
        .named("my_text")
        .is_same_as("diam accusam Tation luptatum".to_string())
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected my_text to be the same as "diam accusam Tation luptatum"
   but was: "diam accusam tation luptatum"
  expected: "diam accusam Tation luptatum"
"#
        ]
    );
}

#[test]
fn string_is_not_same_as_string() {
    let subject: String = "aliqua esse consectetur ullamcorper".to_string();

    assert_that(subject).is_not_same_as("Aliqua esse consectetur ullamcorper".to_string());
}

#[test]
fn string_ref_is_not_same_as_string_ref() {
    let subject: &String = &"adipiscing liber esse anim".to_string();

    assert_that(subject).is_not_same_as(&"adipiscing liber rese anim".to_string());
}

#[test]
fn str_is_not_same_as_str() {
    let subject: &str = "Diam accusam tation luptatum";

    assert_that(subject).is_not_same_as("diam accusam tation luptatum");
}

#[test]
fn verify_str_is_not_same_as_str_fails() {
    let failures = verify_that("diam accusam tation luptatum")
        .named("my_text")
        .is_not_same_as("diam accusam tation luptatum")
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected my_text to be not the same as "diam accusam tation luptatum"
   but was: "diam accusam tation luptatum"
  expected: not "diam accusam tation luptatum"
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
        &[r#"expected my_thing to be empty
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
        &[r#"expected my_thing to be not empty
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
        &[r"expected my_thing to have a length of 29
   but was: 28
  expected: 29
"]
    );
}

#[test]
fn string_has_length_in_range() {
    let subject: String = "fugiat vero cillum dolore".to_string();

    assert_that(subject).has_length_in_range(1..26);
}

#[test]
fn verify_has_length_in_range_fails() {
    let subject: String = "fugiat vero cillum dolore".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_length_in_range(1..25)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have a length within range 1..25
   but was: 25
  expected: 1..25
"]
    );
}

#[test]
fn string_has_length_in_inclusive_range() {
    let subject: String = "fugiat vero cillum dolore".to_string();

    assert_that(subject).has_length_in_range(1..=25);
}

#[test]
fn verify_has_length_in_inclusive_range_fails() {
    let subject: String = "fugiat vero cillum dolore".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_length_in_range(1..=24)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have a length within range 1..=24
   but was: 25
  expected: 1..=24
"]
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
        &[r"expected my_thing to have a length less than 25
   but was: 25
  expected: < 25
"]
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
        &[r"expected my_thing to have a length greater than 28
   but was: 28
  expected: > 28
"]
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
        &[r"expected my_thing to have at most a length of 29
   but was: 30
  expected: <= 29
"]
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
        &[r"expected my_thing to have at least a length of 22
   but was: 21
  expected: >= 22
"]
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
        &[r"expected my_thing to have a char count of 7
   but was: 5
  expected: 7
"]
    );
}

#[test]
fn string_has_char_count_in_range() {
    let subject: String = "\u{0112} \u{0034} \u{0200}".to_string();

    assert_that(subject).has_char_count_in_range(5..6);
}

#[test]
fn string_has_char_count_in_inclusive_range() {
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

    assert_that(subject).has_char_count_in_range(5..6);
}

#[test]
fn verify_str_has_char_count_in_range_fails() {
    let subject: &str = "\u{0112} \u{0034} \u{0200}";

    let failures = verify_that(subject)
        .named("my_thing")
        .has_char_count_in_range(6..12)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have a char count within 6..12
   but was: 5
  expected: 6..12
"]
    );
}

#[test]
fn str_has_char_count_in_inclusive_range() {
    let subject: &str = "\u{0112} \u{0034} \u{0200}";

    assert_that(subject).has_char_count_in_range(5..=5);
}

#[test]
fn verify_str_has_char_count_in_inclusive_range_fails() {
    let subject: &str = "\u{0112} \u{0034} \u{0200}";

    let failures = verify_that(subject)
        .named("my_thing")
        .has_char_count_in_range(6..=12)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have a char count within 6..=12
   but was: 5
  expected: 6..=12
"]
    );
}

#[test]
fn string_has_char_count_less_than() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    assert_that(subject).has_char_count_less_than(8);
}

#[test]
fn verify_string_has_char_count_less_than_fails() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_char_count_less_than(7)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have a char count less than 7
   but was: 7
  expected: < 7
"]
    );
}

#[test]
fn string_has_char_count_greater_than() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    assert_that(subject).has_char_count_greater_than(6);
}

#[test]
fn verify_string_has_char_count_greater_than_fails() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_char_count_greater_than(7)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have a char count greater than 7
   but was: 7
  expected: > 7
"]
    );
}

#[test]
fn string_has_at_most_char_count() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    assert_that(&subject).has_at_most_char_count(7);
    assert_that(subject).has_at_most_char_count(8);
}

#[test]
fn verify_string_has_at_most_char_count_fails() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_at_most_char_count(6)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have at most a char count of 6
   but was: 7
  expected: <= 6
"]
    );
}

#[test]
fn string_has_at_least_char_count() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    assert_that(&subject).has_at_least_char_count(7);
    assert_that(subject).has_at_least_char_count(6);
}

#[test]
fn verify_string_has_at_least_char_count_fails() {
    let subject: String = "\u{0112} \u{0034} \u{0200} \u{01BE}".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .has_at_least_char_count(8)
        .display_failures();

    assert_eq!(
        failures,
        &[r"expected my_thing to have at least a char count of 8
   but was: 7
  expected: >= 8
"]
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
        &[r#"expected my_thing to contain "not a substring"
   but was: "invidunt eos hendrerit commodo"
  expected: "not a substring"
"#]
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
        &[r#"expected my_thing to contain "not a substring"
   but was: "invidunt eos hendrerit commodo"
  expected: "not a substring"
"#]
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
        &[r#"expected my_thing to contain 'Q'
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
        &[r#"expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#]
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
        &[r#"expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#]
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
        &[r#"expected my_thing to contain any of ['x', 'y', 'z']
   but was: "luptatum in nihil laoreet"
  expected: ['x', 'y', 'z']
"#]
    );
}

#[test]
fn string_does_not_contain_other_str() {
    let subject: String = "illum kasd nostrud possim".to_string();

    assert_that(subject).does_not_contain("laboris");
}

#[test]
fn string_does_not_contain_other_string() {
    let subject: String = "consectetuer nulla anim nihil".to_string();

    assert_that(subject).does_not_contain("doming".to_string());
}

#[test]
fn str_does_not_contain_other_str() {
    let subject: &str = "consectetuer duis quis veniam";

    assert_that(subject).does_not_contain("duis veniam");
}

#[test]
fn str_does_not_contain_other_string() {
    let subject: &str = "voluptua liber assum facilisis";

    assert_that(subject).does_not_contain("tue liber assum".to_string());
}

#[test]
fn str_does_not_contain_a_char() {
    let subject: &str = "praesent doming liber accusam";

    assert_that(subject).does_not_contain('v');
}

#[test]
fn verify_string_does_not_contain_other_str_fails() {
    let subject: String = "invidunt eos hendrerit commodo".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain(" eos ")
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not contain " eos "
   but was: "invidunt eos hendrerit commodo"
  expected: not " eos "
"#]
    );
}

#[test]
fn verify_string_does_not_contain_other_string_fails() {
    let subject: String = "invidunt eos hendrerit commodo".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain("eos hend".to_string())
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not contain "eos hend"
   but was: "invidunt eos hendrerit commodo"
  expected: not "eos hend"
"#]
    );
}

#[test]
fn verify_string_does_not_contain_char_fails() {
    let subject: String = "consectetur ex hendrerit officia".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain('x')
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not contain 'x'
   but was: "consectetur ex hendrerit officia"
  expected: not 'x'
"#]
    );
}

#[test]
fn string_does_not_contain_any_char_of_a_slice_of_chars() {
    let subject: String = "dolore reprehenderit erat duis".to_string();

    assert_that(subject).does_not_contain_any_of(&['v', 'm', 'z', 'b'][..]);
}

#[test]
fn str_does_not_contain_any_char_of_an_array_of_chars() {
    let subject: &str = "duo excepteur invidunt nonumy";

    assert_that(subject).does_not_contain_any_of(['b', 'a', 'z']);
}

#[test]
fn string_does_not_contain_any_char_of_a_borrowed_array_of_chars() {
    let subject: String = "sadipscing nibh nisi voluptua".to_string();

    assert_that(subject).does_not_contain_any_of(&['q', 'x', 'k', 'm', 'r']);
}

#[test]
fn verify_str_does_not_contain_any_char_of_a_slice_of_chars_fails() {
    let subject: &str = "luptatum in nihil laoreet";

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain_any_of(&['x', 'n', 'z'][..])
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not contain any of ['x', 'n', 'z']
   but was: "luptatum in nihil laoreet"
  expected: not ['x', 'n', 'z']
"#]
    );
}

#[test]
fn verify_string_does_not_contain_any_char_of_an_array_of_chars_fails() {
    let subject: String = "luptatum in nihil laoreet".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain_any_of(['d', 'k', 'n'])
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not contain any of ['d', 'k', 'n']
   but was: "luptatum in nihil laoreet"
  expected: not ['d', 'k', 'n']
"#]
    );
}

#[test]
fn verify_str_does_not_contain_any_char_of_a_borrowed_array_of_chars_fails() {
    let subject: &str = "luptatum in nihil laoreet";

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_contain_any_of(&['u', 'a', 'l'])
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not contain any of ['u', 'a', 'l']
   but was: "luptatum in nihil laoreet"
  expected: not ['u', 'a', 'l']
"#]
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
        &[r#"expected my_thing to start with "false start"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "false start"
"#]
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
        &[r#"expected my_thing to start with "false start"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "false start"
"#]
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
        &[r#"expected my_thing to start with 'X'
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
        &[r#"expected my_thing to end with "abrupt end"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "abrupt end"
"#]
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
        &[r#"expected my_thing to end with "abrupt end"
   but was: "possim deserunt obcaecat hendrerit"
  expected: "abrupt end"
"#]
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
        &[r#"expected my_thing to end with 'Z'
   but was: "possim deserunt obcaecat hendrerit"
  expected: 'Z'
"#]
    );
}

#[test]
fn string_does_not_start_with_str() {
    let subject: String = "wisi option excepteur labore".to_string();

    assert_that(subject).does_not_start_with("vidi");
}

#[test]
fn string_does_not_start_with_string() {
    let subject: String = "sanctus stet eirmod voluptate".to_string();

    assert_that(subject).does_not_start_with("sandusen ".to_string());
}

#[test]
fn string_does_not_start_with_char() {
    let subject: String = "odio gubergren aliquip blandit".to_string();

    assert_that(subject).does_not_start_with('v');
}

#[test]
fn str_does_not_start_with_str() {
    let subject: &str = "stet nam consetetur placerat";

    assert_that(subject).does_not_start_with("stet ma");
}

#[test]
fn str_does_not_start_with_string() {
    let subject: &str = "dolores invidunt exerci nostrud";

    assert_that(subject).does_not_start_with("color".to_string());
}

#[test]
fn str_does_not_start_with_char() {
    let subject: &str = "odio gubergren aliquip blandit";

    assert_that(subject).does_not_start_with('m');
}

#[test]
fn verify_string_does_not_start_with_str_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_start_with("possim des")
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not start with "possim des"
   but was: "possim deserunt obcaecat hendrerit"
  expected: not "possim des"
"#]
    );
}

#[test]
fn verify_string_does_not_start_with_string_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_start_with("poss".to_string())
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not start with "poss"
   but was: "possim deserunt obcaecat hendrerit"
  expected: not "poss"
"#]
    );
}

#[test]
fn verify_string_does_not_start_with_char_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_start_with('p')
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not start with 'p'
   but was: "possim deserunt obcaecat hendrerit"
  expected: not 'p'
"#]
    );
}

#[test]
fn string_does_not_end_with_str() {
    let subject: String = "wisi option excepteur labore".to_string();

    assert_that(subject).does_not_end_with("libory");
}

#[test]
fn string_does_not_end_with_string() {
    let subject: String = "sanctus stet eirmod voluptate".to_string();

    assert_that(subject).does_not_end_with(" volerate".to_string());
}

#[test]
fn string_does_not_end_with_char() {
    let subject: String = "odio gubergren aliquip blandit".to_string();

    assert_that(subject).does_not_end_with('i');
}

#[test]
fn str_does_not_end_with_str() {
    let subject: &str = "stet nam consetetur placerat";

    assert_that(subject).does_not_end_with("etur benerat");
}

#[test]
fn str_does_not_end_with_string() {
    let subject: &str = "dolores invidunt exerci nostrud";

    assert_that(subject).does_not_end_with("tru".to_string());
}

#[test]
fn str_does_not_end_with_char() {
    let subject: &str = "odio gubergren aliquip blandit";

    assert_that(subject).does_not_end_with('v');
}

#[test]
fn verify_string_does_not_end_with_str_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_end_with("rerit")
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not end with "rerit"
   but was: "possim deserunt obcaecat hendrerit"
  expected: not "rerit"
"#]
    );
}

#[test]
fn verify_string_does_not_end_with_string_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_end_with("caecat hendrerit".to_string())
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not end with "caecat hendrerit"
   but was: "possim deserunt obcaecat hendrerit"
  expected: not "caecat hendrerit"
"#]
    );
}

#[test]
fn verify_string_does_not_end_with_char_fails() {
    let subject: String = "possim deserunt obcaecat hendrerit".to_string();

    let failures = verify_that(subject)
        .named("my_thing")
        .does_not_end_with('t')
        .display_failures();

    assert_eq!(
        failures,
        &[r#"expected my_thing to not end with 't'
   but was: "possim deserunt obcaecat hendrerit"
  expected: not 't'
"#]
    );
}

#[cfg(feature = "regex")]
mod regex {
    use crate::prelude::*;

    #[test]
    fn string_matches_regex() {
        let subject: String = "tincidunt laoreet molestie eros".to_string();

        assert_that(subject).matches(r"\b\w{8}\b");
    }

    #[test]
    fn verify_string_matches_regex_fails() {
        let subject: String = "volutpat lobortis aliquam diam".to_string();

        let failures = verify_that(subject)
            .named("my_thing")
            .matches(r"\b\w{12}\b")
            .display_failures();

        assert_eq!(
            failures,
            &[r"expected my_thing to match the regex \b\w{12}\b
               but was: volutpat lobortis aliquam diam
  does not match regex: \b\w{12}\b
"]
        );
    }

    #[test]
    fn string_does_not_match_regex() {
        let subject: String = "tincidunt\tLaoreet ❤ Molestie eros".to_string();

        assert_that(subject).does_not_match(r"^[a-zA-Z0-9 ]{8,32}$");
    }

    #[test]
    fn verify_string_does_not_match_regex_fails() {
        let subject: String = "volutpat lobortis aliquam diam".to_string();

        let failures = verify_that(subject)
            .named("my_thing")
            .does_not_match(r"^[a-zA-Z0-9 ]{8,32}$")
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected my_thing to not match the regex ^[a-zA-Z0-9 ]{8,32}$
               but was: volutpat lobortis aliquam diam
      does match regex: ^[a-zA-Z0-9 ]{8,32}$
"
            ]
        );
    }

    #[test]
    #[should_panic = r"failed to match string with regex: regex parse error:
    ^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,32}$
     ^^^
error: look-around, including look-ahead and look-behind, is not supported"]
    fn string_matches_regex_given_an_invalid_regex_panics() {
        let subject: String = "s3cr3tPass".to_string();

        assert_that(subject)
            .named("password")
            .matches(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,32}$");
    }

    #[test]
    #[should_panic = "failed to match string with regex: Compiled regex exceeds size limit of 10485760 bytes"]
    fn string_matches_regex_given_regex_exceeds_default_size_limit_panics() {
        let subject: String = "s3cr3tPass".to_string();

        assert_that(subject)
            .named("my_thing")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .matches(r"^(\/[\w-]{1,255}){1,64}\/?$");
    }
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
                "expected subject to be equal to \"invi wisi exercitation anim placerat\"\n   \
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
            &[
                "expected subject to be not equal to \"aute aliquip culpa blandit\"\n   \
               but was: \"aute aliquip culpa blandit\"\n  \
              expected: not \"aute aliquip culpa blandit\"\n\
            "
            ]
        );
    }

    #[test]
    fn highlight_diffs_is_same_as_for_strings() {
        let failures = verify_that("no fugiat pariatur placerat")
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_same_as("no Fugiat Pariatur placerat")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to be the same as \"no Fugiat Pariatur placerat\"\n   \
                    but was: \"no \u{1b}[31mf\u{1b}[0mugiat \u{1b}[31mp\u{1b}[0mariatur placerat\"\n  \
                   expected: \"no \u{1b}[34mF\u{1b}[0mugiat \u{1b}[34mP\u{1b}[0mariatur placerat\"\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_is_not_same_as_for_strings() {
        let failures = verify_that("justo clita in stet")
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .is_not_same_as("justo clita in stet")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to be not the same as \"justo clita in stet\"\n   \
               but was: \"justo clita in stet\"\n  \
              expected: not \"justo clita in stet\"\n\
            "
            ]
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
            &["expected subject to be empty\n   \
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
            &["expected subject to be not empty\n   \
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
            &["expected subject to have a length of 29\n   \
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
            &["expected subject to have a length within range 8..=20\n   \
                   but was: \u{1b}[31m26\u{1b}[0m\n  \
                  expected: \u{1b}[32m8..=20\u{1b}[0m\n\
            "]
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
            &["expected subject to contain \"status\"\n   \
                    but was: \"\u{1b}[31msanctus stet eiusmod odio\u{1b}[0m\"\n  \
                   expected: \"\u{1b}[33mstatus\u{1b}[0m\"\n\
                "]
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
            &["expected subject to contain \"status\"\n   \
                    but was: \"\u{1b}[31msanctus stet eiusmod odio\u{1b}[0m\"\n  \
                   expected: \"\u{1b}[33mstatus\u{1b}[0m\"\n\
                "]
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
            &["expected subject to contain 'E'\n   \
                 but was: \"\u{1b}[31msanctus stet eiusmod odio\u{1b}[0m\"\n  \
                expected: '\u{1b}[33mE\u{1b}[0m'\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_contain_str() {
        let subject = "sanctus stet eiusmod odio".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .does_not_contain("stet eius")
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not contain \"stet eius\"\n   \
                    but was: \"sanctus \u{1b}[31mstet eius\u{1b}[0mmod odio\"\n  \
                   expected: not \"\u{1b}[33mstet eius\u{1b}[0m\"\n\
                "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_contain_string() {
        let subject = "sanctus stet eiusmod odio".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .does_not_contain("stet eius".to_string())
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not contain \"stet eius\"\n   \
                    but was: \"sanctus \u{1b}[31mstet eius\u{1b}[0mmod odio\"\n  \
                   expected: not \"\u{1b}[33mstet eius\u{1b}[0m\"\n\
                "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_contain_char() {
        let subject = "sanctus stett eiusmod odio".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .does_not_contain('t')
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not contain 't'\n   \
                 but was: \"sanc\u{1b}[31mt\u{1b}[0mus s\u{1b}[31mt\u{1b}[0me\u{1b}[31mtt\u{1b}[0m eiusmod odio\"\n  \
                expected: not '\u{1b}[33mt\u{1b}[0m'\n\
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
            &["expected subject to start with \"una\"\n   \
                   but was: \"\u{1b}[31mnul\u{1b}[0mla feugiat illum culpa\"\n  \
                  expected: \"\u{1b}[32muna\u{1b}[0m\"\n\
            "]
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
            &["expected subject to start with \"una\"\n   \
                    but was: \"\u{1b}[31mnul\u{1b}[0mla feugiat illum culpa\"\n  \
                   expected: \"\u{1b}[32muna\u{1b}[0m\"\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_start_with_str() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_start_with("null")
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not start with \"null\"\n   \
                   but was: \"\u{1b}[31mnull\u{1b}[0ma feugiat illum culpa\"\n  \
                  expected: not \"\u{1b}[32mnull\u{1b}[0m\"\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_start_with_string() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_start_with("null".to_string())
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not start with \"null\"\n   \
                    but was: \"\u{1b}[31mnull\u{1b}[0ma feugiat illum culpa\"\n  \
                   expected: not \"\u{1b}[32mnull\u{1b}[0m\"\n\
            "]
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
            &["expected subject to start with 'o'\n   \
                   but was: \"\u{1b}[31mc\u{1b}[0mommodo sadipscing id imperdiet\"\n  \
                  expected: '\u{1b}[32mo\u{1b}[0m'\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_start_with_char() {
        let subject = "commodo sadipscing id imperdiet".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_start_with('c')
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not start with 'c'\n   \
                   but was: \"\u{1b}[31mc\u{1b}[0mommodo sadipscing id imperdiet\"\n  \
                  expected: not '\u{1b}[32mc\u{1b}[0m'\n\
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
            &["expected subject to end with \"innocence\"\n   \
                   but was: \"nulla feugiat il\u{1b}[31mlum culpa\u{1b}[0m\"\n  \
                  expected: \"\u{1b}[32minnocence\u{1b}[0m\"\n\
            "]
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
            &["expected subject to end with \"innocence\"\n   \
                    but was: \"nulla feugiat il\u{1b}[31mlum culpa\u{1b}[0m\"\n  \
                   expected: \"\u{1b}[32minnocence\u{1b}[0m\"\n\
                "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_end_with_str() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_end_with("um culpa")
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not end with \"um culpa\"\n   \
                   but was: \"nulla feugiat ill\u{1b}[31mum culpa\u{1b}[0m\"\n  \
                  expected: not \"\u{1b}[32mum culpa\u{1b}[0m\"\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_end_with_string() {
        let subject = "nulla feugiat illum culpa".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_end_with("lpa".to_string())
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not end with \"lpa\"\n   \
                    but was: \"nulla feugiat illum cu\u{1b}[31mlpa\u{1b}[0m\"\n  \
                   expected: not \"\u{1b}[32mlpa\u{1b}[0m\"\n\
                "]
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
            &["expected subject to end with 'e'\n   \
                   but was: \"commodo sadipscing id imperdie\u{1b}[31mt\u{1b}[0m\"\n  \
                  expected: '\u{1b}[32me\u{1b}[0m'\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_end_with_char() {
        let subject = "commodo sadipscing id imperdiet".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .does_not_end_with('t')
            .display_failures();

        assert_eq!(
            failures,
            &["expected subject to not end with 't'\n   \
                   but was: \"commodo sadipscing id imperdie\u{1b}[31mt\u{1b}[0m\"\n  \
                  expected: not '\u{1b}[32mt\u{1b}[0m'\n\
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
            &["expected subject to contain any of ['a', 'b', 'c']\n   \
                    but was: \"\u{1b}[31mproident tempor est sed\u{1b}[0m\"\n  \
                   expected: \u{1b}[34m['a', 'b', 'c']\u{1b}[0m\n\
                "]
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
            &["expected subject to contain any of ['a', 'b', 'c']\n   \
                    but was: \"\u{1b}[31mproident tempor est sed\u{1b}[0m\"\n  \
                   expected: \u{1b}[34m['a', 'b', 'c']\u{1b}[0m\n\
                "]
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
            &["expected subject to contain any of ['a', 'b', 'c']\n   \
                    but was: \"\u{1b}[31mproident tempor est sed\u{1b}[0m\"\n  \
                   expected: \u{1b}[34m['a', 'b', 'c']\u{1b}[0m\n\
                "]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_contain_any_of_a_char_slice() {
        let subject = "proident tempor est sed".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .does_not_contain_any_of(&['r', 'b', 'c'][..])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to not contain any of ['r', 'b', 'c']\n   \
                    but was: \"p\u{1b}[31mr\u{1b}[0moident tempo\u{1b}[31mr\u{1b}[0m est sed\"\n  \
                   expected: not [\u{1b}[34m'r'\u{1b}[0m, 'b', 'c']\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_contain_any_of_a_char_array() {
        let subject = "proident tempor est sed".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .does_not_contain_any_of(['a', 's', 'e'])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to not contain any of ['a', 's', 'e']\n   \
                    but was: \"proid\u{1b}[31me\u{1b}[0mnt t\u{1b}[31me\u{1b}[0mmpor \u{1b}[31mes\u{1b}[0mt \u{1b}[31mse\u{1b}[0md\"\n  \
                   expected: not ['a', \u{1b}[34m's'\u{1b}[0m, \u{1b}[34m'e'\u{1b}[0m]\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_contain_any_of_a_borrowed_char_array() {
        let subject = "proident tempor est sed".to_string();

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_BLUE)
            .does_not_contain_any_of(&['p', 'o', 'r'])
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected subject to not contain any of ['p', 'o', 'r']\n   \
                    but was: \"\u{1b}[31mpro\u{1b}[0mident tem\u{1b}[31mpor\u{1b}[0m est sed\"\n  \
                   expected: not [\u{1b}[34m'p'\u{1b}[0m, \u{1b}[34m'o'\u{1b}[0m, \u{1b}[34m'r'\u{1b}[0m]\n\
                "
            ]
        );
    }
}

#[cfg(all(feature = "colored", feature = "regex"))]
mod colored_regex {
    use crate::prelude::*;
    use crate::std::string::ToString;

    #[test]
    fn highlight_diffs_string_matches_regex() {
        let subject: String = "volutpat lobortis aliquam diam".to_string();

        let failures = verify_that(subject)
            .named("my_thing")
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .matches(r"\b\w{12}\b")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected my_thing to match the regex \\b\\w{12}\\b\n               \
                                but was: \u{1b}[31mvolutpat lobortis aliquam diam\u{1b}[0m\n  \
                   does not match regex: \u{1b}[32m\\b\\w{12}\\b\u{1b}[0m\n\
                "
            ]
        );
    }

    #[test]
    fn highlight_diffs_string_does_not_match_regex() {
        let subject: String = "volutpat lobortis aliquam diam".to_string();

        let failures = verify_that(subject)
            .named("my_thing")
            .with_diff_format(DIFF_FORMAT_RED_YELLOW)
            .does_not_match(r"^[a-zA-Z0-9 ]{8,32}$")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "expected my_thing to not match the regex ^[a-zA-Z0-9 ]{8,32}$\n               \
                             but was: \u{1b}[31mvolutpat lobortis aliquam diam\u{1b}[0m\n      \
                    does match regex: \u{1b}[33m^[a-zA-Z0-9 ]{8,32}$\u{1b}[0m\n\
                "
            ]
        );
    }
}
