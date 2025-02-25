use crate::prelude::*;

#[test]
fn expression_is_true() {
    assert_that(42 == 42).is_true();
}

#[test]
fn bool_is_false() {
    assert_that(false).is_false();
}
