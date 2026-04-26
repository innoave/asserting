use crate::prelude::*;
use crate::std::string::{String, ToString};
use crate::std::vec;
use crate::std::vec::Vec;
#[cfg(feature = "bigdecimal")]
use bigdecimal::BigDecimal;
#[cfg(feature = "float-cmp")]
use time::OffsetDateTime;
#[cfg(feature = "float-cmp")]
use time::macros::datetime;

#[cfg(feature = "float-cmp")]
#[derive(Debug, Clone, PartialEq)]
struct Item {
    name: String,
    price: f32,
    quantity: u32,
}

#[cfg(feature = "float-cmp")]
struct Order {
    id: String,
    purchased_at: OffsetDateTime,
    items: Vec<Item>,
    vat: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Gender {
    Male,
    Female,
    NonBinary,
    PreferNotToSay,
}

struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

impl Person {
    fn name(&self) -> &str {
        &self.name
    }
}

struct Answer(i32);

#[test]
fn mapping_person_name_starts_with_alex() {
    let person = Person {
        name: "Alexander".to_string(),
        age: 31,
        gender: Gender::Male,
    };

    assert_that(person).mapping(|p| p.name).starts_with("Alex");
}

#[test]
fn extracting_person_name_contains_i() {
    let person = Person {
        name: "Silvia".to_string(),
        age: 27,
        gender: Gender::Female,
    };

    assert_that(person).extracting(|p| p.name).contains('i');
}

#[test]
fn extracting_ref_person_name_via_accessor_contains_via() {
    let person = Person {
        name: "Silvia".to_string(),
        age: 27,
        gender: Gender::Female,
    };

    assert_that(person)
        .extracting_ref("name", Person::name)
        .contains("via");
}

#[test]
fn extracting_ref_to_assert_all_person_fields() {
    let person = Person {
        name: "Silvia".to_string(),
        age: 27,
        gender: Gender::PreferNotToSay,
    };

    assert_that(person)
        .extracting_ref("name", |p| &p.name)
        .is_equal_to("Silvia")
        .and()
        .extracting_ref("age", |p| &p.age)
        .is_at_least(18)
        .and()
        .extracting_ref("gender", |p| &p.gender)
        .is_equal_to(Gender::PreferNotToSay);
}

#[test]
fn verify_extracting_ref_to_assert_all_fields_fails_with_all_failures() {
    let person = Person {
        name: "silvia".to_string(),
        age: 17,
        gender: Gender::NonBinary,
    };

    let failures = verify_that(person)
        .named("person")
        .extracting_ref("name", Person::name)
        .is_equal_to("Silvia")
        .and()
        .extracting_ref("age", |p| &p.age)
        .is_at_least(18)
        .and()
        .extracting_ref("gender", |p| &p.gender)
        .is_equal_to(Gender::PreferNotToSay)
        .display_failures();

    assert_eq!(
        failures,
        &[
            r#"expected person.name to be equal to "Silvia"
   but was: "silvia"
  expected: "Silvia"
"#,
            r"expected person.age to be at least 18
   but was: 17
  expected: >= 18
",
            r"expected person.gender to be equal to PreferNotToSay
   but was: NonBinary
  expected: PreferNotToSay
",
        ]
    );
}

#[cfg(feature = "float-cmp")]
#[test]
fn extracting_ref_to_assert_all_order_item_fields() {
    let order = Order {
        id: "019d359f-d2f1-7d64-826e-c111ae12dd24".to_string(),
        purchased_at: datetime!(2026-03-28 14:20:33 +01:00),
        items: vec![
            Item {
                name: "Apple".to_string(),
                price: 1.99,
                quantity: 6,
            },
            Item {
                name: "Orange".to_string(),
                price: 2.99,
                quantity: 3,
            },
        ],
        vat: 0.15,
    };

    assert_that(order)
        .extracting_ref("id", |o| &o.id)
        .is_not_empty()
        .and()
        .extracting_ref("purchased_at", |o| &o.purchased_at)
        .is_between(
            datetime!(2026-03-28 14:00 +01:00),
            datetime!(2026-03-28 15:00 +01:00),
        )
        .and()
        .extracting_ref("items", |o| &o.items)
        .has_length(2)
        .extracting_ref("[0]", |items| &items[0])
        .extracting_ref("name", |i| &i.name)
        .is_equal_to("Apple")
        .and()
        .extracting_ref("price", |i| &i.price)
        .is_close_to(1.99)
        .and()
        .extracting_ref("quantity", |i| &i.quantity)
        .is_equal_to(6)
        .and()
        .and()
        .contains_exactly([
            Item {
                name: "Apple".to_string(),
                price: 1.99,
                quantity: 6,
            },
            Item {
                name: "Orange".to_string(),
                price: 2.99,
                quantity: 3,
            },
        ])
        .and()
        .extracting_ref("vat", |o| &o.vat)
        .is_close_to(0.15);
}

#[test]
fn assert_that_extracted_ref_satisfies_predicate() {
    let answer = Answer(42);

    assert_that(answer)
        .named("answer")
        .extracting_ref("val", |answer| &answer.0)
        .satisfies(|actual| *actual == 42)
        .is_at_least(42);
}

#[test]
fn verify_that_subject_satisfies_predicate_fails() {
    let subject = Answer(51);

    let failures = verify_that(subject)
        .named("answer")
        .extracting_ref("val", |answer| &answer.0)
        .satisfies(|actual| *actual == 42)
        .display_failures();

    assert_eq!(
        failures,
        &["expected answer.val to satisfy the given predicate, but returned false\n"]
    );
}

#[test]
fn verify_that_subject_satisfies_predicate_fails_with_custom_message() {
    let subject = Answer(51);

    let failures = verify_that(subject)
        .named("answer")
        .extracting_ref("val", |answer| &answer.0)
        .satisfies_with_message("the answer to all important questions is 42", |actual| {
            *actual == 42
        })
        .display_failures();

    assert_eq!(failures, &["the answer to all important questions is 42\n"]);
}

#[test]
fn extracting_ref_string_is_equal_to() {
    struct Name(String);

    let name = Name("Alexander".to_string());

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .is_equal_to("Alexander");
}

#[test]
fn extracting_ref_string_is_same_as() {
    struct Name(String);

    let name = Name("Alexander".to_string());

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .is_same_as("Alexander".to_string());
}

#[test]
fn extracting_ref_i32_is_zero() {
    struct Int(i32);

    let number = Int(0);

    assert_that(number).extracting_ref("0", |n| &n.0).is_zero();
}

#[test]
fn extracting_ref_i32_is_one() {
    struct Int(i32);

    let number = Int(1);

    assert_that(number).extracting_ref("0", |n| &n.0).is_one();
}

#[test]
fn extracting_ref_i32_is_positive() {
    struct Int(i32);

    let number = Int(1);

    assert_that(number)
        .extracting_ref("0", |n| &n.0)
        .is_positive();
}

#[test]
fn extracting_ref_i32_is_negative() {
    struct Int(i32);

    let number = Int(-1);

    assert_that(number)
        .extracting_ref("0", |n| &n.0)
        .is_negative();
}

#[test]
fn extracting_ref_i32_is_not_positive_and_is_not_negative() {
    struct Int(i32);

    let number = Int(0);

    assert_that(number)
        .extracting_ref("0", |n| &n.0)
        .is_not_positive()
        .is_not_negative();
}

#[test]
fn extracting_ref_i32_is_in_range() {
    struct Int(i32);

    let number = Int(9);

    assert_that(number)
        .extracting_ref("0", |n| &n.0)
        .is_in_range(1..=9);
}

#[cfg(feature = "float-cmp")]
#[test]
fn extracting_ref_f32_is_close_to() {
    struct Float(f32);

    let value = Float(1.99);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_close_to(1.99);
}

#[test]
fn extracting_ref_f32_is_zero() {
    struct Float(f32);

    let value = Float(0.);

    assert_that(value).extracting_ref("0", |f| &f.0).is_zero();
}

#[test]
fn extracting_ref_f32_is_one() {
    struct Float(f32);

    let value = Float(1.);

    assert_that(value).extracting_ref("0", |f| &f.0).is_one();
}

#[test]
fn extracting_ref_f32_is_positive() {
    struct Float(f32);

    let value = Float(1.);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_positive();
}

#[test]
fn extracting_ref_f32_is_negative() {
    struct Float(f32);

    let value = Float(-1.);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_negative();
}

#[test]
fn extracting_ref_f32_is_not_positive_and_is_not_negative() {
    struct Float(f32);

    let value = Float(0.);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_not_positive()
        .is_not_negative();
}

#[test]
fn extracting_ref_f32_is_infinite() {
    struct Float(f32);

    let value = Float(f32::INFINITY);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_infinite();
}

#[test]
fn extracting_ref_f32_is_not_a_number() {
    struct Float(f32);

    let value = Float(f32::NAN);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_not_a_number();
}

#[cfg(feature = "float-cmp")]
#[test]
fn extracting_ref_f64_is_close_to_within_margin() {
    struct Float(f64);

    let value = Float(1.99);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_close_to_with_margin(1.99, (0.001, 2));
}

#[test]
fn extracting_ref_f64_is_zero() {
    struct Float(f64);

    let value = Float(0.);

    assert_that(value).extracting_ref("0", |f| &f.0).is_zero();
}

#[test]
fn extracting_ref_f64_is_one() {
    struct Float(f64);

    let value = Float(1.);

    assert_that(value).extracting_ref("0", |f| &f.0).is_one();
}

#[test]
fn extracting_ref_f64_is_positive() {
    struct Float(f64);

    let value = Float(1.);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_positive();
}

#[test]
fn extracting_ref_f64_is_negative() {
    struct Float(f64);

    let value = Float(-1.);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_negative();
}

#[test]
fn extracting_ref_f64_is_not_positive_and_is_not_negative() {
    struct Float(f64);

    let value = Float(0.);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_not_positive()
        .is_not_negative();
}

#[test]
fn extracting_ref_f64_is_infinite() {
    struct Float(f64);

    let value = Float(f64::INFINITY);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_infinite();
}

#[test]
fn extracting_ref_f64_is_not_a_number() {
    struct Float(f64);

    let value = Float(f64::NAN);

    assert_that(value)
        .extracting_ref("0", |f| &f.0)
        .is_not_a_number();
}

#[cfg(feature = "bigdecimal")]
#[test]
fn extracting_ref_bigdecimal_has_scale_of() {
    struct DecimalNumber(BigDecimal);

    let number = DecimalNumber(
        "23.99182405"
            .parse()
            .unwrap_or_else(|err| panic!("{}", err)),
    );

    assert_that(number)
        .extracting_ref("0", |n| &n.0)
        .has_scale_of(8);
}

#[cfg(feature = "bigdecimal")]
#[test]
fn extracting_ref_bigdecimal_has_precision_of() {
    struct DecimalNumber(BigDecimal);

    let number = DecimalNumber(
        "4123.99182405"
            .parse()
            .unwrap_or_else(|err| panic!("{}", err)),
    );

    assert_that(number)
        .extracting_ref("0", |n| &n.0)
        .has_precision_of(12);
}

#[cfg(feature = "bigdecimal")]
#[test]
fn extracting_ref_bigdecimal_is_integer() {
    struct DecimalNumber(BigDecimal);

    let number = DecimalNumber("123.0".parse().unwrap_or_else(|err| panic!("{}", err)));

    assert_that(number)
        .extracting_ref("0", |n| &n.0)
        .is_integer();
}

#[test]
fn extracting_ref_bool_is_true() {
    struct Flag(bool);

    assert_that(Flag(true))
        .extracting_ref("0", |f| &f.0)
        .is_true();
}

#[test]
fn extracting_ref_bool_is_false() {
    struct Flag(bool);

    assert_that(Flag(false))
        .extracting_ref("0", |f| &f.0)
        .is_false();
}

#[test]
fn extracting_ref_char_is_lowercase() {
    struct Character(char);

    assert_that(Character('r'))
        .extracting_ref("0", |c| &c.0)
        .is_lowercase();
}

#[test]
fn extracting_ref_char_is_uppercase() {
    struct Character(char);

    assert_that(Character('R'))
        .extracting_ref("0", |c| &c.0)
        .is_uppercase();
}

#[test]
fn extracting_ref_char_is_ascii() {
    struct Character(char);

    assert_that(Character('@'))
        .extracting_ref("0", |c| &c.0)
        .is_ascii();
}

#[test]
fn extracting_ref_char_is_alphabetic() {
    struct Character(char);

    assert_that(Character('Z'))
        .extracting_ref("0", |c| &c.0)
        .is_alphabetic();
}

#[test]
fn extracting_ref_char_is_alphanumeric() {
    struct Character(char);

    assert_that(Character('Z'))
        .extracting_ref("0", |c| &c.0)
        .is_alphanumeric();

    assert_that(Character('5'))
        .extracting_ref("0", |c| &c.0)
        .is_alphanumeric();
}

#[test]
fn extracting_ref_char_is_control_char() {
    struct Character(char);

    assert_that(Character('\t'))
        .extracting_ref("0", |c| &c.0)
        .is_control_char();

    assert_that(Character('\u{1b}'))
        .extracting_ref("0", |c| &c.0)
        .is_control_char();
}

#[test]
fn extracting_ref_char_is_digit() {
    struct Character(char);

    assert_that(Character('0'))
        .extracting_ref("0", |c| &c.0)
        .is_digit(10);
}

#[test]
fn extracting_ref_char_is_whitespace() {
    struct Character(char);

    assert_that(Character(' '))
        .extracting_ref("0", |c| &c.0)
        .is_whitespace();
    assert_that(Character('\n'))
        .extracting_ref("0", |c| &c.0)
        .is_whitespace();
}

#[test]
fn extracting_ref_string_is_empty() {
    struct Name(String);

    let name = Name(String::new());

    assert_that(name).extracting_ref("0", |n| &n.0).is_empty();
}

#[test]
fn extracting_ref_string_is_not_empty() {
    struct Name(String);

    let name = Name(" ".to_string());

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .is_not_empty();
}

#[test]
fn extracting_ref_vec_is_empty() {
    struct Bytes(Vec<u8>);

    let name = Bytes(vec![]);

    assert_that(name).extracting_ref("0", |n| &n.0).is_empty();
}

#[test]
fn extracting_ref_vec_is_not_empty() {
    struct Bytes(Vec<u8>);

    let name = Bytes(vec![48, 65]);

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .is_not_empty();
}

#[test]
fn extracting_ref_string_has_length() {
    struct Name(String);

    let name = Name("Alex".to_string());

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .has_length(4);
}

#[test]
fn extracting_ref_string_has_char_count() {
    struct Text(String);

    let name = Text("imper \u{0180} diet al \u{02AA} \u{01AF} zzril".to_string());

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .has_char_count(25);
}

#[test]
fn extracting_ref_option_some() {
    struct Optional(Option<String>);

    let note = Optional(Some("note".to_string()));

    assert_that(note).extracting_ref("0", |n| &n.0).is_some();
}

#[test]
fn extracting_ref_option_none() {
    struct Optional(Option<String>);

    let note = Optional(None);

    assert_that(note).extracting_ref("0", |n| &n.0).is_none();
}

#[test]
fn extracting_ref_option_some_is_equal_to() {
    struct Optional(Option<String>);

    let note = Optional(Some("a note".to_string()));

    assert_that(note)
        .extracting_ref("0", |n| &n.0)
        .some()
        .is_equal_to("a note");
}

#[test]
fn extracting_ref_option_has_value() {
    struct Optional(Option<String>);

    let note = Optional(Some("a note".to_string()));

    assert_that(note)
        .extracting_ref("0", |n| &n.0)
        .has_value("a note");
}

#[test]
fn extracting_ref_result_is_ok() {
    struct Response(Result<i32, String>);

    let response = Response(Ok(-123));

    assert_that(response).extracting_ref("0", |r| &r.0).is_ok();
}

#[test]
fn extracting_ref_result_is_err() {
    struct Response(Result<i32, String>);

    let response = Response(Err("not found".to_string()));

    assert_that(response).extracting_ref("0", |r| &r.0).is_err();
}

#[test]
fn extracting_ref_result_ok_is_negative() {
    struct Response(Result<i32, String>);

    let response = Response(Ok(-123));

    assert_that(response)
        .extracting_ref("0", |r| &r.0)
        .ok()
        .is_negative();
}

#[test]
fn extracting_ref_result_err_is_equal_to() {
    struct Response(Result<i32, String>);

    let response = Response(Err("not found".to_string()));

    assert_that(response)
        .extracting_ref("0", |r| &r.0)
        .err()
        .is_equal_to("not found");
}

#[test]
fn extracting_ref_result_has_value() {
    struct Response(Result<i32, String>);

    let response = Response(Ok(-123));

    assert_that(response)
        .extracting_ref("0", |r| &r.0)
        .has_value(-123);
}

#[test]
fn extracting_ref_result_has_error() {
    struct Response(Result<i32, String>);

    let response = Response(Err("not found".to_string()));

    assert_that(response)
        .extracting_ref("0", |r| &r.0)
        .has_error("not found");
}

#[test]
fn extracting_ref_string_contains_char() {
    struct Name(String);

    let name = Name("Alexander is here".to_string());

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .contains('x');
}

#[test]
fn extracting_ref_string_contains_any_of_chars() {
    struct Name(String);

    let name = Name("Alexander is here".to_string());

    assert_that(name)
        .extracting_ref("0", |n| &n.0)
        .contains_any_of(['a', 'e', 'i', 'o', 'u']);
}

#[test]
fn extracting_ref_vec_has_length() {
    struct Bytes(Vec<u8>);

    let bytes = Bytes(vec![1, 2, 3, 4, 5]);

    assert_that(bytes)
        .extracting_ref("0", |b| &b.0)
        .has_length(5);
}

#[test]
fn extracting_ref_vec_contains() {
    struct Names(Vec<String>);

    let names = Names(vec![
        "Silvia".to_string(),
        "Alexander".to_string(),
        "Robert".to_string(),
    ]);

    assert_that(names)
        .extracting_ref("0", |n| &n.0)
        .contains("Alexander");
}

#[test]
fn extracting_ref_vec_contains_exactly() {
    struct Names(Vec<String>);

    let names = Names(vec![
        "Silvia".to_string(),
        "Alexander".to_string(),
        "Robert".to_string(),
    ]);

    assert_that(names)
        .extracting_ref("0", |n| &n.0)
        .contains_exactly(["Silvia", "Alexander", "Robert"]);
}

#[test]
fn extracting_ref_vec_contains_only() {
    struct Names(Vec<String>);

    let names = Names(vec![
        "Silvia".to_string(),
        "Alexander".to_string(),
        "Robert".to_string(),
    ]);

    assert_that(names)
        .extracting_ref("0", |n| &n.0)
        .contains_only(["Silvia", "Robert", "Philipp", "Alexander"]);
}

#[test]
fn extracting_ref_vec_contains_any() {
    struct Names(Vec<String>);

    let names = Names(vec![
        "Silvia".to_string(),
        "Alexander".to_string(),
        "Robert".to_string(),
    ]);

    assert_that(names)
        .extracting_ref("0", |n| &n.0)
        .contains_any_of(["Robert", "Philipp", "Peter"]);
}

#[test]
fn extracting_ref_vec_contains_all() {
    struct Names(Vec<String>);

    let names = Names(vec![
        "Silvia".to_string(),
        "Alexander".to_string(),
        "Robert".to_string(),
    ]);

    assert_that(names)
        .extracting_ref("0", |n| &n.0)
        .contains_all_of(["Robert", "Silvia"]);
}

#[test]
fn extracting_ref_vec_contains_all_in_order() {
    struct Names(Vec<String>);

    let names = Names(vec![
        "Silvia".to_string(),
        "Alexander".to_string(),
        "Robert".to_string(),
    ]);

    assert_that(names)
        .extracting_ref("0", |n| &n.0)
        .contains_all_in_order(["Silvia", "Robert"]);
}

mod iterator_all_elements {
    use super::*;

    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u8,
    }

