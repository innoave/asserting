//! This module tests the assertion methods on various custom types.
#![allow(unused_crate_dependencies)]

use asserting::prelude::*;

#[derive(Debug, PartialEq)]
struct IntVal(i32);

#[test]
fn int_val_is_in_array_of_single_int_val() {
    let subject = IntVal(42);

    assert_that!(subject).is_in([IntVal(42)]);
}

#[test]
fn int_val_is_in_array_of_several_int_val() {
    let subject = IntVal(42);

    assert_that!(subject).is_in([IntVal(33), IntVal(42), IntVal(99)]);
}

#[test]
fn int_val_ref_is_in_borrowed_array_of_int_val() {
    let subject = IntVal(42);

    assert_that!(&subject).is_in(&[IntVal(42), IntVal(11), IntVal(28)]);
}

#[test]
fn int_val_ref_is_in_slice_of_several_int_val() {
    let subject = IntVal(42);

    assert_that!(&subject).is_in(&[IntVal(90), IntVal(42)][..]);
}

#[test]
fn int_val_is_in_empty_array_fails() {
    let subject = IntVal(42);

    let failures = verify_that!(subject).is_in([]).display_failures();

    assert_that!(failures).single_element().is_equal_to(
        r"expected subject to be in []
          but was: IntVal(42)
  which is not in: []
",
    );
}
