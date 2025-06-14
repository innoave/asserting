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
    use hashbrown::HashMap;

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
            "assertion failed: expected subject to be equal to Some(Foo { lorem: \"Hello World!\", ipsum: 42, dolor: Some(\"hey ho!\") })\n   \
                 but was: Some(Foo { lorem: \"\u{1b}[31m¡\u{1b}[0mH\u{1b}[31mo\u{1b}[0ml\u{1b}[31ma,\u{1b}[0m W\u{1b}[31me\u{1b}[0ml\u{1b}[31mt\u{1b}[0m!\", ipsum: 42, dolor: Some(\"hey\") })\n  \
               expected: Some(Foo { lorem: \"H\u{1b}[32me\u{1b}[0ml\u{1b}[32mlo\u{1b}[0m W\u{1b}[32mor\u{1b}[0ml\u{1b}[32md\u{1b}[0m!\", ipsum: 42, dolor: Some(\"hey\u{1b}[32m ho!\u{1b}[0m\") })\n\
            ",
        ]);
    }

    #[test]
    fn mark_unexpected_highlights_a_string_with_double_quotes() {
        let marked_string = mark_unexpected("blandit invidunt", &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string).is_equal_to("\u{1b}[31m\"blandit invidunt\"\u{1b}[0m");
    }

    #[test]
    fn mark_missing_highlights_a_string_with_double_quotes() {
        let marked_string = mark_missing("blandit invidunt", &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string).is_equal_to("\u{1b}[33m\"blandit invidunt\"\u{1b}[0m");
    }

    #[test]
    fn mark_unexpected_string_highlights_a_string_without_double_quotes() {
        let marked_string = mark_unexpected_string("blandit invidunt", &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string).is_equal_to("\u{1b}[31mblandit invidunt\u{1b}[0m");
    }

    #[test]
    fn mark_missing_string_highlights_a_string_without_double_quotes() {
        let marked_string = mark_missing_string("blandit invidunt", &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string).is_equal_to("\u{1b}[33mblandit invidunt\u{1b}[0m");
    }

    #[test]
    fn mark_unexpected_highlights_a_char_with_single_quotes() {
        let marked_char = mark_unexpected(&'R', &DIFF_FORMAT_RED_GREEN);

        assert_that(marked_char).is_equal_to("\u{1b}[31m'R'\u{1b}[0m");
    }

    #[test]
    fn mark_missing_highlights_a_char_with_single_quotes() {
        let marked_char = mark_missing(&'R', &DIFF_FORMAT_RED_GREEN);

        assert_that(marked_char).is_equal_to("\u{1b}[32m'R'\u{1b}[0m");
    }

    #[test]
    fn mark_unexpected_char_highlights_char_without_single_quotes() {
        let marked_char = mark_unexpected_char('R', &DIFF_FORMAT_RED_GREEN);

        assert_that(marked_char).is_equal_to("\u{1b}[31mR\u{1b}[0m");
    }

    #[test]
    fn mark_missing_char_highlights_char_without_single_quotes() {
        let marked_char = mark_missing_char('R', &DIFF_FORMAT_RED_GREEN);

        assert_that(marked_char).is_equal_to("\u{1b}[32mR\u{1b}[0m");
    }

    #[test]
    fn mark_unexpected_substring_in_string_highlights_a_substring_within_the_string_as_unexpected()
    {
        let marked_string = mark_unexpected_substring_in_string(
            "mollit est eu amet",
            "st eu a",
            &DIFF_FORMAT_RED_BLUE,
        );

        assert_that(marked_string).is_equal_to("mollit e\u{1b}[31mst eu a\u{1b}[0mmet");
    }

    #[test]
    fn mark_unexpected_substring_in_string_highlights_nothing_if_the_string_does_not_contain_the_substring(
    ) {
        let marked_string = mark_unexpected_substring_in_string(
            "mollit est eu amet",
            "st eux a",
            &DIFF_FORMAT_RED_BLUE,
        );

        assert_that(marked_string).is_equal_to("mollit est eu amet");
    }

    #[test]
    fn mark_missing_substring_in_string_highlights_a_substring_within_the_string_as_missing() {
        let marked_string = mark_missing_substring_in_string(
            "mollit est eu amet",
            "t est eu",
            &DIFF_FORMAT_RED_BLUE,
        );

        assert_that(marked_string).is_equal_to("molli\u{1b}[34mt est eu\u{1b}[0m amet");
    }

    #[test]
    fn mark_missing_substring_in_string_highlights_nothing_if_the_string_does_not_contain_the_substring(
    ) {
        let marked_string = mark_missing_substring_in_string(
            "mollit est eu amet",
            "xt est eu",
            &DIFF_FORMAT_RED_BLUE,
        );

        assert_that(marked_string).is_equal_to("mollit est eu amet");
    }

    #[test]
    fn mark_unexpected_char_in_string_highlights_all_occurences_of_a_char_within_a_string_as_unexpected(
    ) {
        let marked_string =
            mark_unexpected_char_in_string("zzril mazim sint", 'z', &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string)
            .is_equal_to("\u{1b}[31mzz\u{1b}[0mril ma\u{1b}[31mz\u{1b}[0mim sint");
    }

    #[test]
    fn mark_unexpected_char_in_string_highlights_nothing_if_the_string_does_not_contain_the_character(
    ) {
        let marked_string =
            mark_unexpected_char_in_string("zzril mazim sint", 'v', &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string).is_equal_to("zzril mazim sint");
    }

    #[test]
    fn mark_missing_char_in_string_highlights_all_occurences_of_a_char_within_a_string_as_missing()
    {
        let marked_string =
            mark_missing_char_in_string("zzril mazim sint", 'z', &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string)
            .is_equal_to("\u{1b}[33mzz\u{1b}[0mril ma\u{1b}[33mz\u{1b}[0mim sint");
    }

    #[test]
    fn mark_missing_char_in_string_highlights_nothing_if_the_string_does_not_contain_the_character()
    {
        let marked_string =
            mark_missing_char_in_string("zzril mazim sint", 'e', &DIFF_FORMAT_RED_YELLOW);

        assert_that(marked_string).is_equal_to("zzril mazim sint");
    }

    #[test]
    fn mark_selected_chars_in_string_as_unexpected_highlights_selected_characters_as_unexpected() {
        let marked_string = mark_selected_chars_in_string_as_unexpected(
            "rebum placerat consetetur",
            &[0, 8, 9, 10, 24].into(),
            &DIFF_FORMAT_RED_YELLOW,
        );

        assert_that(marked_string).is_equal_to(
            "\u{1b}[31mr\u{1b}[0mebum pl\u{1b}[31mace\u{1b}[0mrat consetetu\u{1b}[31mr\u{1b}[0m",
        );
    }

    #[test]
    fn mark_selected_chars_in_string_as_unexpected_with_empty_selection_highlights_nothing() {
        let marked_string = mark_selected_chars_in_string_as_unexpected(
            "rebum placerat consetetur",
            &[].into(),
            &DIFF_FORMAT_RED_YELLOW,
        );

        assert_that(marked_string).is_equal_to("rebum placerat consetetur");
    }

    #[test]
    fn mark_selected_chars_in_string_as_missing_highlights_selected_characters_as_missing() {
        let marked_string = mark_selected_chars_in_string_as_missing(
            "rebum placerat consetetur",
            &[0, 8, 9, 10, 20, 21].into(),
            &DIFF_FORMAT_RED_YELLOW,
        );

        assert_that(marked_string).is_equal_to(
            "\u{1b}[33mr\u{1b}[0mebum pl\u{1b}[33mace\u{1b}[0mrat conse\u{1b}[33mte\u{1b}[0mtur",
        );
    }

    #[test]
    fn mark_selected_chars_in_string_as_missing_with_empty_selection_highlights_nothing() {
        let marked_string = mark_selected_chars_in_string_as_missing(
            "rebum placerat consetetur",
            &[].into(),
            &DIFF_FORMAT_RED_YELLOW,
        );

        assert_that(marked_string).is_equal_to("rebum placerat consetetur");
    }

    #[test]
    fn mark_selected_items_in_collection_for_empty_collection() {
        let collection: &[usize] = &[];
        let selected: HashSet<usize> = [1, 4].into();

        let marked_collection = mark_selected_items_in_collection(
            collection,
            &selected,
            &DIFF_FORMAT_RED_GREEN,
            mark_missing,
        );

        assert_that(marked_collection).is_equal_to("[]");
    }

    #[test]
    fn mark_all_items_in_collection_for_empty_collection() {
        let collection: &[usize] = &[];

        let marked_collection =
            mark_all_items_in_collection(collection, &DIFF_FORMAT_RED_GREEN, mark_missing);

        assert_that(marked_collection).is_equal_to("[]");
    }

    #[test]
    fn mark_selected_entries_in_map_for_empty_map() {
        let map: HashMap<String, usize> = HashMap::new();
        let map_entries: Vec<_> = map.iter().collect();
        let selected: HashSet<usize> = [1, 4].into();

        let marked_map = mark_selected_entries_in_map(
            &map_entries,
            &selected,
            &DIFF_FORMAT_RED_GREEN,
            mark_missing,
        );

        assert_that(marked_map).is_equal_to("{}");
    }

    #[test]
    fn mark_all_entries_in_map_for_empty_map() {
        let map: HashMap<String, usize> = HashMap::new();
        let map_entries: Vec<_> = map.iter().collect();

        let marked_map =
            mark_all_entries_in_map(&map_entries, &DIFF_FORMAT_RED_GREEN, mark_missing);

        assert_that(marked_map).is_equal_to("{}");
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
    use crate::env;
    use proptest::prelude::*;

    #[test]
    fn get_configured_diff_format_when_env_var_not_set() {
        env::remove_var(ENV_VAR_HIGHLIGHT_DIFFS);

        let diff_format = configured_diff_format();

        assert_that(diff_format).is_equal_to(DEFAULT_DIFF_FORMAT);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_not_set_and_no_color_env_var_set() {
        env::remove_var(ENV_VAR_HIGHLIGHT_DIFFS);
        env::set_var("NO_COLOR", "1");

        let diff_format = configured_diff_format();

        env::remove_var("NO_COLOR");

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_unknown_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "not-valid");

        let diff_format = configured_diff_format();

        assert_that(diff_format).is_equal_to(DEFAULT_DIFF_FORMAT);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_bold_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "bold");

        let diff_format = configured_diff_format();

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_BOLD);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_bold_mode_and_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "bold");
        env::set_var("NO_COLOR", "1");

        let diff_format = configured_diff_format();

        env::remove_var("NO_COLOR");

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_BOLD);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_red_green_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");

        let diff_format = configured_diff_format();

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_RED_GREEN);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_red_green_mode_and_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");
        env::set_var("NO_COLOR", "1");

        let diff_format = configured_diff_format();

        env::remove_var("NO_COLOR");

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_red_blue_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-blue");

        let diff_format = configured_diff_format();

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_RED_BLUE);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_red_blue_mode_and_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-blue");
        env::set_var("NO_COLOR", "1");

        let diff_format = configured_diff_format();

        env::remove_var("NO_COLOR");

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_red_yellow_mode() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-yellow");

        let diff_format = configured_diff_format();

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_RED_YELLOW);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_red_yellow_mode_and_no_color_env_var_set() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-yellow");
        env::set_var("NO_COLOR", "1");

        let diff_format = configured_diff_format();

        env::remove_var("NO_COLOR");

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    fn get_configured_diff_format_when_env_var_set_to_off() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "off");

        let diff_format = configured_diff_format();

        assert_that(diff_format).is_equal_to(DIFF_FORMAT_NO_HIGHLIGHT);
    }

    proptest! {
        #[test]
        fn setting_env_var_to_bold_is_case_insensitive(
            mode in "[bB][oO][lL][dD]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, &mode);

            let diff_format = configured_diff_format();

            prop_assert_eq!(diff_format, DIFF_FORMAT_BOLD);
        }

        #[test]
        fn setting_env_var_to_red_blue_is_case_insensitive(
            mode in "[rR][eE][dD]-[bB][lL][uU][eE]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, &mode);

            let diff_format = configured_diff_format();

            prop_assert_eq!(diff_format, DIFF_FORMAT_RED_BLUE);
        }

        #[test]
        fn setting_env_var_to_red_yellow_is_case_insensitive(
            mode in "[rR][eE][dD]-[yY][eE][lL][lL][oO][wW]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, &mode);

            let diff_format = configured_diff_format();

            prop_assert_eq!(diff_format, DIFF_FORMAT_RED_YELLOW);
        }

        #[test]
        fn setting_env_var_to_off_is_case_insensitive(
            mode in "[oO][fF][fF]"
        ) {
            env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, &mode);

            let diff_format = configured_diff_format();

            prop_assert_eq!(diff_format, DIFF_FORMAT_NO_HIGHLIGHT);
        }
    }

    #[test]
    fn assert_that_sets_the_diff_format_to_red_green() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");

        let assertion = assert_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_RED_GREEN);
    }

    #[test]
    fn verify_that_sets_the_diff_format_to_no_highlighting() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");

        let assertion = verify_that(42);

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }

    #[test]
    fn assert_that_code_sets_the_diff_format_to_red_green() {
        env::set_var(ENV_VAR_HIGHLIGHT_DIFFS, "red-green");

        let assertion = assert_that_code(|| {});

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_RED_GREEN);
    }

    #[test]
    fn verify_that_code_sets_the_diff_format_to_no_highlighting() {
        let assertion = verify_that_code(|| {});

        assert_that(assertion.diff_format()).is_equal_to(&DIFF_FORMAT_NO_HIGHLIGHT);
    }
}
