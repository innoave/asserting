use super::*;
use crate::recursive_comparison::value::{
    self, bool, char, field, float32, float64, int8, int16, int32, int64, int128, isize, map, none,
    seq, some, string, struct_, struct_variant, tuple, tuple_struct, tuple_variant, uint8, uint16,
    uint32, uint64, uint128, unit, unit_struct, unit_variant, usize,
};
use crate::std::string::ToString;
use indexmap::IndexMap;
use serde::Serialize;
use serde_bytes::Bytes;

mod error {
    use super::*;

    #[test]
    fn can_create_custom_error() {
        let error = Error::custom("gubergren eu nonummy");

        assert_eq!(error, Error::Message("gubergren eu nonummy".into()));
    }

    #[test]
    fn display_string_of_custom_error() {
        let error = Error::custom("gubergren eu nonummy");

        assert_eq!(error.to_string(), "gubergren eu nonummy");
    }
}

#[test]
fn serialize_tuple_of_bool() {
    let value = (true, false);

    let serialized = to_recursive_value(&value).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(serialized, tuple([bool(true), bool(false)]));
}

#[test]
fn serialize_int8() {
    let serialized = to_recursive_value(&-42_i8).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, int8(-42));
}

#[test]
fn serialize_int16() {
    let serialized = to_recursive_value(&-16_i16).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, int16(-16));
}

#[test]
fn serialize_int32() {
    let serialized = to_recursive_value(&-32_i32).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, int32(-32));
}

#[test]
fn serialize_int64() {
    let serialized = to_recursive_value(&-64_i64).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, int64(-64));
}

#[test]
fn serialize_int128() {
    let serialized = to_recursive_value(&-128_i128).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, int128(-128));
}

#[test]
fn serialize_isize() {
    let serialized = to_recursive_value(&-333_isize).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, isize(-333));
}

#[test]
fn serialize_uint8() {
    let serialized = to_recursive_value(&42_u8).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, uint8(42));
}

#[test]
fn serialize_uint16() {
    let serialized = to_recursive_value(&16_u16).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, uint16(16));
}

#[test]
fn serialize_uint32() {
    let serialized = to_recursive_value(&32_u32).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, uint32(32));
}

#[test]
fn serialize_uint64() {
    let serialized = to_recursive_value(&64_u64).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, uint64(64));
}

#[test]
fn serialize_uint128() {
    let serialized = to_recursive_value(&128_u128).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, uint128(128));
}

#[test]
fn serialize_usize() {
    let serialized = to_recursive_value(&555_usize).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, usize(555));
}

#[test]
fn serialize_float32() {
    let serialized = to_recursive_value(&-0.5_f32).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, float32(-0.5));
}

#[test]
fn serialize_float64() {
    let serialized = to_recursive_value(&1.2_f64).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, float64(1.2));
}

#[test]
fn serialize_char() {
    let serialized = to_recursive_value(&'@').unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, char('@'));
}

#[test]
fn serialize_string() {
    let serialized =
        to_recursive_value(&"hello".to_string()).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, string("hello"));
}

#[test]
fn serialize_bytes() {
    let buffer = Bytes::new(&[65, 66, 67]);

    let serialized = to_recursive_value(&buffer).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(serialized, seq([uint8(65), uint8(66), uint8(67)]));
}

#[test]
fn serialize_none() {
    let maybe: Option<i16> = None;

    let serialized = to_recursive_value(&maybe).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(serialized, none());
}

#[test]
fn serialize_some_i16() {
    let maybe: Option<i16> = Some(-60);

    let serialized = to_recursive_value(&maybe).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(serialized, some(int16(-60)));
}

#[test]
fn serialize_unit() {
    let serialized = to_recursive_value(&()).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, unit());
}

#[test]
fn serialize_unit_struct() {
    #[derive(Serialize)]
    struct Noop;

    let serialized = to_recursive_value(&Noop).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, unit_struct("Noop"));
}

#[test]
fn serialize_unit_variant() {
    #[allow(dead_code)]
    #[derive(Serialize)]
    enum Opacity {
        Transparent,
        Opaque,
    }

    let serialized =
        to_recursive_value(&Opacity::Transparent).unwrap_or_else(|err| panic!("{err:?}"));
    assert_eq!(serialized, unit_variant("Opacity", "Transparent"),);
}

#[test]
fn serialize_vec_of_i32() {
    let sequence = vec![-1, -2, -3];

    let serialized = to_recursive_value(&sequence).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(serialized, seq([int32(-1), int32(-2), int32(-3)]));
}

#[test]
fn serialize_slice_of_i32() {
    let slice = &[-1, -2, -3][..];

    let serialized = to_recursive_value(slice).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(serialized, seq([int32(-1), int32(-2), int32(-3)]));
}

#[test]
fn serialize_array_of_i32() {
    let array = [-1, -2, -3];

    let serialized = to_recursive_value(&array).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(serialized, value::array([int32(-1), int32(-2), int32(-3)]));
}

#[test]
fn serialize_tuple_of_string_and_bool_and_u64() {
    let tuple = ("foo".to_string(), true, 42_u64);

    let serialized = to_recursive_value(&tuple).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(
        serialized,
        value::tuple([string("foo"), bool(true), uint64(42)])
    );
}

#[test]
fn serialize_tuple_struct() {
    #[derive(Serialize)]
    struct Point3D(i16, i16, i16);

    let point = Point3D(16, -7, 0);

    let serialized = to_recursive_value(&point).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(
        serialized,
        tuple_struct("Point3D", [int16(16), int16(-7), int16(0)])
    );
}

#[test]
fn serialize_tuple_variant() {
    #[allow(dead_code)]
    #[derive(Serialize)]
    enum Color {
        Rgb(u8, u8, u8),
        Hsl(u8, u8, u8),
    }

    let color = Color::Rgb(128, 64, 32);

    let serialized = to_recursive_value(&color).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(
        serialized,
        tuple_variant("Color", "Rgb", [uint8(128), uint8(64), uint8(32)])
    );
}

#[test]
fn serialize_map_of_string_u64() {
    let mapping: IndexMap<String, u64> = IndexMap::from_iter([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
    ]);

    let serialized = to_recursive_value(&mapping).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(
        serialized,
        map([
            (string("one"), uint64(1)),
            (string("two"), uint64(2)),
            (string("three"), uint64(3))
        ])
    );
}

#[test]
fn serialize_struct() {
    #[derive(Serialize)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: -8 };

    let serialized = to_recursive_value(&point).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(
        serialized,
        struct_("Point", [field("x", int32(10)), field("y", int32(-8))])
    );
}

#[test]
fn serialize_stuct_variant() {
    #[allow(dead_code)]
    #[derive(Serialize)]
    enum Color {
        Rgb {
            red: u8,
            green: u8,
            blue: u8,
        },
        Hsl {
            hue: u8,
            saturation: u8,
            lightness: u8,
        },
    }

    let color = Color::Rgb {
        red: 128,
        green: 64,
        blue: 32,
    };

    let serialized = to_recursive_value(&color).unwrap_or_else(|err| panic!("{err:?}"));

    assert_eq!(
        serialized,
        struct_variant(
            "Color",
            "Rgb",
            [
                ("red", uint8(128)),
                ("green", uint8(64)),
                ("blue", uint8(32))
            ]
        )
    );
}
