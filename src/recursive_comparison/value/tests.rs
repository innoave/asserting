use super::*;
use crate::recursive_comparison::serialize::to_recursive_value;
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Foo {
    text: String,
    age: u8,
    qux: Qux,
    bytes: Vec<u8>,
    precision: Option<f32>,
    array: Vec<i32>,
    grouped: IndexMap<&'static str, Vec<i64>>,
    pair: (usize, &'static str),
    bar: Bar,
    baz: Baz,
    samples: Vec<Sample>,
}

#[derive(Serialize, Debug)]
struct Qux {
    name: String,
    corge: Corge,
    baz: Baz,
}

#[derive(Serialize, Debug)]
struct Corge {
    grault: bool,
    tinu: (),
}

#[derive(Serialize, Debug)]
enum Sample {
    One,
    Two(i64),
    Three(u64, String),
    Four { left: String, right: char },
}

#[derive(Serialize, Debug)]
struct Bar(i32);

#[derive(Serialize, Debug)]
struct Baz(usize, Vec<i32>);

impl Default for Bar {
    fn default() -> Self {
        Self(-99)
    }
}

impl Default for Baz {
    fn default() -> Self {
        Self(122, vec![0, 0, -1, 1, -2, 2])
    }
}

impl Default for Qux {
    fn default() -> Self {
        Self {
            name: "Silvia".to_string(),
            corge: Corge {
                grault: true,
                tinu: (),
            },
            baz: Baz(99, vec![100, 666, -100]),
        }
    }
}

impl Default for Foo {
    fn default() -> Self {
        Self {
            text: "magna laborum".to_string(),
            age: 21,
            qux: Qux::default(),
            bytes: vec![24, 17, 64, 19],
            precision: Some(2.5),
            array: vec![12, -8, -34, 55, 76],
            grouped: IndexMap::from_iter([("old", vec![1, -1]), ("new", vec![-1, 0, 1])]),
            pair: (123_456, "sit wisi"),
            bar: Bar::default(),
            baz: Baz::default(),
            samples: vec![
                Sample::One,
                Sample::Two(22),
                Sample::Three(33, "amet".into()),
                Sample::Four {
                    left: "dolores".into(),
                    right: 'v',
                },
            ],
        }
    }
}

#[test]
fn debug_string_of_field() {
    let field = Field {
        name: "foo".into(),
        value: Value::Bool(true),
    };

    assert_eq!(format!("{field:?}"), "foo: true");
}

#[test]
fn debug_string_of_value() {
    let foo = Foo::default();

    let value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(format!("{value:?}"), format!("{foo:?}"));
}

#[test]
fn debug_string_of_empty_struct() {
    #[derive(Serialize, Debug)]
    struct AnEmptyStruct {}

    let data = AnEmptyStruct {};

    let value = to_recursive_value(&data).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(format!("{value:?}"), format!("{data:?}"));
}

#[test]
fn debug_string_of_anonymous_struct() {
    let value = struct_(
        "",
        vec![
            Field {
                name: "foo".into(),
                value: Value::Bool(true),
            },
            Field {
                name: "bar".into(),
                value: Value::Number(Number::I64(42)),
            },
        ],
    );

    assert_eq!(format!("{value:?}"), "{ foo: true, bar: 42 }");
}

#[test]
fn debug_string_of_tuple() {
    let value = tuple(vec![Value::Bool(true), Value::Number(Number::I64(42))]);

    assert_eq!(format!("{value:?}"), "(true, 42)");
}

#[test]
fn debug_string_of_tuple_struct() {
    let value = tuple_struct(
        "TruesCount",
        vec![Value::Bool(true), Value::Number(Number::I64(42))],
    );

    assert_eq!(format!("{value:?}"), "TruesCount(true, 42)");
}

#[test]
fn type_name_of_bool_value() {
    let value = Value::Bool(true);

    assert_eq!(value.type_name(), Cow::Borrowed("bool"));
}

#[test]
fn type_name_of_char_value() {
    let value = Value::Char('@');

    assert_eq!(value.type_name(), Cow::Borrowed("char"));
}

#[test]
fn type_name_of_map_value() {
    let value = Value::Map(Map::from_iter([
        (Value::String("foo".into()), Value::Number(Number::U64(0))),
        (Value::String("bar".into()), Value::Number(Number::U64(42))),
    ]));

    assert_eq!(value.type_name(), Cow::Borrowed("Map<String, u64>"));
}

#[test]
fn type_name_of_int32_value() {
    let value = Value::Number(Number::I32(42));

    assert_eq!(value.type_name(), Cow::Borrowed("i32"));
}

#[test]
fn type_name_of_empty_sequence_value() {
    let value = Value::Seq(vec![]);

    assert_eq!(value.type_name(), Cow::Borrowed("Vec<Value>"));
}

#[test]
fn type_name_of_sequence_of_u16_values() {
    let value = Value::Seq(vec![Value::Number(Number::U16(42))]);

    assert_eq!(value.type_name(), Cow::Borrowed("Vec<u16>"));
}

#[test]
fn type_name_of_string_value() {
    let value = Value::String("foo".into());

    assert_eq!(value.type_name(), Cow::Borrowed("String"));
}

#[test]
fn type_name_of_person_struct() {
    let value = Value::Struct {
        type_name: "Person".into(),
        fields: vec![],
    };

    assert_eq!(value.type_name(), Cow::Borrowed("Person"));
}

#[test]
fn type_name_of_tuple_one_element() {
    let value = tuple([Value::String("foo".into())]);

    assert_eq!(value.type_name(), Cow::Borrowed("(String,)"));
}

#[test]
fn type_name_of_tuple_two_elements() {
    let value = tuple([Value::String("foo".into()), Value::Number(Number::I16(-42))]);

    assert_eq!(value.type_name(), Cow::Borrowed("(String, i16)"));
}

#[test]
fn type_name_of_unit_value() {
    let value = Value::Unit;

    assert_eq!(value.type_name(), Cow::Borrowed("()"));
}

#[test]
fn depth_first_iterator_visits_fields_in_correct_order() {
    let foo = Foo {
        text: "magna laborum".to_string(),
        age: 21,
        qux: Qux {
            name: "Silvia".to_string(),
            corge: Corge {
                grault: false,
                tinu: (),
            },
            baz: Baz(99, vec![100, 666, -100]),
        },
        bytes: vec![24, 17, 64, 19],
        precision: Some(2.5),
        array: vec![12, -8, -34, 55, 76],
        grouped: IndexMap::from_iter([("old", vec![1, -1]), ("new", vec![-1, 0, 1])]),
        pair: (123_456, "sit wisi"),
        bar: Bar(-99),
        baz: Baz(122, vec![0, 0, -1, 1, -2, 2]),
        samples: vec![
            Sample::One,
            Sample::Two(22),
            Sample::Three(33, "amet".into()),
            Sample::Four {
                left: "dolores".into(),
                right: 'v',
            },
        ],
    };

    let value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let visited = value
        .depth_first_iter()
        .map(|(path, field)| (path.to_string(), format!("{field:?}")))
        .collect::<Vec<_>>();

    assert_eq!(
        visited,
        vec![
            ("text".to_string(), "\"magna laborum\"".to_string()),
            ("age".to_string(), "21".to_string()),
            ("qux.name".to_string(), "\"Silvia\"".to_string()),
            ("qux.corge.grault".to_string(), "false".to_string()),
            ("qux.corge.tinu".to_string(), "()".to_string()),
            ("qux.baz.0".to_string(), "99".to_string()),
            ("qux.baz.1".to_string(), "[100, 666, -100]".to_string()),
            ("bytes".to_string(), "[24, 17, 64, 19]".to_string()),
            ("precision.0".to_string(), "2.5".to_string()),
            ("array".to_string(), "[12, -8, -34, 55, 76]".to_string()),
            (
                "grouped".to_string(),
                "{\"old\": [1, -1], \"new\": [-1, 0, 1]}".to_string()
            ),
            ("pair.0".to_string(), "123456".to_string()),
            ("pair.1".to_string(), "\"sit wisi\"".to_string()),
            ("bar.0".to_string(), "-99".to_string()),
            ("baz.0".to_string(), "122".to_string()),
            ("baz.1".to_string(), "[0, 0, -1, 1, -2, 2]".to_string()),
            (
                "samples".to_string(),
                "[One, Two(22), Three(33, \"amet\"), Four { left: \"dolores\", right: 'v' }]"
                    .to_string()
            ),
        ]
    );
}

#[test]
fn get_path_foo_empty_path() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from(""));

    assert_eq!(value, Some(&foo_value));
}

