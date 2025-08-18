//! Example printing colored diffs in the terminal for a failed assertion.

// just to prevent some linter warnings
mod fixture;

use asserting::prelude::*;

#[derive(Debug, PartialEq)]
struct Foo {
    lorem: String,
    ipsum: i32,
    dolor: Option<String>,
}

fn test() {
    let subject = Foo {
        lorem: "¡Hola, Welt!".into(),
        ipsum: 42,
        dolor: Some("hey".into()),
    };

    assert_that!(subject).is_equal_to(Foo {
        lorem: "Hello World!".into(),
        ipsum: 42,
        dolor: Some("hey ho!".into()),
    });
}

fn main() {
    test();
}
