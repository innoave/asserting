use super::*;
use crate::recursive_comparison::value::proptest_support::*;
use crate::std::cmp::Ordering;
use crate::std::hash::BuildHasher;
use hashbrown::DefaultHashBuilder;
use proptest::prelude::*;

proptest! {
    #[test]
    fn number_from_i8_integer(
        value in any::<i8>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::I8(value));
    }

    #[test]
    fn number_from_i16_integer(
        value in any::<i16>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::I16(value));
    }

    #[test]
    fn number_from_i32_integer(
        value in any::<i32>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::I32(value));
    }

    #[test]
    fn number_from_i64_integer(
        value in any::<i64>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::I64(value));
    }

    #[test]
    fn number_from_i128_integer(
        value in any::<i128>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::I128(value));
    }

    #[test]
    fn number_from_u8_integer(
        value in any::<u8>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::U8(value));
    }

    #[test]
    fn number_from_u16_integer(
        value in any::<u16>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::U16(value));
    }

    #[test]
    fn number_from_u32_integer(
        value in any::<u32>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::U32(value));
    }

    #[test]
    fn number_from_u64_integer(
        value in any::<u64>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::U64(value));
    }

    #[test]
    fn number_from_u128_integer(
        value in any::<u128>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::U128(value));
    }

    #[test]
    fn number_from_f32_integer(
        value in any::<f32>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::F32(F32(value)));
    }

    #[test]
    fn number_from_f64_integer(
        value in any::<f64>()
    ) {
        let number = Number::from(value);

        prop_assert_eq!(number, Number::F64(F64(value)));
    }
}

