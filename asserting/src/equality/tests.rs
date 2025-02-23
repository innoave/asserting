use crate::equality::IsEqualTo;
use crate::specification::assert_that;

#[test]
fn string_is_equal_to_string() {
    let subject = "a string result".to_string();

    assert_that(subject).is_equal_to("a string result".to_string());
}