#[test]
fn get_path_foo_one_level_deep_not_existing() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("not_existing"));

    assert_eq!(value, None);
}

#[test]
fn get_path_foo_text() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("text"));

    assert_eq!(value, Some(&Value::String("magna laborum".into())));
}

#[test]
fn get_path_foo_age() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("age"));

    assert_eq!(value, Some(&Value::Number(Number::U8(21))));
}

#[test]
fn get_path_foo_qux() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux"));

    assert_eq!(
        value,
        Some(&Value::Struct {
            type_name: "Qux".into(),
            fields: vec![
                Field {
                    name: "name".into(),
                    value: Value::String("Silvia".into()),
                },
                Field {
                    name: "corge".into(),
                    value: Value::Struct {
                        type_name: "Corge".into(),
                        fields: vec![
                            Field {
                                name: "grault".into(),
                                value: Value::Bool(true),
                            },
                            Field {
                                name: "tinu".into(),
                                value: Value::Unit,
                            },
                        ]
                    },
                },
                Field {
                    name: "baz".into(),
                    value: Value::TupleStruct {
                        type_name: "Baz".into(),
                        values: vec![
                            Field {
                                name: "0".into(),
                                value: Value::Number(Number::U64(99))
                            },
                            Field {
                                name: "1".into(),
                                value: Value::Seq(vec![
                                    Value::Number(Number::I32(100)),
                                    Value::Number(Number::I32(666)),
                                    Value::Number(Number::I32(-100)),
                                ])
                            },
                        ],
                    }
                }
            ],
        })
    );
}