proptest! {
    #[test]
    fn newtype_f32_from_f32(
        value in any::<f32>()
    ) {
        let float = F32::from(value);

        prop_assert_eq!(float, F32(value));
    }

    #[test]
    fn newtype_f64_from_f64(
        value in any::<f64>()
    ) {
        let float = F64::from(value);

        prop_assert_eq!(float, F64(value));
    }

    #[test]
    fn newtype_f32_debug_string(
        value in any_f32_newtype()
    ) {
        prop_assert_eq!(format!("{:?}", value), format!("{:?}", value.0));
    }

    #[test]
    fn newtype_f64_debug_string(
        value in any_f64_newtype()
    ) {
        prop_assert_eq!(format!("{:?}", value), format!("{:?}", value.0));
    }

    #[test]
    fn newtype_f32_display_string(
        value in any_f32_newtype()
    ) {
        prop_assert_eq!(format!("{}", value), format!("{}", value.0));
    }

    #[test]
    fn newtype_f64_display_string(
        value in any_f64_newtype()
    ) {
        prop_assert_eq!(format!("{}", value), format!("{}", value.0));
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn newtype_f32_val(
        value in any_f32_newtype()
    ) {
        prop_assert_eq!(value.val(), value.0);
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn newtype_f64_val(
        value in any_f64_newtype()
    ) {
        prop_assert_eq!(value.val(), value.0);
    }
}

#[test]
fn newtype_f32_eq_nan() {
    assert_eq!(F32(f32::NAN), F32(f32::NAN));
    assert_ne!(F32(f32::NAN), F32(0.));
    assert_ne!(F32(f32::NAN), F32(f32::MAX));
    assert_ne!(F32(f32::NAN), F32(f32::MIN));
}

#[test]
fn newtype_f32_eq_infinity() {
    assert_eq!(F32(f32::INFINITY), F32(f32::INFINITY));
    assert_ne!(F32(f32::INFINITY), F32(f32::NEG_INFINITY));
}

#[test]
fn newtype_f32_eq_negative_infinity() {
    assert_eq!(F32(f32::NEG_INFINITY), F32(f32::NEG_INFINITY));
    assert_ne!(F32(f32::NEG_INFINITY), F32(f32::INFINITY));
}

#[test]
fn newtype_f32_cmp_nan() {
    assert_eq!(
        F32(f32::NAN).partial_cmp(&F32(f32::NAN)),
        Some(Ordering::Equal)
    );
    assert_eq!(F32(f32::NAN).partial_cmp(&F32(0.)), Some(Ordering::Greater));
    assert_eq!(
        F32(f32::NAN).partial_cmp(&F32(f32::MAX)),
        Some(Ordering::Greater)
    );
    assert_eq!(
        F32(f32::NAN).partial_cmp(&F32(f32::MIN)),
        Some(Ordering::Greater)
    );

    assert_eq!(F32(f32::NAN).cmp(&F32(f32::NAN)), Ordering::Equal);
    assert_eq!(F32(f32::NAN).cmp(&F32(0.)), Ordering::Greater);
    assert_eq!(F32(f32::NAN).cmp(&F32(f32::MAX)), Ordering::Greater);
    assert_eq!(F32(f32::NAN).cmp(&F32(f32::MIN)), Ordering::Greater);
}

#[test]
fn newtype_f32_cmp_infinity() {
    assert_eq!(
        F32(f32::INFINITY).partial_cmp(&F32(f32::INFINITY)),
        Some(Ordering::Equal)
    );
    assert_eq!(
        F32(f32::INFINITY).partial_cmp(&F32(f32::NEG_INFINITY)),
        Some(Ordering::Greater)
    );

    assert_eq!(F32(f32::INFINITY).cmp(&F32(f32::INFINITY)), Ordering::Equal);
    assert_eq!(
        F32(f32::INFINITY).cmp(&F32(f32::NEG_INFINITY)),
        Ordering::Greater
    );
}

#[test]
fn newtype_f32_cmp_negative_infinity() {
    assert_eq!(
        F32(f32::NEG_INFINITY).partial_cmp(&F32(f32::NEG_INFINITY)),
        Some(Ordering::Equal)
    );
    assert_eq!(
        F32(f32::NEG_INFINITY).partial_cmp(&F32(f32::INFINITY)),
        Some(Ordering::Less)
    );

    assert_eq!(
        F32(f32::NEG_INFINITY).cmp(&F32(f32::NEG_INFINITY)),
        Ordering::Equal
    );
    assert_eq!(
        F32(f32::NEG_INFINITY).cmp(&F32(f32::INFINITY)),
        Ordering::Less
    );
}

#[allow(clippy::float_cmp)]
#[test]
fn newtype_f32_cmp_zero_and_negative_zero() {
    assert_eq!(F32(0.).partial_cmp(&F32(-0.)), Some(Ordering::Greater));
    assert_eq!(F32(0.).cmp(&F32(-0.)), Ordering::Greater);
    assert_eq!(F32(0.), F32(-0.));

    assert_eq!(0_f32.total_cmp(&-0_f32), Ordering::Greater);
    assert_eq!(0_f32, -0_f32);
}

#[test]
fn newtype_f32_hash() {
    let hash_builder = DefaultHashBuilder::default();

    let nan_hash = hash_builder.hash_one(F32(f32::NAN));
    let inf_hash = hash_builder.hash_one(F32(f32::INFINITY));
    let neg_inf_hash = hash_builder.hash_one(F32(f32::NEG_INFINITY));

    assert_ne!(nan_hash, inf_hash);
    assert_ne!(nan_hash, neg_inf_hash);
    assert_ne!(inf_hash, neg_inf_hash);

    let zero_hash = hash_builder.hash_one(F64(0.));
    let neg_zero_hash = hash_builder.hash_one(F64(-0.));

    assert_eq!(zero_hash, neg_zero_hash);

    assert_ne!(zero_hash, nan_hash);
    assert_ne!(zero_hash, inf_hash);
    assert_ne!(neg_zero_hash, nan_hash);
    assert_ne!(neg_zero_hash, neg_inf_hash);
}

#[test]
fn newtype_f64_eq_nan() {
    assert_eq!(F64(f64::NAN), F64(f64::NAN));
    assert_ne!(F64(f64::NAN), F64(0.));
    assert_ne!(F64(f64::NAN), F64(f64::MAX));
    assert_ne!(F64(f64::NAN), F64(f64::MIN));
}

#[test]
fn newtype_f64_eq_infinity() {
    assert_eq!(F64(f64::INFINITY), F64(f64::INFINITY));
    assert_ne!(F64(f64::INFINITY), F64(f64::NEG_INFINITY));
}

#[test]
fn newtype_f64_eq_negative_infinity() {
    assert_eq!(F64(f64::NEG_INFINITY), F64(f64::NEG_INFINITY));
    assert_ne!(F64(f64::NEG_INFINITY), F64(f64::INFINITY));
}

#[test]
fn newtype_f64_cmp_nan() {
    assert_eq!(
        F64(f64::NAN).partial_cmp(&F64(f64::NAN)),
        Some(Ordering::Equal)
    );
    assert_eq!(F64(f64::NAN).partial_cmp(&F64(0.)), Some(Ordering::Greater));
    assert_eq!(
        F64(f64::NAN).partial_cmp(&F64(f64::MAX)),
        Some(Ordering::Greater)
    );
    assert_eq!(
        F64(f64::NAN).partial_cmp(&F64(f64::MIN)),
        Some(Ordering::Greater)
    );

    assert_eq!(F64(f64::NAN).cmp(&F64(f64::NAN)), Ordering::Equal);
    assert_eq!(F64(f64::NAN).cmp(&F64(0.)), Ordering::Greater);
    assert_eq!(F64(f64::NAN).cmp(&F64(f64::MAX)), Ordering::Greater);
    assert_eq!(F64(f64::NAN).cmp(&F64(f64::MIN)), Ordering::Greater);
}

#[test]
fn newtype_f64_cmp_infinity() {
    assert_eq!(
        F64(f64::INFINITY).partial_cmp(&F64(f64::INFINITY)),
        Some(Ordering::Equal)
    );
    assert_eq!(
        F64(f64::INFINITY).partial_cmp(&F64(f64::NEG_INFINITY)),
        Some(Ordering::Greater)
    );

    assert_eq!(F64(f64::INFINITY).cmp(&F64(f64::INFINITY)), Ordering::Equal);
    assert_eq!(
        F64(f64::INFINITY).cmp(&F64(f64::NEG_INFINITY)),
        Ordering::Greater
    );
}

#[test]
fn newtype_f64_cmp_negative_infinity() {
    assert_eq!(
        F64(f64::NEG_INFINITY).partial_cmp(&F64(f64::NEG_INFINITY)),
        Some(Ordering::Equal)
    );
    assert_eq!(
        F64(f64::NEG_INFINITY).partial_cmp(&F64(f64::INFINITY)),
        Some(Ordering::Less)
    );

    assert_eq!(
        F64(f64::NEG_INFINITY).cmp(&F64(f64::NEG_INFINITY)),
        Ordering::Equal
    );
    assert_eq!(
        F64(f64::NEG_INFINITY).cmp(&F64(f64::INFINITY)),
        Ordering::Less
    );
}

#[allow(clippy::float_cmp)]
#[test]
fn newtype_f64_cmp_zero_and_negative_zero() {
    assert_eq!(F64(0.).partial_cmp(&F64(-0.)), Some(Ordering::Greater));
    assert_eq!(F64(0.).cmp(&F64(-0.)), Ordering::Greater);
    assert_eq!(F64(0.), F64(-0.));

    assert_eq!(0_f64.total_cmp(&-0_f64), Ordering::Greater);
    assert_eq!(0_f64, -0_f64);
}

#[test]
fn newtype_f64_hash() {
    let hash_builder = DefaultHashBuilder::default();

    let nan_hash = hash_builder.hash_one(F64(f64::NAN));
    let inf_hash = hash_builder.hash_one(F64(f64::INFINITY));
    let neg_inf_hash = hash_builder.hash_one(F64(f64::NEG_INFINITY));

    assert_ne!(nan_hash, inf_hash);
    assert_ne!(nan_hash, neg_inf_hash);
    assert_ne!(inf_hash, neg_inf_hash);

    let zero_hash = hash_builder.hash_one(F64(0.));
    let neg_zero_hash = hash_builder.hash_one(F64(-0.));

    assert_eq!(zero_hash, neg_zero_hash);

    assert_ne!(zero_hash, nan_hash);
    assert_ne!(zero_hash, inf_hash);
    assert_ne!(neg_zero_hash, nan_hash);
    assert_ne!(neg_zero_hash, neg_inf_hash);
}

proptest! {
    #[test]
    fn number_from_primitive(
        number in any_number()
    ) {
        let value = match number {
            Number::I8(val) => Number::from(val),
            Number::I16(val) => Number::from(val),
            Number::I32(val) => Number::from(val),
            Number::I64(val) => Number::from(val),
            Number::I128(val) => Number::from(val),
            Number::U8(val) => Number::from(val),
            Number::U16(val) => Number::from(val),
            Number::U32(val) => Number::from(val),
            Number::U64(val) => Number::from(val),
            Number::U128(val) => Number::from(val),
            Number::F32(F32(val)) => Number::from(val),
            Number::F64(F64(val)) => Number::from(val),
        };

        prop_assert_eq!(value, number);
    }

    #[test]
    fn number_debug_string(
        number in any_number()
    ) {
        let primitive_debug_string = match number {
            Number::I8(val) => format!("{val:?}"),
            Number::I16(val) => format!("{val:?}"),
            Number::I32(val) => format!("{val:?}"),
            Number::I64(val) => format!("{val:?}"),
            Number::I128(val) => format!("{val:?}"),
            Number::U8(val) => format!("{val:?}"),
            Number::U16(val) => format!("{val:?}"),
            Number::U32(val) => format!("{val:?}"),
            Number::U64(val) => format!("{val:?}"),
            Number::U128(val) => format!("{val:?}"),
            Number::F32(F32(val)) => format!("{val:?}"),
            Number::F64(F64(val)) => format!("{val:?}"),
        };

        prop_assert_eq!(format!("{number:?}"), primitive_debug_string);
    }

    #[test]
    fn number_display_string(
        number in any_number()
    ) {
        let primitive_display_string = match number {
            Number::I8(val) => format!("{val}"),
            Number::I16(val) => format!("{val}"),
            Number::I32(val) => format!("{val}"),
            Number::I64(val) => format!("{val}"),
            Number::I128(val) => format!("{val}"),
            Number::U8(val) => format!("{val}"),
            Number::U16(val) => format!("{val}"),
            Number::U32(val) => format!("{val}"),
            Number::U64(val) => format!("{val}"),
            Number::U128(val) => format!("{val}"),
            Number::F32(F32(val)) => format!("{val}"),
            Number::F64(F64(val)) => format!("{val}"),
        };

        prop_assert_eq!(format!("{number}"), primitive_display_string);
    }
}
