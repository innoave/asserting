use crate::recursive_comparison::value::*;

#[test]
fn false_value() {
    let value = value!(false);

    assert_eq!(value, bool(false));
}

#[test]
fn true_value() {
    let value = value!(true);

    assert_eq!(value, bool(true));
}

#[test]
fn char_value() {
    let value = value!('@');

    assert_eq!(value, char('@'));
}

#[test]
fn u8_value() {
    let value = value!(42_u8);

    assert_eq!(value, uint8(42));
}

#[test]
fn implicit_i32_value() {
    let value = value!(-234);

    assert_eq!(value, int32(-234));
}

#[test]
fn str_value() {
    let value = value!("Hello, world!");

    assert_eq!(value, string("Hello, world!"));
}

#[test]
fn string_value() {
    let value = value!("Hello, world!".to_string());

    assert_eq!(value, string("Hello, world!"));
}

#[test]
fn anonymous_struct_value_with_one_field() {
    let value = value!({ name: "Alice" });

    assert_eq!(value, struct_with_fields([field("name", string("Alice"))]));
}

#[test]
fn anonymous_struct_value_with_one_field_and_optional_comma() {
    let value = value!({
        name: "Alice",
    });

    assert_eq!(value, struct_with_fields([field("name", string("Alice"))]));
}

#[test]
fn anonymous_struct_value_with_two_fields_without_trailing_comma() {
    let value = value!({ name: "Alice", age: 25_u8 });

    assert_eq!(
        value,
        struct_with_fields([field("name", string("Alice")), field("age", uint8(25))])
    );
}

#[test]
fn anonymous_struct_value_with_two_fields() {
    let value = value!({
        name: "Alice",
        age: 25_u8,
    });

    assert_eq!(
        value,
        struct_with_fields([field("name", string("Alice")), field("age", uint8(25))])
    );
}

#[test]
fn named_struct_value_with_one_field() {
    let value = value!(Foo { name: "Alice" });

    assert_eq!(value, struct_("Foo", [field("name", string("Alice"))]));
}

#[test]
fn named_struct_value_with_two_fields_and_trailing_comma() {
    let value = value!(Foo {
        name: "Alice",
        age: 25_u8,
    });

    assert_eq!(
        value,
        struct_(
            "Foo",
            [field("name", string("Alice")), field("age", uint8(25))]
        )
    );
}

#[test]
fn tuple_struct_value_with_one_field() {
    let value = value!(Foo("Silvia"));

    assert_eq!(value, tuple_struct("Foo", [string("Silvia")]));
}

#[test]
fn tuple_value_with_one_field_and_without_trailing_comma() {
    let value = value!(("Alice"));

    assert_eq!(value, tuple([string("Alice")]));
}

#[test]
fn tuple_value_with_one_field_and_trailing_comma() {
    let value = value!(("Alice",));

    assert_eq!(value, tuple([string("Alice")]));
}

#[test]
fn tuple_value_with_two_fields() {
    let value = value!(("Alice", 25_u8));

    assert_eq!(value, tuple([string("Alice"), uint8(25)]));
}

#[test]
fn tuple_value_with_two_fields_and_trailing_comma() {
    let value = value!(("Alice", 25_u8,));

    assert_eq!(value, tuple([string("Alice"), uint8(25)]));
}

#[test]
fn empty_struct() {
    let value = value!({});

    assert_eq!(value, struct_with_fields::<Field>([]));
}

#[test]
fn empty_tuple() {
    let value = value!(());

    assert_eq!(value, tuple([]));
}

#[test]
fn unit_value() {
    let value = value!(());

    assert_eq!(value, unit());
}
