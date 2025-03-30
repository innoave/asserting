use crate::prelude::*;

#[test]
fn highlight_diffs_is_equal_to() {
    let failures = verify_that(37).is_equal_to(42).display_failures();

    assert_eq!(
        failures,
        &["assertion failed: expected subject is equal to 42\n   but was: 37\n  expected: 42\n"]
    );
}
