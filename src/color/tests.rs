use crate::prelude::*;

#[cfg(not(feature = "color"))]
mod without_color_feature {
    use super::*;
    use crate::color::DIFF_FORMAT_NO_HIGHLIGHT;

    #[test]
    fn assert_that_sets_the_diff_format_to_no_highlighting() {
        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    fn verify_that_sets_the_diff_format_to_no_highlighting() {
        let assertion = verify_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }
}

#[cfg(feature = "color")]
mod with_color_feature {
    use super::*;
    use crate::color::with_color::ENV_VAR_HIGHLIGHT_DIFFS;
    use crate::color::{
        DEFAULT_DIFF_FORMAT, DIFF_FORMAT_BOLD, DIFF_FORMAT_COLORED, DIFF_FORMAT_CVD_COLORED,
        DIFF_FORMAT_NO_HIGHLIGHT,
    };
    use crate::std::env;
    use serial_test::serial;

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_not_set() {
        env::remove_var(ENV_VAR_HIGHLIGHT_DIFFS);

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DEFAULT_DIFF_FORMAT);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_bold_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "bold");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_BOLD);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_colored_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "colored");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_COLORED);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_cvd_colored_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "cvd-colored");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_CVD_COLORED);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_off() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "off");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    #[serial]
    fn verify_that_sets_the_diff_format_to_no_highlighting() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "CVD-colored");

        let assertion = verify_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    fn highlight_diffs_is_equal_to_for_integers() {
        let failures = verify_that(37)
            .with_diff_format(DIFF_FORMAT_CVD_COLORED)
            .is_equal_to(42)
            .display_failures();

        assert_eq!(
            failures,
            &["assertion failed: expected subject is equal to 42\n   \
               but was: \u{1b}[31m37\u{1b}[0m\n  \
              expected: \u{1b}[34m42\u{1b}[0m\n\
            "]
        );
    }

    #[test]
    fn highlight_diffs_is_equal_to_for_strings() {
        let failures = verify_that("invidunt wisi facilisis exercitation")
            .with_diff_format(DIFF_FORMAT_CVD_COLORED)
            .is_equal_to("invi wisi exercitation anim placerat")
            .display_failures();

        assert_eq!(
            failures,
            &[
                "assertion failed: expected subject is equal to \"invi wisi exercitation anim placerat\"\n   \
                   but was: \"invi\u{1b}[31mdunt\u{1b}[0m wisi \u{1b}[31mfacilisis \u{1b}[0mexercitation\"\n  \
                  expected: \"invi wisi exercitation\u{1b}[34m anim placerat\u{1b}[0m\"\n\
            "]
        );
    }
}
