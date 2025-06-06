use crate::prelude::*;
use crate::std::format;
use proptest::prelude::*;

#[test]
fn char_is_lowercase() {
    assert_that('m').is_lowercase();
}

#[test]
fn verify_char_is_lowercase_fails() {
    let failures = verify_that('M').is_lowercase().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is lowercase
   but was: M
  expected: m
"]
    );
}

#[test]
fn borrowed_char_is_lowercase() {
    assert_that(&'m').is_lowercase();
}

#[test]
fn verify_borrowed_char_is_lowercase_fails() {
    let failures = verify_that(&'M').is_lowercase().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is lowercase
   but was: M
  expected: m
"]
    );
}

#[test]
fn char_is_uppercase() {
    assert_that('K').is_uppercase();
}

#[test]
fn verify_char_is_uppercase_fails() {
    let failures = verify_that('k').is_uppercase().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is uppercase
   but was: k
  expected: K
"]
    );
}

#[test]
fn borrowed_char_is_uppercase() {
    assert_that(&'X').is_uppercase();
}

#[test]
fn verify_borrowed_char_is_uppercase_fails() {
    let failures = verify_that(&'x').is_uppercase().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is uppercase
   but was: x
  expected: X
"]
    );
}

#[test]
fn char_is_ascii() {
    assert_that('@').is_ascii();
}

#[test]
fn verify_char_is_ascii_fails() {
    let failures = verify_that('€').is_ascii().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is an ASCII character
   but was: €
  expected: an ASCII character
"]
    );
}

#[test]
fn borrowed_char_is_ascii() {
    assert_that(&'@').is_ascii();
}

#[test]
fn verify_borrowed_char_is_ascii_fails() {
    let failures = verify_that(&'❤').is_ascii().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is an ASCII character
   but was: ❤
  expected: an ASCII character
"]
    );
}

#[test]
fn char_is_alphabetic() {
    assert_that('L').is_alphabetic();
}

#[test]
fn verify_char_is_alphabetic_fails() {
    let failures = verify_that('1').is_alphabetic().display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is an alphabetic character
   but was: 1
  expected: an alphabetic character
"
        ]
    );
}

#[test]
fn borrowed_char_is_alphabetic() {
    assert_that(&'a').is_alphabetic();
}

#[test]
fn verify_borrowed_char_is_alphabetic_fails() {
    let failures = verify_that(&'@').is_alphabetic().display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is an alphabetic character
   but was: @
  expected: an alphabetic character
"
        ]
    );
}

#[test]
fn char_is_alphanumeric() {
    assert_that('L').is_alphanumeric();
    assert_that('1').is_alphanumeric();
}

#[test]
fn verify_char_is_alphanumeric_fails() {
    let failures = verify_that('@').is_alphanumeric().display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is an alphanumeric character
   but was: @
  expected: an alphanumeric character
"
        ]
    );
}

#[test]
fn borrowed_char_is_alphanumeric() {
    assert_that(&'a').is_alphanumeric();
    assert_that(&'0').is_alphanumeric();
}

#[test]
fn verify_borrowed_char_is_alphanumeric_fails() {
    let failures = verify_that(&'+').is_alphanumeric().display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is an alphanumeric character
   but was: +
  expected: an alphanumeric character
"
        ]
    );
}

#[test]
fn char_is_control_char() {
    assert_that('\u{1b}').is_control_char();
}

#[test]
fn verify_char_is_control_char_fails() {
    let failures = verify_that('[').is_control_char().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is a control character
   but was: [
  expected: a control character
"]
    );
}

#[test]
fn borrowed_char_is_control_char() {
    assert_that(&'\t').is_control_char();
}

#[test]
fn verify_borrowed_char_is_control_char_fails() {
    let failures = verify_that(&'@').is_control_char().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is a control character
   but was: @
  expected: a control character
"]
    );
}

#[test]
fn char_is_digit_in_radix_10() {
    assert_that('9').is_digit(10);
}

#[test]
fn verify_char_is_digit_in_radix_10_fails() {
    let failures = verify_that('A').is_digit(10).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is a digit in the radix 10
   but was: A
  expected: a digit in the radix 10
"
        ]
    );
}

#[test]
fn borrowed_char_is_digit_in_radix_10() {
    assert_that(&'8').is_digit(10);
}

#[test]
fn verify_borrowed_char_is_digit_in_radix_10_fails() {
    let failures = verify_that(&'F').is_digit(10).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is a digit in the radix 10
   but was: F
  expected: a digit in the radix 10
"
        ]
    );
}

#[test]
fn char_is_digit_in_radix_16() {
    assert_that('B').is_digit(16);
}

#[test]
fn verify_char_is_digit_in_radix_16_fails() {
    let failures = verify_that('G').is_digit(16).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is a digit in the radix 16
   but was: G
  expected: a digit in the radix 16
"
        ]
    );
}

#[test]
fn borrowed_char_is_digit_in_radix_16() {
    assert_that(&'c').is_digit(16);
}

#[test]
fn verify_borrowed_char_is_digit_in_radix_16_fails() {
    let failures = verify_that(&'g').is_digit(16).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is a digit in the radix 16
   but was: g
  expected: a digit in the radix 16
"
        ]
    );
}

#[test]
fn char_is_digit_in_radix_7() {
    assert_that('6').is_digit(7);
}

#[test]
fn verify_char_is_digit_in_radix_7_fails() {
    let failures = verify_that('7').is_digit(7).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is a digit in the radix 7
   but was: 7
  expected: a digit in the radix 7
"
        ]
    );
}

#[test]
fn borrowed_char_is_digit_in_radix_7() {
    assert_that(&'0').is_digit(7);
}

#[test]
fn verify_borrowed_char_is_digit_in_radix_7_fails() {
    let failures = verify_that(&'9').is_digit(7).display_failures();

    assert_eq!(
        failures,
        &[
            r"assertion failed: expected subject is a digit in the radix 7
   but was: 9
  expected: a digit in the radix 7
"
        ]
    );
}

#[test]
fn char_is_whitespace() {
    assert_that(' ').is_whitespace();
}

#[test]
fn verify_char_is_whitespace_fails() {
    let failures = verify_that('_').is_whitespace().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is a whitespace
   but was: _
  expected: a whitespace
"]
    );
}

#[test]
fn borrowed_char_is_whitespace() {
    assert_that(&'\t').is_whitespace();
}

#[test]
fn verify_borrowed_char_is_whitespace_fails() {
    let failures = verify_that(&'=').is_whitespace().display_failures();

    assert_eq!(
        failures,
        &[r"assertion failed: expected subject is a whitespace
   but was: =
  expected: a whitespace
"]
    );
}

proptest! {
    #[test]
    fn asserting_ascii_and_lowercase_is_equivalent_to_ascii_lowercase_method(
        chr in any::<char>(),
    ) {
        prop_assert_eq!(chr.is_ascii() && chr.is_lowercase(), chr.is_ascii_lowercase());
    }

    #[test]
    fn asserting_ascii_and_uppercase_is_equivalent_to_ascii_uppercase_method(
        chr in any::<char>(),
    ) {
        prop_assert_eq!(chr.is_ascii() && chr.is_uppercase(), chr.is_ascii_uppercase());
    }
}
