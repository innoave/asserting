use super::*;
use crate::prelude::*;

#[cfg(not(feature = "colored"))]
mod without_colored_feature {
    use super::*;

    #[test]
    fn default_diff_format_is_no_highlight() {
        assert_that(DEFAULT_DIFF_FORMAT).is_equal_to(DIFF_FORMAT_NO_HIGHLIGHT);
    }

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

#[cfg(feature = "colored")]
mod with_colored_feature {
    use super::*;

    #[test]
    fn default_diff_format_is_red_green() {
        assert_that(DEFAULT_DIFF_FORMAT).is_equal_to(DIFF_FORMAT_RED_GREEN);
    }

    #[test]
    fn highlight_diffs_is_equal_to_for_custom_struct() {
        #[derive(Debug, PartialEq)]
        struct Foo {
            lorem: String,
            ipsum: i32,
            dolor: Option<String>,
        }

        let subject = Some(Foo {
            lorem: "¡Hola, Welt!".into(),
            ipsum: 42,
            dolor: Some("hey".into()),
        });

        let failures = verify_that(subject)
            .with_diff_format(DIFF_FORMAT_RED_GREEN)
            .is_equal_to(Some(Foo {
                lorem: "Hello World!".into(),
                ipsum: 42,
                dolor: Some("hey ho!".into()),
            }))
            .display_failures();

        assert_eq!(failures, &[
            "assertion failed: expected subject is equal to Some(Foo { lorem: \"Hello World!\", ipsum: 42, dolor: Some(\"hey ho!\") })\n   \
                 but was: Some(Foo { lorem: \"\u{1b}[31m¡\u{1b}[0mH\u{1b}[31mo\u{1b}[0ml\u{1b}[31ma,\u{1b}[0m W\u{1b}[31me\u{1b}[0ml\u{1b}[31mt\u{1b}[0m!\", ipsum: 42, dolor: Some(\"hey\") })\n  \
               expected: Some(Foo { lorem: \"H\u{1b}[32me\u{1b}[0ml\u{1b}[32mlo\u{1b}[0m W\u{1b}[32mor\u{1b}[0ml\u{1b}[32md\u{1b}[0m!\", ipsum: 42, dolor: Some(\"hey\u{1b}[32m ho!\u{1b}[0m\") })\n\
            ",
        ]);
    }
}

#[cfg(all(feature = "colored", not(feature = "std")))]
mod with_colored_but_not_std_feature {
    use super::*;

    #[test]
    fn assert_that_sets_the_diff_format_to_red_green() {
        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_RED_GREEN);
    }

    #[test]
    fn verify_that_sets_the_diff_format_to_no_highlighting() {
        let assertion = verify_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }
}

#[cfg(all(feature = "colored", feature = "std"))]
mod with_colored_and_std_features {
    use super::*;
    use crate::colored::with_colored_feature::ENV_VAR_HIGHLIGHT_DIFFS;
    use crate::std::env;
    use proptest::prelude::*;
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
    fn assert_that_sets_default_diff_format_env_var_not_set_no_color_env_var_set() {
        env::remove_var(ENV_VAR_HIGHLIGHT_DIFFS);
        env::set_var("NO_COLOR", "1");

        let assertion = assert_that(42);

        env::remove_var("NO_COLOR");

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_unknown_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "not-valid");

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
    fn assert_that_sets_default_diff_format_env_var_set_to_bold_mode_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "bold");
        env::set_var("NO_COLOR", "1");

        let assertion = assert_that(42);

        env::remove_var("NO_COLOR");

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_BOLD);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_red_green_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_RED_GREEN);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_red_green_mode_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");
        env::set_var("NO_COLOR", "1");

        let assertion = assert_that(42);

        env::remove_var("NO_COLOR");

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_red_blue_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-blue");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_RED_BLUE);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_red_blue_mode_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-blue");
        env::set_var("NO_COLOR", "1");

        let assertion = assert_that(42);

        env::remove_var("NO_COLOR");

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_red_yellow_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-yellow");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_RED_YELLOW);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_red_yellow_mode_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-yellow");
        env::set_var("NO_COLOR", "1");

        let assertion = assert_that(42);

        env::remove_var("NO_COLOR");

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    #[serial]
    fn assert_that_sets_default_diff_format_env_var_set_to_off() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "off");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    proptest! {
        #[test]
        #[serial]
        fn setting_env_var_to_bold_is_case_insensitive(
            mode in "[bB][oO][lL][dD]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, mode);

            let assertion = assert_that(42);

            prop_assert_eq!(assertion.diff_format(), &DIFF_FORMAT_BOLD);
        }

        #[test]
        #[serial]
        fn setting_env_var_to_red_blue_is_case_insensitive(
            mode in "[rR][eE][dD]-[bB][lL][uU][eE]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, mode);

            let assertion = assert_that(42);

            prop_assert_eq!(assertion.diff_format(), &DIFF_FORMAT_RED_BLUE);
        }

        #[test]
        #[serial]
        fn setting_env_var_to_red_yellow_is_case_insensitive(
            mode in "[rR][eE][dD]-[yY][eE][lL][lL][oO][wW]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, mode);

            let assertion = assert_that(42);

            prop_assert_eq!(assertion.diff_format(), &DIFF_FORMAT_RED_YELLOW);
        }

        #[test]
        #[serial]
        fn setting_env_var_to_off_is_case_insensitive(
            mode in "[oO][fF][fF]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, mode);

            let assertion = assert_that(42);

            prop_assert_eq!(assertion.diff_format(), &DIFF_FORMAT_NO_HIGHLIGHT);
        }
    }

    #[test]
    fn verify_that_sets_the_diff_format_to_no_highlighting() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");

        let assertion = verify_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }
}