    struct People(Vec<Person>);

    struct Numbers(Vec<i32>);

    struct Words(Vec<&'static str>);

    #[test]
    fn assert_each_element_of_an_iterator_of_integer() {
        let subject = Numbers(vec![2, 4, 6, 8, 10]);

        assert_that(subject)
            .extracting_ref("0", |numbers| &numbers.0)
            .is_not_empty()
            .each_element(|e| e.is_positive().is_at_most(20));
    }

    #[test]
    fn assert_each_element_of_an_iterator_of_person() {
        let subject = People(vec![
            Person {
                name: "John".into(),
                age: 42,
            },
            Person {
                name: "Jane".into(),
                age: 20,
            },
        ]);

        assert_that(subject)
            .extracting_ref("0", |people| &people.0)
            .is_not_empty()
            .each_element(|person| {
                person
                    .extracting_ref("name", |p| &p.name)
                    .starts_with('J')
                    .and()
                    .extracting_ref("age", |p| &p.age)
                    .is_at_most(42)
            });
    }

    #[test]
    #[should_panic = "expected numbers.val[1] to be not equal to 4\n   but was: 4\n  expected: not 4\n"]
    fn assert_each_element_of_an_iterator_panics_if_one_assertion_fails() {
        let subject = Numbers(vec![2, 4, 6, 8, 10]);

        assert_that(subject)
            .named("numbers")
            .extracting_ref("val", |numbers| &numbers.0)
            .is_not_empty()
            .each_element(|e| e.is_not_equal_to(4));
    }

    #[test]
    fn verify_assert_each_element_of_an_iterator_fails() {
        let subject = Numbers(vec![2, 4, 6, 8, 10]);

        let failures = verify_that(&subject)
            .named("numbers")
            .extracting_ref("val", |numbers| &numbers.0)
            .each_element(|e| e.is_greater_than(2).is_at_most(7))
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected numbers.val[0] to be greater than 2
   but was: 2
  expected: > 2
",
                r"expected numbers.val[3] to be at most 7
   but was: 8
  expected: <= 7
",
                r"expected numbers.val[4] to be at most 7
   but was: 10
  expected: <= 7
",
            ]
        );
    }

    #[test]
    fn assert_any_element_of_an_iterator_of_str() {
        let subject = Words(vec!["one", "two", "three", "four", "five"]);

        assert_that(subject)
            .extracting_ref("0", |words| &words.0)
            .is_not_empty()
            .any_element(|e| e.contains("ee"));
    }

    #[test]
    fn assert_any_element_of_an_iterator_of_person() {
        let subject = People(vec![
            Person {
                name: "John".into(),
                age: 42,
            },
            Person {
                name: "Jane".into(),
                age: 20,
            },
        ]);

        assert_that(subject)
            .extracting_ref("0", |people| &people.0)
            .is_not_empty()
            .any_element(|person| {
                person
                    .extracting_ref("name", |p| &p.name)
                    .is_equal_to("John")
                    .and()
                    .extracting_ref("age", |p| &p.age)
                    .is_at_least(42)
            });
    }

    #[test]
    fn verify_any_element_of_an_iterator_asserting_two_properties_fails() {
        let subject = People(vec![
            Person {
                name: "John".into(),
                age: 42,
            },
            Person {
                name: "Jane".into(),
                age: 20,
            },
        ]);

        let failures = verify_that(subject)
            .named("people")
            .extracting_ref("0", |people| &people.0)
            .any_element(|person| {
                person
                    .extracting_ref("name", |p| &p.name)
                    .is_equal_to("John")
                    .and()
                    .extracting_ref("age", |p| &p.age)
                    .is_at_most(20)
            })
            .display_failures();

        assert_eq!(
            failures,
            &[
                r"expected people.0[0].age to be at most 20
   but was: 42
  expected: <= 20
",
                r#"expected people.0[1].name to be equal to "John"
   but was: "Jane"
  expected: "John"
"#
            ]
        );
    }

    #[test]
    fn verify_any_element_of_an_iterator_assertion_for_elements_fails() {
        let subject = Words(vec!["one", "two", "three", "four", "five"]);

        let failures = verify_that(subject)
            .named("words")
            .extracting_ref("0", |words| &words.0)
            .any_element(|e| e.starts_with("fu"))
            .display_failures();

        assert_eq!(
            failures,
            &[
                r#"expected words.0[0] to start with "fu"
   but was: "one"
  expected: "fu"
"#,
                r#"expected words.0[1] to start with "fu"
   but was: "two"
  expected: "fu"
"#,
                r#"expected words.0[2] to start with "fu"
   but was: "three"
  expected: "fu"
"#,
                r#"expected words.0[3] to start with "fu"
   but was: "four"
  expected: "fu"
"#,
                r#"expected words.0[4] to start with "fu"
   but was: "five"
  expected: "fu"
"#,
            ]
        );
    }
}