#[test]
fn get_path_foo_bytes() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("bytes"));

    assert_eq!(
        value,
        Some(&Value::Seq(vec![
            Value::Number(Number::U8(24)),
            Value::Number(Number::U8(17)),
            Value::Number(Number::U8(64)),
            Value::Number(Number::U8(19))
        ]))
    );
}

#[test]
fn get_path_foo_precision() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("precision"));

    assert_eq!(
        value,
        Some(&Value::TupleVariant {
            type_name: "Option".into(),
            variant: "Some".into(),
            values: vec![Field {
                name: "0".into(),
                value: Value::Number(Number::F32(F32(2.5))),
            }],
        })
    );
}

#[test]
fn get_path_foo_grouped() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("grouped"));

    assert_eq!(
        value,
        Some(&Value::Map(Map::from_iter([
            (
                Value::String("old".into()),
                Value::Seq(vec![
                    Value::Number(Number::I64(1)),
                    Value::Number(Number::I64(-1))
                ])
            ),
            (
                Value::String("new".into()),
                Value::Seq(vec![
                    Value::Number(Number::I64(-1)),
                    Value::Number(Number::I64(0)),
                    Value::Number(Number::I64(1))
                ])
            )
        ])))
    );
}

#[test]
fn get_path_foo_pair() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("pair"));

    assert_eq!(
        value,
        Some(&Value::Tuple(vec![
            Field {
                name: "0".into(),
                value: Value::Number(Number::U64(123_456)),
            },
            Field {
                name: "1".into(),
                value: Value::String("sit wisi".into()),
            }
        ]))
    );
}

#[test]
fn get_path_foo_baz() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("baz"));

    assert_eq!(
        value,
        Some(&Value::TupleStruct {
            type_name: "Baz".into(),
            values: vec![
                Field {
                    name: "0".into(),
                    value: Value::Number(Number::U64(122)),
                },
                Field {
                    name: "1".into(),
                    value: Value::Seq(vec![
                        Value::Number(Number::I32(0)),
                        Value::Number(Number::I32(0)),
                        Value::Number(Number::I32(-1)),
                        Value::Number(Number::I32(1)),
                        Value::Number(Number::I32(-2)),
                        Value::Number(Number::I32(2)),
                    ]),
                }
            ],
        })
    );
}

#[test]
fn get_path_foo_samples() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("samples"));

    assert_eq!(
        value,
        Some(&Value::Seq(vec![
            Value::UnitVariant {
                type_name: "Sample".into(),
                variant: "One".into()
            },
            Value::TupleVariant {
                type_name: "Sample".into(),
                variant: "Two".into(),
                values: vec![Field {
                    name: "0".into(),
                    value: Value::Number(Number::I64(22))
                }]
            },
            Value::TupleVariant {
                type_name: "Sample".into(),
                variant: "Three".into(),
                values: vec![
                    Field {
                        name: "0".into(),
                        value: Value::Number(Number::U64(33))
                    },
                    Field {
                        name: "1".into(),
                        value: Value::String("amet".into())
                    }
                ]
            },
            Value::StructVariant {
                type_name: "Sample".into(),
                variant: "Four".into(),
                fields: vec![
                    Field {
                        name: "left".into(),
                        value: Value::String("dolores".into())
                    },
                    Field {
                        name: "right".into(),
                        value: Value::Char('v')
                    }
                ]
            },
        ]))
    );
}

#[test]
fn get_path_foo_two_levels_deep_not_existing() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux.not_existing"));

    assert_eq!(value, None);
}

#[test]
fn get_path_foo_qux_name() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux.name"));

    assert_eq!(value, Some(&Value::String("Silvia".into())));
}

#[test]
fn get_path_foo_qux_corge() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux.corge"));

    assert_eq!(
        value,
        Some(&Value::Struct {
            type_name: "Corge".into(),
            fields: vec![
                Field {
                    name: "grault".into(),
                    value: Value::Bool(true),
                },
                Field {
                    name: "tinu".into(),
                    value: Value::Unit
                }
            ],
        })
    );
}

