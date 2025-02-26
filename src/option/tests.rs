use crate::prelude::*;

#[test]
fn option_of_string_has_some_str_value() {
    let subject = Some("non tempor ea delenit".to_string());

    assert_that(subject).has_value("non tempor ea delenit");
}

#[test]
fn option_of_str_has_some_str_value() {
    let subject = Some("facilisi cupiditat elitr facilisis");

    assert_that(subject).has_value("facilisi cupiditat elitr facilisis");
}

#[test]
fn option_of_str_has_some_string_value() {
    let subject = Some("invidunt commodi eros vel");

    assert_that(subject).has_value("invidunt commodi eros vel".to_string());
}
