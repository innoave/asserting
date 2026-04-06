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
        .extracting_ref("person.name", Person::name)
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
        .extracting_ref("person.name", |p| &p.name)
        .is_equal_to("Silvia")
        .and()
        .extracting_ref("person.age", |p| &p.age)
        .is_at_least(18)
        .and()
        .extracting_ref("person.gender", |p| &p.gender)
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
        .extracting_ref("person.name", Person::name)
        .is_equal_to("Silvia")
        .and()
        .extracting_ref("person.age", |p| &p.age)
        .is_at_least(18)
        .and()
        .extracting_ref("person.gender", |p| &p.gender)
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
        .extracting_ref("order.id", |o| &o.id)
        .is_not_empty()
        .and()
        .extracting_ref("order.purchased_at", |o| &o.purchased_at)
        .is_between(
            datetime!(2026-03-28 14:00 +01:00),
            datetime!(2026-03-28 15:00 +01:00),
        )
        .and()
        .extracting_ref("order.items", |o| &o.items)
        .has_length(2)
        .extracting_ref("order.items[0]", |items| &items[0])
        .extracting_ref("order.items[0].name", |i| &i.name)
        .is_equal_to("Apple")
        .and()
        .extracting_ref("order.items[0].price", |i| &i.price)
        .is_close_to(1.99)
        .and()
        .extracting_ref("order.items[0].quantity", |i| &i.quantity)
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
        .extracting_ref("order.vat", |o| &o.vat)
        .is_close_to(0.15);
}

#[test]
fn extracting_ref_string_is_equal_to() {
    struct Name(String);

    let name = Name("Alexander".to_string());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .is_equal_to("Alexander");
}

#[test]
fn extracting_ref_string_is_same_as() {
    struct Name(String);

    let name = Name("Alexander".to_string());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .is_same_as("Alexander".to_string());
}

#[test]
fn extracting_ref_i32_is_zero() {
    struct Int(i32);

    let number = Int(0);

    assert_that(number)
        .extracting_ref("number.0", |n| &n.0)
        .is_zero();
}

#[test]
fn extracting_ref_i32_is_one() {
    struct Int(i32);

    let number = Int(1);

    assert_that(number)
        .extracting_ref("number.0", |n| &n.0)
        .is_one();
}

#[test]
fn extracting_ref_i32_is_positive() {
    struct Int(i32);

    let number = Int(1);

    assert_that(number)
        .extracting_ref("number.0", |n| &n.0)
        .is_positive();
}

#[test]
fn extracting_ref_i32_is_negative() {
    struct Int(i32);

    let number = Int(-1);

    assert_that(number)
        .extracting_ref("number.0", |n| &n.0)
        .is_negative();
}

#[test]
fn extracting_ref_i32_is_not_positive_and_is_not_negative() {
    struct Int(i32);

    let number = Int(0);

    assert_that(number)
        .extracting_ref("number.0", |n| &n.0)
        .is_not_positive()
        .is_not_negative();
}

#[test]
fn extracting_ref_i32_is_in_range() {
    struct Int(i32);

    let number = Int(9);

    assert_that(number)
        .extracting_ref("number.0", |n| &n.0)
        .is_in_range(1..=9);
}

#[cfg(feature = "float-cmp")]
#[test]
fn extracting_ref_f32_is_close_to() {
    struct Float(f32);

    let value = Float(1.99);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_close_to(1.99);
}