#[test]
fn get_path_foo_qux_baz() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux.baz"));

    assert_eq!(
        value,
        Some(&Value::TupleStruct {
            type_name: "Baz".into(),
            values: vec![
                Field {
                    name: "0".into(),
                    value: Value::Number(Number::U64(99)),
                },
                Field {
                    name: "1".into(),
                    value: Value::Seq(vec![
                        Value::Number(Number::I32(100)),
                        Value::Number(Number::I32(666)),
                        Value::Number(Number::I32(-100)),
                    ]),
                }
            ]
        })
    );
}

#[test]
fn get_path_foo_three_levels_deep_not_existing() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux.corge.not_existing"));

    assert_eq!(value, None);
}

#[test]
fn get_path_foo_qux_corge_grault() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux.corge.grault"));

    assert_eq!(value, Some(&Value::Bool(true)));
}

#[test]
fn get_path_foo_qux_corge_tinu() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("qux.corge.tinu"));

    assert_eq!(value, Some(&Value::Unit));
}

#[test]
fn get_path_foo_indexing_into_tuple_0() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("pair.0"));

    assert_eq!(value, Some(&Value::Number(Number::U64(123_456))));
}

#[test]
fn get_path_foo_indexing_into_tuple_1() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("pair.1"));

    assert_eq!(value, Some(&Value::String("sit wisi".into())));
}

#[test]
fn get_path_foo_indexing_into_tuple_struct_0() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("baz.0"));

    assert_eq!(value, Some(&Value::Number(Number::U64(122))));
}

#[test]
fn get_path_foo_indexing_into_tuple_struct_1() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("baz.1"));

    assert_eq!(
        value,
        Some(&Value::Seq(vec![
            Value::Number(Number::I32(0)),
            Value::Number(Number::I32(0)),
            Value::Number(Number::I32(-1)),
            Value::Number(Number::I32(1)),
            Value::Number(Number::I32(-2)),
            Value::Number(Number::I32(2)),
        ]))
    );
}

#[test]
fn get_path_foo_indexing_into_tuple_struct_out_of_bounds() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("baz.2"));

    assert_eq!(value, None);
}

#[test]
fn get_path_foo_indexing_into_sequence_0() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("array.0"));

    assert_eq!(value, Some(&Value::Number(Number::I32(12))));
}

#[test]
fn get_path_foo_indexing_into_sequence_1() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("array.1"));

    assert_eq!(value, Some(&Value::Number(Number::I32(-8))));
}

#[test]
fn get_path_foo_indexing_into_sequence_4() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("array.4"));

    assert_eq!(value, Some(&Value::Number(Number::I32(76))));
}

#[test]
fn get_path_foo_indexing_into_sequence_out_of_bounds() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("array.5"));

    assert_eq!(value, None);
}

#[test]
fn get_path_foo_get_key_from_map_old() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("grouped.old"));

    assert_eq!(
        value,
        Some(&Value::Seq(vec![
            Value::Number(Number::I64(1)),
            Value::Number(Number::I64(-1))
        ]))
    );
}

#[test]
fn get_path_foo_get_key_from_map_new() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("grouped.new"));

    assert_eq!(
        value,
        Some(&Value::Seq(vec![
            Value::Number(Number::I64(-1)),
            Value::Number(Number::I64(0)),
            Value::Number(Number::I64(1))
        ]))
    );
}

#[test]
fn get_path_foo_get_key_from_map_no_mapping() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("grouped.not_existing"));

    assert_eq!(value, None);
}

#[test]
fn get_path_foo_path_to_tuple_variant_field_0() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("samples.2.0"));

    assert_eq!(value, Some(&Value::Number(Number::U64(33))));
}

#[test]
fn get_path_foo_path_to_tuple_variant_field_1() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("samples.2.1"));

    assert_eq!(value, Some(&Value::String("amet".to_string())));
}

#[test]
fn get_path_foo_path_to_struct_variant_field_left() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("samples.3.left"));

    assert_eq!(value, Some(&Value::String("dolores".into())));
}

#[test]
fn get_path_foo_path_to_struct_variant_field_right() {
    let foo = Foo::default();
    let foo_value = to_recursive_value(&foo).unwrap_or_else(|err| panic!("{err:?}"));

    let value = foo_value.get_path(&Path::from("samples.3.right"));

    assert_eq!(value, Some(&Value::Char('v')));
}

#[test]
fn get_path_string_not_existing() {
    let string_value = Value::String("Lorem ipsum dolor sit amet".into());

    let value = string_value.get_path(&Path::from("Lorem"));

    assert_eq!(value, None);
}
