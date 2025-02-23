use crate::equality::IsEqualTo;
use crate::specification::assert_that;
use proptest::prelude::*;

//
// String and str
//

proptest! {
    #[test]
    fn string_is_equal_to_string(
        input in any::<String>(),
    ) {
        let subject: String = input.clone();
        let expected: String = input;

        assert_that(subject).is_equal_to(expected);
    }

    #[test]
    fn string_is_equal_to_str(
        input in any::<String>(),
    ) {
        let subject: String = input.clone();
        let expected: &str = &input;

        assert_that(subject).is_equal_to(expected);
    }

    #[test]
    fn string_ref_is_equal_to_str(
        input in any::<String>(),
    ) {
        let subject: &String = &input;
        let expected: &str = &input;

        assert_that::<&String>(subject).is_equal_to(expected);
    }

    #[test]
    fn str_is_equal_to_str(
        input in any::<String>(),
    ) {
        let subject: &str = &input;
        let expected: &str = &input;

        assert_that(subject).is_equal_to(expected);
    }

    #[test]
    fn str_is_equal_to_string(
        input in any::<String>(),
    ) {
        let subject: &str = &input.clone();
        let expected: String = input;

        assert_that(subject).is_equal_to(expected);
    }

    #[test]
    fn string_is_not_equal_to_string(
        (input1, input2) in (any::<String>(), any::<String>()).prop_filter("which are not equal", |(s1, s2)| s1 != s2),
    ) {
        let subject: String = input1;
        let expected: String = input2;

        assert_that(subject).is_not_equal_to(expected);
    }

    #[test]
    fn string_is_not_equal_to_str(
        (input1, input2) in (any::<String>(), any::<String>()).prop_filter("which are not equal", |(s1, s2)| s1 != s2),
    ) {
        let subject: String = input1;
        let expected: &str = &input2;

        assert_that(subject).is_not_equal_to(expected);
    }

    #[test]
    fn string_ref_is_not_equal_to_str(
        (input1, input2) in (any::<String>(), any::<String>()).prop_filter("which are not equal", |(s1, s2)| s1 != s2),
    ) {
        let subject: String = input1;
        let expected: &str = &input2;

        assert_that::<&String>(&subject).is_not_equal_to(expected);
    }

    #[test]
    fn str_is_not_equal_to_str(
        (input1, input2) in (any::<String>(), any::<String>()).prop_filter("which are not equal", |(s1, s2)| s1 != s2),
    ) {
        let subject: &str = &input1;
        let expected: &str = &input2;

        assert_that(subject).is_not_equal_to(expected);
    }

    #[test]
    fn str_is_not_equal_to_string(
        (input1, input2) in (any::<String>(), any::<String>()).prop_filter("which are not equal", |(s1, s2)| s1 != s2),
    ) {
        let subject: &str = &input1;
        let expected: String = input2;

        assert_that(subject).is_not_equal_to(expected);
    }
}

//
// Integer
//

proptest! {
    #[test]
    fn usize_is_equal_to_usize(
        input in any::<usize>(),
    ) {
        let subject: usize = input;
        let expected: usize = input;

        assert_that(subject).is_equal_to(expected);
    }

    #[test]
    fn usize_is_not_equal_to_usize(
        (input1, input2) in (any::<usize>(), any::<usize>()).prop_filter("which are not equal", |(s1, s2)| s1 != s2),
    ) {
        let subject: usize = input1;
        let expected: usize = input2;

        assert_that(subject).is_not_equal_to(expected);
    }
}

//
// bool
//

proptest! {
    #[test]
    fn bool_is_equal_to_bool(
        input in any::<bool>(),
    ) {
        let subject: bool = input;
        let expected: bool = input;

        assert_that(subject).is_equal_to(expected);
    }

    #[test]
    fn bool_is_not_equal_to_bool(
        input in any::<bool>(),
    ) {
        let subject: bool = input;
        let expected: bool = !input;

        assert_that(subject).is_not_equal_to(expected);
    }
}
