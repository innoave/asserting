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
fn i32_value() {
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
fn adhoc_struct_value_with_one_field() {
    let value = value!({
        name: "Alice"
    });

    assert_eq!(value, struct_with_fields([field("name", string("Alice"))]));
}

#[test]
fn named_struct_value_with_one_field() {
    let value = value!(Foo { name: "Alice" });

    assert_eq!(value, struct_("Foo", [field("name", string("Alice"))]));
}

#[test]
fn tuple_struct_value_with_one_field() {
    let value = value!(Foo("Silvia"));

    assert_eq!(value, tuple_struct("Foo", [string("Silvia")]));
}