mod iterator_extracted_elements_ref {
    use super::*;

    #[allow(dead_code)]
    struct Order {
        id: u64,
        items: Vec<&'static str>,
    }

    #[test]
    fn first_element_of_iterator_with_one_element() {
        let order = Order {
            id: 55,
            items: vec!["Apple"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .first_element_ref()
            .is_equal_to("Apple")
            .has_length(5)
            .starts_with("App")
            .and()
            .last_element_ref()
            .is_equal_to("Apple");
    }

    #[test]
    fn first_element_of_iterator_with_several_elements() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Cherry", "Grapes", "Orange"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .first_element_ref()
            .is_equal_to("Apple")
            .has_length(5)
            .starts_with('A')
            .and()
            .last_element_ref()
            .is_equal_to("Orange");
    }

    #[cfg(feature = "panic")]
    #[test]
    fn first_element_of_iterator_with_no_elements_fails() {
        let order = Order {
            id: 55,
            items: vec![],
        };

        assert_that_code(|| {
            assert_that(order)
                .named("order")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .extracting_ref("items", |o| &o.items)
                .first_element_ref()
                .is_equal_to("Apple");
        })
        .panics_with_message(
            r"expected order.items to have at least one element, but has no elements
  actual: []
",
        );
    }

    #[test]
    fn verify_first_element_of_iterator_assertion_fails() {
        let order = Order {
            id: 55,
            items: vec!["Melon", "Banana", "Cherry", "Grapes", "Orange"],
        };

        let failures = verify_that(order)
            .named("order")
            .extracting_ref("items", |o| &o.items)
            .first_element_ref()
            .is_equal_to("Apple")
            .display_failures();

        assert_eq!(
            failures,
            &[
                r#"expected the first element of order.items to be equal to "Apple"
   but was: "Melon"
  expected: "Apple"
"#
            ]
        );
    }

    #[test]
    fn last_element_of_iterator_with_one_element() {
        let order = Order {
            id: 55,
            items: vec!["Apple"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .last_element_ref()
            .is_equal_to("Apple")
            .has_length(5)
            .starts_with("Ap")
            .and()
            .first_element_ref()
            .is_equal_to("Apple");
    }

    #[test]
    fn last_element_of_iterator_with_several_elements() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Cherry", "Grapes", "Orange"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .last_element_ref()
            .is_equal_to("Orange")
            .has_length(6)
            .starts_with("Oran")
            .and()
            .first_element_ref()
            .is_equal_to("Apple");
    }

    #[cfg(feature = "panic")]
    #[test]
    fn last_element_of_iterator_with_no_elements_fails() {
        let order = Order {
            id: 55,
            items: vec![],
        };

        assert_that_code(|| {
            assert_that(order)
                .named("order")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .extracting_ref("items", |o| &o.items)
                .last_element_ref()
                .is_equal_to("Grapes");
        })
        .panics_with_message(
            r"expected order.items to have at least one element, but has no elements
  actual: []
",
        );
    }

    #[test]
    fn verify_last_element_of_iterator_assertion_fails() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Melon"],
        };

        let failures = verify_that(order)
            .named("order")
            .extracting_ref("items", |o| &o.items)
            .last_element_ref()
            .is_equal_to("Cherry")
            .display_failures();

        assert_eq!(
            failures,
            &[
                r#"expected the last element of order.items to be equal to "Cherry"
   but was: "Melon"
  expected: "Cherry"
"#
            ]
        );
    }

