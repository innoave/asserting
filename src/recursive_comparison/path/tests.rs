use super::*;
use crate::std::format;
use crate::std::string::ToString;

#[test]
fn path_from_empty_str() {
    let path = Path::from("");

    assert!(path.segments().is_empty());
}

#[test]
fn path_from_empty_str_is_equal_to_empty_path() {
    let path = Path::from("");

    assert_eq!(path, Path::empty());
}

#[test]
fn path_from_str_one_fields_deep() {
    let path = Path::from("foo");

    assert_eq!(path.segments(), &[Cow::Borrowed("foo")]);
}

#[test]
fn path_from_str_two_fields_deep() {
    let path = Path::from("foo.bar");

    assert_eq!(
        path.segments(),
        &[Cow::Borrowed("foo"), Cow::Borrowed("bar")]
    );
}

#[test]
fn path_from_str_three_fields_deep_with_wildcards() {
    let path = Path::from("foo.bar.*");

    assert_eq!(
        path.segments(),
        &[
            Cow::Borrowed("foo"),
            Cow::Borrowed("bar"),
            Cow::Borrowed("*")
        ]
    );
}

#[test]
fn path_from_empty_string() {
    let path = Path::from(String::new());

    assert!(path.segments().is_empty());
}

#[test]
fn path_from_empty_string_is_equal_to_empty_path() {
    let path = Path::from(String::new());

    assert_eq!(path, Path::empty());
}

#[test]
fn path_from_string_one_fields_deep() {
    let path = Path::from(String::from("foo"));

    assert_eq!(path.segments(), &[Cow::Borrowed("foo")]);
}

#[test]
fn path_from_string_two_fields_deep() {
    let path = Path::from(String::from("foo.bar"));

    assert_eq!(
        path.segments(),
        &[Cow::Borrowed("foo"), Cow::Borrowed("bar")]
    );
}

#[test]
fn path_from_string_three_fields_deep_with_wildcards() {
    let path = Path::from(String::from("foo.bar.*"));

    assert_eq!(
        path.segments(),
        &[
            Cow::Borrowed("foo"),
            Cow::Borrowed("bar"),
            Cow::Borrowed("*")
        ]
    );
}

#[test]
fn debug_string_of_empty_path() {
    let path = Path::new("");

    let debug_string = format!("{path:?}");

    assert_eq!(debug_string, "[]");
}

#[test]
fn debug_string_of_path_with_one_segment() {
    let path = Path::new("foo");

    let debug_string = format!("{path:?}");

    assert_eq!(debug_string, r#"["foo"]"#);
}

#[test]
fn debug_string_of_path_with_three_segments() {
    let path = Path::new("foo.bar.qux");

    let debug_string = format!("{path:?}");

    assert_eq!(debug_string, r#"["foo", "bar", "qux"]"#);
}

#[test]
fn display_string_of_empty_path() {
    let path = Path::new("");

    let display_string = path.to_string();

    assert_eq!(display_string, "");
}

#[test]
fn display_string_of_path_with_one_segment() {
    let path = Path::new("foo");

    let display_string = path.to_string();

    assert_eq!(display_string, "foo");
}

#[test]
fn display_string_of_path_with_three_segments() {
    let path = Path::new("foo.bar.qux");

    let display_string = path.to_string();

    assert_eq!(display_string, "foo.bar.qux");
}

#[test]
fn append_field_name_to_empty_path() {
    let path = Path::empty();

    let new_path = path.append("foo");

    assert_eq!(new_path.segments(), &[Cow::Borrowed("foo")]);
    assert!(path.segments().is_empty());
}

#[test]
fn append_field_name_to_path_one_field_deep() {
    let path = Path::new("foo");

    let new_path = path.append("bar");

    assert_eq!(
        new_path.segments(),
        &[Cow::Borrowed("foo"), Cow::Borrowed("bar")]
    );
    assert_eq!(path.segments(), &[Cow::Borrowed("foo")]);
}

#[test]
fn append_field_name_to_path_two_fields_deep() {
    let path = Path::new("foo.bar");

    let new_path = path.append("qux");

    assert_eq!(
        new_path.segments(),
        &[
            Cow::Borrowed("foo"),
            Cow::Borrowed("bar"),
            Cow::Borrowed("qux")
        ]
    );
    assert_eq!(
        path.segments(),
        &[Cow::Borrowed("foo"), Cow::Borrowed("bar")]
    );
}

#[test]
fn empty_path_start_with_empty_path() {
    assert!(Path::from("").starts_with(&Path::empty()));
}

#[test]
fn non_empty_path_does_not_start_with_empty_path() {
    assert!(!Path::from("foo.bar.baz").starts_with(&Path::empty()));
}

#[test]
fn one_field_path_starts_with_one_field_path() {
    assert!(Path::from("foo").starts_with(&Path::from("foo")));
}

#[test]
fn one_field_path_does_not_start_with_one_field_path_with_shorter_field_name() {
    assert!(!Path::from("foobar").starts_with(&Path::from("foo")));
}

#[test]
fn one_field_path_does_not_start_with_one_field_path_with_longer_field_name() {
    assert!(!Path::from("foo").starts_with(&Path::from("foobar")));
}

#[test]
fn one_field_path_does_not_start_with_two_field_path() {
    assert!(!Path::from("foo").starts_with(&Path::from("foo.bar")));
}

#[test]
fn two_fields_path_starts_with_two_fields_path() {
    assert!(Path::from("foo.bar").starts_with(&Path::from("foo.bar")));
}

#[test]
fn two_fields_path_does_not_start_with_two_fields_path_with_shorter_field_name() {
    assert!(!Path::from("foo.barx").starts_with(&Path::from("foo.bar")));
}

#[test]
fn two_fields_path_does_not_start_with_two_fields_path_with_longer_field_name() {
    assert!(!Path::from("foo.bar").starts_with(&Path::from("foo.barx")));
}

#[test]
fn two_fields_path_does_not_start_with_three_fields_path() {
    assert!(!Path::from("foo.bar").starts_with(&Path::from("foo.bar.baz")));
}

#[test]
fn two_fields_path_starts_with_one_field_path() {
    assert!(Path::from("foo.bar").starts_with(&Path::from("foo")));
}

#[test]
fn three_fields_path_starts_with_three_fields_path() {
    assert!(Path::from("foo.bar.baz").starts_with(&Path::from("foo.bar.baz")));
}

#[test]
fn three_fields_path_does_not_start_with_three_fields_path_with_shorter_field_name() {
    assert!(!Path::from("foo.bar.baz").starts_with(&Path::from("foo.bar.b")));
}

#[test]
fn three_fields_path_starts_with_two_fields_path() {
    assert!(Path::from("foo.bar.baz").starts_with(&Path::from("foo.bar")));
}

#[test]
fn three_fields_path_does_not_start_with_two_fields_path_with_shorter_field_name() {
    assert!(!Path::from("foo.bar.baz").starts_with(&Path::from("foo.b")));
}

#[test]
fn three_fields_path_starts_with_one_field_path() {
    assert!(Path::from("foo.bar.baz").starts_with(&Path::from("foo")));
}