#[test]
fn extracting_ref_f32_is_zero() {
    struct Float(f32);

    let value = Float(0.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_zero();
}

#[test]
fn extracting_ref_f32_is_one() {
    struct Float(f32);

    let value = Float(1.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_one();
}

#[test]
fn extracting_ref_f32_is_positive() {
    struct Float(f32);

    let value = Float(1.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_positive();
}

#[test]
fn extracting_ref_f32_is_negative() {
    struct Float(f32);

    let value = Float(-1.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_negative();
}

#[test]
fn extracting_ref_f32_is_not_positive_and_is_not_negative() {
    struct Float(f32);

    let value = Float(0.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_not_positive()
        .is_not_negative();
}

#[test]
fn extracting_ref_f32_is_infinite() {
    struct Float(f32);

    let value = Float(f32::INFINITY);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_infinite();
}

#[test]
fn extracting_ref_f32_is_not_a_number() {
    struct Float(f32);

    let value = Float(f32::NAN);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_not_a_number();
}

#[cfg(feature = "float-cmp")]
#[test]
fn extracting_ref_f64_is_close_to_within_margin() {
    struct Float(f64);

    let value = Float(1.99);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_close_to_with_margin(1.99, (0.001, 2));
}

#[test]
fn extracting_ref_f64_is_zero() {
    struct Float(f64);

    let value = Float(0.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_zero();
}

#[test]
fn extracting_ref_f64_is_one() {
    struct Float(f64);

    let value = Float(1.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_one();
}

#[test]
fn extracting_ref_f64_is_positive() {
    struct Float(f64);

    let value = Float(1.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_positive();
}

#[test]
fn extracting_ref_f64_is_negative() {
    struct Float(f64);

    let value = Float(-1.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_negative();
}

#[test]
fn extracting_ref_f64_is_not_positive_and_is_not_negative() {
    struct Float(f64);

    let value = Float(0.);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_not_positive()
        .is_not_negative();
}

#[test]
fn extracting_ref_f64_is_infinite() {
    struct Float(f64);

    let value = Float(f64::INFINITY);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
        .is_infinite();
}

#[test]
fn extracting_ref_f64_is_not_a_number() {
    struct Float(f64);

    let value = Float(f64::NAN);

    assert_that(value)
        .extracting_ref("float.0", |f| &f.0)
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
        .extracting_ref("number.0", |n| &n.0)
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
        .extracting_ref("number.0", |n| &n.0)
        .has_precision_of(12);
}

#[cfg(feature = "bigdecimal")]
#[test]
fn extracting_ref_bigdecimal_is_integer() {
    struct DecimalNumber(BigDecimal);

    let number = DecimalNumber("123.0".parse().unwrap_or_else(|err| panic!("{}", err)));

    assert_that(number)
        .extracting_ref("number.0", |n| &n.0)
        .is_integer();
}

#[test]
fn extracting_ref_bool_is_true() {
    struct Flag(bool);

    assert_that(Flag(true))
        .extracting_ref("flag.0", |f| &f.0)
        .is_true();
}

#[test]
fn extracting_ref_bool_is_false() {
    struct Flag(bool);

    assert_that(Flag(false))
        .extracting_ref("flag.0", |f| &f.0)
        .is_false();
}

#[test]
fn extracting_ref_char_is_lowercase() {
    struct Character(char);

    assert_that(Character('r'))
        .extracting_ref("character.0", |c| &c.0)
        .is_lowercase();
}

#[test]
fn extracting_ref_char_is_uppercase() {
    struct Character(char);

    assert_that(Character('R'))
        .extracting_ref("character.0", |c| &c.0)
        .is_uppercase();
}

#[test]
fn extracting_ref_char_is_ascii() {
    struct Character(char);

    assert_that(Character('@'))
        .extracting_ref("character.0", |c| &c.0)
        .is_ascii();
}

#[test]
fn extracting_ref_char_is_alphabetic() {
    struct Character(char);

    assert_that(Character('Z'))
        .extracting_ref("character.0", |c| &c.0)
        .is_alphabetic();
}

#[test]
fn extracting_ref_char_is_alphanumeric() {
    struct Character(char);

    assert_that(Character('Z'))
        .extracting_ref("character.0", |c| &c.0)
        .is_alphanumeric();

    assert_that(Character('5'))
        .extracting_ref("character.0", |c| &c.0)
        .is_alphanumeric();
}

#[test]
fn extracting_ref_char_is_control_char() {
    struct Character(char);

    assert_that(Character('\t'))
        .extracting_ref("character.0", |c| &c.0)
        .is_control_char();

    assert_that(Character('\u{1b}'))
        .extracting_ref("character.0", |c| &c.0)
        .is_control_char();
}

#[test]
fn extracting_ref_char_is_digit() {
    struct Character(char);

    assert_that(Character('0'))
        .extracting_ref("character.0", |c| &c.0)
        .is_digit(10);
}

#[test]
fn extracting_ref_char_is_whitespace() {
    struct Character(char);

    assert_that(Character(' '))
        .extracting_ref("character.0", |c| &c.0)
        .is_whitespace();
    assert_that(Character('\n'))
        .extracting_ref("character.0", |c| &c.0)
        .is_whitespace();
}

#[test]
fn extracting_ref_string_is_empty() {
    struct Name(String);

    let name = Name(String::new());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .is_empty();
}

#[test]
fn extracting_ref_string_is_not_empty() {
    struct Name(String);

    let name = Name(" ".to_string());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .is_not_empty();
}

#[test]
fn extracting_ref_vec_is_empty() {
    struct Bytes(Vec<u8>);

    let name = Bytes(vec![]);

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .is_empty();
}

#[test]
fn extracting_ref_vec_is_not_empty() {
    struct Bytes(Vec<u8>);

    let name = Bytes(vec![48, 65]);

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .is_not_empty();
}

#[test]
fn extracting_ref_string_has_length() {
    struct Name(String);

    let name = Name("Alex".to_string());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .has_length(4);
}

#[test]
fn extracting_ref_string_has_char_count() {
    struct Text(String);

    let name = Text("imper \u{0180} diet al \u{02AA} \u{01AF} zzril".to_string());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .has_char_count(25);
}

#[test]
fn extracting_ref_option_some() {
    struct Optional(Option<String>);

    let note = Optional(Some("note".to_string()));

    assert_that(note)
        .extracting_ref("note.0", |n| &n.0)
        .is_some();
}

#[test]
fn extracting_ref_option_none() {
    struct Optional(Option<String>);

    let note = Optional(None);

    assert_that(note)
        .extracting_ref("note.0", |n| &n.0)
        .is_none();
}

#[test]
fn extracting_ref_option_some_is_equal_to() {
    struct Optional(Option<String>);

    let note = Optional(Some("a note".to_string()));

    assert_that(note)
        .extracting_ref("note.0", |n| &n.0)
        .some()
        .is_equal_to("a note");
}

#[test]
fn extracting_ref_option_has_value() {
    struct Optional(Option<String>);

    let note = Optional(Some("a note".to_string()));

    assert_that(note)
        .extracting_ref("note.0", |n| &n.0)
        .has_value("a note");
}

#[test]
fn extracting_ref_result_is_ok() {
    struct Response(Result<i32, String>);

    let response = Response(Ok(-123));

    assert_that(response)
        .extracting_ref("response.0", |r| &r.0)
        .is_ok();
}

#[test]
fn extracting_ref_result_is_err() {
    struct Response(Result<i32, String>);

    let response = Response(Err("not found".to_string()));

    assert_that(response)
        .extracting_ref("response.0", |r| &r.0)
        .is_err();
}

#[test]
fn extracting_ref_result_ok_is_negative() {
    struct Response(Result<i32, String>);

    let response = Response(Ok(-123));

    assert_that(response)
        .extracting_ref("response.0", |r| &r.0)
        .ok()
        .is_negative();
}

#[test]
fn extracting_ref_result_err_is_equal_to() {
    struct Response(Result<i32, String>);

    let response = Response(Err("not found".to_string()));

    assert_that(response)
        .extracting_ref("response.0", |r| &r.0)
        .err()
        .is_equal_to("not found");
}

#[test]
fn extracting_ref_result_has_value() {
    struct Response(Result<i32, String>);

    let response = Response(Ok(-123));

    assert_that(response)
        .extracting_ref("response.0", |r| &r.0)
        .has_value(-123);
}

#[test]
fn extracting_ref_result_has_error() {
    struct Response(Result<i32, String>);

    let response = Response(Err("not found".to_string()));

    assert_that(response)
        .extracting_ref("response.0", |r| &r.0)
        .has_error("not found");
}

#[test]
fn extracting_ref_string_contains_char() {
    struct Name(String);

    let name = Name("Alexander is here".to_string());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .contains('x');
}

#[test]
fn extracting_ref_string_contains_any_of_chars() {
    struct Name(String);

    let name = Name("Alexander is here".to_string());

    assert_that(name)
        .extracting_ref("name.0", |n| &n.0)
        .contains_any_of(['a', 'e', 'i', 'o', 'u']);
}

#[test]
fn extracting_ref_vec_has_length() {
    struct Bytes(Vec<u8>);

    let bytes = Bytes(vec![1, 2, 3, 4, 5]);

    assert_that(bytes)
        .extracting_ref("bytes.0", |b| &b.0)
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
        .extracting_ref("names.0", |n| &n.0)
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
        .extracting_ref("names.0", |n| &n.0)
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
        .extracting_ref("names.0", |n| &n.0)
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
        .extracting_ref("names.0", |n| &n.0)
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
        .extracting_ref("names.0", |n| &n.0)
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
        .extracting_ref("names.0", |n| &n.0)
        .contains_all_in_order(["Silvia", "Robert"]);
}