    #[test]
    fn nth_element_of_iterator_with_one_element() {
        let order = Order {
            id: 55,
            items: vec!["Apple"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .nth_element_ref(0)
            .is_equal_to("Apple")
            .has_length(5)
            .starts_with("App")
            .and()
            .first_element_ref()
            .is_equal_to("Apple");
    }

    #[test]
    fn nth_element_of_iterator_with_several_elements_second_element() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Cherry", "Grapes", "Orange"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .nth_element_ref(1)
            .is_equal_to("Banana")
            .has_length(6)
            .starts_with("Ban")
            .and()
            .nth_element_ref(3)
            .is_equal_to("Grapes");
    }

    #[test]
    fn nth_element_of_iterator_with_several_elements_fifth_element() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Cherry", "Grapes", "Orange"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .nth_element_ref(4)
            .is_equal_to("Orange")
            .has_length(6)
            .starts_with("Or")
            .and()
            .nth_element_ref(0)
            .is_equal_to("Apple");
    }

    #[cfg(feature = "panic")]
    #[test]
    fn nth_element_of_iterator_with_five_elements_6th_element_fails() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Cherry", "Grapes", "Orange"],
        };

        assert_that_code(|| {
            assert_that(order)
                .named("order")
                .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
                .extracting_ref("items", |o| &o.items)
                .nth_element_ref(5)
                .is_equal_to("Melon");
        })
        .panics_with_message(
            r#"expected order.items to have at least 6 elements, but has 5 elements
  actual: ["Apple", "Banana", "Cherry", "Grapes", "Orange"]
"#,
        );
    }

    #[test]
    fn verify_nth_element_of_iterator_assertion_fails() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Melon", "Cherry", "Grapes", "Orange"],
        };

        let failures = verify_that(order)
            .named("order")
            .extracting_ref("items", |o| &o.items)
            .nth_element_ref(1)
            .is_equal_to("Banana")
            .display_failures();

        assert_eq!(
            failures,
            &[r#"expected order.items[1] to be equal to "Banana"
   but was: "Melon"
  expected: "Banana"
"#]
        );
    }

    #[test]
    fn elements_at_positions_of_iterator() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Cherry", "Grapes", "Orange"],
        };

        assert_that(order)
            .extracting_ref("items", |o| &o.items)
            .elements_ref_at([0, 2, 4])
            .contains_exactly(["Apple", "Cherry", "Orange"]);
    }

    #[test]
    fn verify_elements_at_positions_of_empty_iterator_fails() {
        let order = Order {
            id: 55,
            items: vec![],
        };

        let failures = verify_that(order)
            .named("order")
            .extracting_ref("items", |o| &o.items)
            .elements_ref_at([0, 1])
            .contains_exactly(["Apple", "Banana"])
            .display_failures();

        assert_eq!(
            failures,
            &[
                r#"expected order.items at positions [0, 1] to contain exactly in order ["Apple", "Banana"]
       but was: []
      expected: ["Apple", "Banana"]
       missing: ["Apple", "Banana"]
         extra: []
  out-of-order: []
"#
            ]
        );
    }

    #[test]
    fn verify_elements_at_out_of_bounds_position_fails() {
        let order = Order {
            id: 55,
            items: vec!["Apple", "Banana", "Cherry"],
        };

        let failures = verify_that(order)
            .named("order")
            .extracting_ref("items", |o| &o.items)
            .elements_ref_at([0, 3])
            .contains_exactly(["Apple", "Grapes"])
            .display_failures();

        assert_eq!(
            failures,
            &[
                r#"expected order.items at positions [0, 3] to contain exactly in order ["Apple", "Grapes"]
       but was: ["Apple"]
      expected: ["Apple", "Grapes"]
       missing: ["Grapes"]
         extra: []
  out-of-order: []
"#
            ]
        );
    }
}
