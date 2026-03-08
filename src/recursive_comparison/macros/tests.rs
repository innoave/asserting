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
fn anonymous_struct_value_with_one_field_no_trailing_comma() {
    let value = value!({ name: "Alice" });

    assert_eq!(value, struct_with_fields([field("name", string("Alice"))]));
}

#[test]
fn anonymous_struct_value_with_one_field_of_negative_number() {
    let value = value!({ sum: -22_i16 });

    assert_eq!(value, struct_with_fields([field("sum", int16(-22))]));
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
fn named_struct_value_with_one_field_of_negative_number() {
    let value = value!(Foo { count: -234_i64 });

    assert_eq!(value, struct_("Foo", [field("count", int64(-234))]));
}

#[test]
fn named_struct_value_with_two_fields_no_trailing_comma() {
    let value = value!(Foo {
        name: "Alice",
        age: 25_u8
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
fn empty_named_struct() {
    let value = value!(Foo {});

    assert_eq!(value, struct_("Foo", Vec::<Field>::new()));
}

#[test]
fn struct_variant_value_with_one_field_no_trailing_comma() {
    let value = value!(Foo::Bar { baz: -5.5_f32 });

    assert_eq!(
        value,
        struct_variant("Foo", "Bar", [("baz", float32(-5.5))])
    );
}

#[test]
fn struct_variant_value_with_two_field_no_trailing_comma() {
    let value = value!(Foo::Bar {
        baz: -5.5_f32,
        qux: "Silvia".to_string()
    });

    assert_eq!(
        value,
        struct_variant(
            "Foo",
            "Bar",
            [("baz", float32(-5.5)), ("qux", string("Silvia"))]
        )
    );
}

#[test]
fn struct_variant_value_with_two_fields_and_trailing_comma() {
    let value = value!(Foo::Bar {
        baz: '@',
        qux: "hello, world!".to_string(),
    });

    assert_eq!(
        value,
        struct_variant(
            "Foo",
            "Bar",
            [("baz", char('@')), ("qux", string("hello, world!"))]
        )
    );
}

#[test]
fn empty_struct_variant() {
    let value = value!(Foo::Bar {});

    assert_eq!(value, struct_variant("Foo", "Bar", Vec::<Field>::new()));
}

#[test]
fn tuple_struct_value_with_one_field() {
    let value = value!(Foo("Silvia"));

    assert_eq!(value, tuple_struct("Foo", [string("Silvia")]));
}

#[test]
fn tuple_struct_value_with_three_fields_no_trailing_comma() {
    let value = value!(Foo("Silvia", true, -2.4_f32));

    assert_eq!(
        value,
        tuple_struct("Foo", [string("Silvia"), bool(true), float32(-2.4)])
    );
}

#[test]
fn tuple_struct_value_with_three_fields_and_trailing_comma() {
    let value = value!(Foo("Silvia", true, -2.4_f32,));

    assert_eq!(
        value,
        tuple_struct("Foo", [string("Silvia"), bool(true), float32(-2.4)])
    );
}

#[test]
fn empty_tuple_struct() {
    let value = value!(Foo());

    assert_eq!(value, unit_struct("Foo"));
}

#[test]
fn tuple_value_with_one_field_no_trailing_comma() {
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
fn tuple_value_with_three_fields_and_negative_number() {
    let value = value!(("Alice", 1.2_f64, -87_i16));

    assert_eq!(value, tuple([string("Alice"), float64(1.2), int16(-87)]));
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

#[test]
fn tuple_variant_value_with_one_field() {
    let value = value!(Foo::Bar(2.5_f32));

    assert_eq!(value, tuple_variant("Foo", "Bar", [float32(2.5)]));
}

#[test]
fn tuple_variant_value_with_one_field_negative_number() {
    let value = value!(Foo::Bar(-2.5_f32));

    assert_eq!(value, tuple_variant("Foo", "Bar", [float32(-2.5)]));
}

#[test]
fn tuple_variant_value_with_two_field() {
    let value = value!(Foo::Bar("Silvia", 1228_i64));

    assert_eq!(
        value,
        tuple_variant("Foo", "Bar", [string("Silvia"), int64(1228)])
    );
}

#[test]
fn tuple_variant_value_with_two_field_one_with_negative_number() {
    let value = value!(Foo::Bar("Silvia", -1228_i64));

    assert_eq!(
        value,
        tuple_variant("Foo", "Bar", [string("Silvia"), int64(-1228)])
    );
}

#[test]
fn unit_variant_value() {
    let value = value!(Foo::Bar);

    assert_eq!(value, unit_variant("Foo", "Bar"));
}

#[test]
fn unit_struct_value() {
    let value = value!(Foo);

    assert_eq!(value, unit_struct("Foo"));
}

#[test]
fn empty_seq_value() {
    let value = value!([]);

    assert_eq!(value, seq([]));
}

#[test]
fn seq_value_with_one_element() {
    let value = value!([25_u32]);

    assert_eq!(value, seq([uint32(25)]));
}

#[test]
fn seq_value_with_one_element_and_trailing_comma() {
    let value = value!(['@',]);

    assert_eq!(value, seq([char('@')]));
}

#[test]
fn seq_value_with_two_elements_no_trailing_comma() {
    let value = value!([25, -32]);

    assert_eq!(value, seq([int32(25), int32(-32)]));
}

#[test]
fn seq_value_with_two_elements_and_trailing_comma() {
    let value = value!([25, -32,]);

    assert_eq!(value, seq([int32(25), int32(-32)]));
}

#[test]
fn seq_value_with_three_elements() {
    let value = value!(["alpha", "beta", "gamma"]);

    assert_eq!(
        value,
        seq([string("alpha"), string("beta"), string("gamma")])
    );
}

#[test]
fn seq_value_with_expression() {
    let value = value!([25, 3 + 7, 55]);

    assert_eq!(value, seq([int32(25), int32(10), int32(55)]));
}

#[test]
fn struct_value_with_captured_variable() {
    let name = "Alice".to_string();

    let value = value!({
        name: name,
        age: 25_u8,
    });

    assert_eq!(
        value,
        struct_with_fields([("name", string("Alice")), ("age", uint8(25))])
    );
}

#[test]
fn anonymous_struct_with_nested_anonymous_struct_value_and_unit_variant() {
    let value = value!({
        name: "Silvia",
        gender: Gender::Female,
        age: 25_u8,
        address: {
            street: "123 Main St",
            city: "New York",
            state: "NY",
            zip: 10001_u32,
            home: true,
        }
    });

    assert_eq!(
        value,
        struct_with_fields([
            ("name", string("Silvia")),
            ("gender", unit_variant("Gender", "Female")),
            ("age", uint8(25)),
            (
                "address",
                struct_with_fields([
                    ("street", string("123 Main St")),
                    ("city", string("New York")),
                    ("state", string("NY")),
                    ("zip", uint32(10001)),
                    ("home", bool(true)),
                ])
            )
        ])
    );
}

#[test]
fn anonymous_struct_with_nested_unit_variant() {
    let value = value!({
        foo: Foo::Bar
    });

    assert_eq!(
        value,
        struct_with_fields([("foo", unit_variant("Foo", "Bar"))])
    );
}

#[test]
fn anonymous_struct_with_nested_struct_variant() {
    let value = value!({
        foo: Foo::Bar {
            gender: Gender::Male,
            count: 3472_u64,
        },
    });

    assert_eq!(
        value,
        struct_with_fields([(
            "foo",
            struct_variant(
                "Foo",
                "Bar",
                [
                    ("gender", unit_variant("Gender", "Male")),
                    ("count", uint64(3472))
                ]
            )
        )])
    );
}

#[test]
fn anonymous_struct_with_nested_seq() {
    let value = value!({
        names: [
            "Alice",
            "Bob",
            "Charlie",
        ],
    });

    assert_eq!(
        value,
        struct_with_fields([(
            "names",
            seq([string("Alice"), string("Bob"), string("Charlie")])
        )])
    );
}

#[test]
fn anonymous_struct_with_nested_tuple() {
    let value = value!({
        foo: ("Silvia", false, -2.3_f32),
    });

    assert_eq!(
        value,
        struct_with_fields([(
            "foo",
            tuple([string("Silvia"), bool(false), float32(-2.3_f32)])
        )])
    );
}

#[test]
fn anonymous_struct_with_nested_named_struct() {
    let value = value!({
        foo: Foo {
            bar: "xyz",
            baz: 42_i16,
        },
        qux: "abc"
    });

    assert_eq!(
        value,
        struct_with_fields([
            (
                "foo",
                struct_("Foo", [("bar", string("xyz")), ("baz", int16(42))])
            ),
            ("qux", string("abc"))
        ])
    );
}

#[test]
fn anonymous_struct_with_nested_tuple_struct() {
    let value = value!({
        foo: Bar(4.6_f32, 12)
    }
    );

    assert_eq!(
        value,
        struct_with_fields([("foo", tuple_struct("Bar", [float32(4.6), int32(12)]))])
    );
}
