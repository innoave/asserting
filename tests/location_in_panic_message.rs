//! Tests that assure that the location in the panic message of the various
//! assertions is correct.
#![allow(unused_crate_dependencies)]

#[cfg(feature = "std")]
mod with_std {
    use asserting::assert_panic_location;
    use asserting::prelude::*;

    #[test]
    fn failing_location_of_is_equal_to() {
        assert_panic_location!(assert_that!(2).is_equal_to(1));
    }

    #[test]
    fn failing_location_of_is_zero() {
        assert_panic_location!(assert_that!(2).is_zero());
    }

    #[test]
    fn failing_location_of_is_none() {
        assert_panic_location!(assert_that!(Some(42)).is_none());
    }

    #[test]
    fn failing_location_of_is_err() {
        assert_panic_location!(assert_that!(Ok::<_, String>(42)).is_err());
    }
}

#[cfg(not(feature = "std"))]
mod with_no_std {
    use asserting::prelude::*;

    #[test]
    #[should_panic = "expected 2 to be equal to 1\n   but was: 2\n  expected: 1\n"]
    fn a_failing_test_panics_in_no_std_environment() {
        assert_that!(2)
            .with_diff_format(DIFF_FORMAT_NO_HIGHLIGHT)
            .is_equal_to(1);
    }
}
