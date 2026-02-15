//! This example demonstrates that writing a helper function for asserting a
//! custom type is often easier than writing custom assertions.
//!
//! The helper function `assert_snake_body` is used to assert the state of a
//! snake. It reuses built-in assertions on each field. In case of failing
//! assertions, the names of the fields are printed with the failure message.
//!
//! Running the example will print a failed assertion to the console:
//!
//! ```console
//! thread 'main' (5388) panicked at examples\assertion_function.rs:107:5:
//! assertion of snake body failed:
//!
//! expected snake.length to be equal to 3
//!    but was: 2
//!   expected: 3
//!
//! expected snake.body to contain exactly in order [Coord { x: 2, y: 1 }, Coord { x: 1, y: 1 }, Coord { x: 1, y: 2 }]
//!        but was: [Coord { x: 2, y: 1 }, Coord { x: 1, y: 2 }, Coord { x: -1, y: 1 }]
//!       expected: [Coord { x: 2, y: 1 }, Coord { x: 1, y: 1 }, Coord { x: 1, y: 2 }]
//!        missing: [Coord { x: 1, y: 1 }]
//!          extra: [Coord { x: -1, y: 1 }]
//!   out-of-order: [Coord { x: 1, y: 2 }]
//!
//! expected snake.head to be equal to Coord { x: 2, y: 1 }
//!    but was: Coord { x: 3, y: 1 }
//!   expected: Coord { x: 2, y: 1 }
//! ```
//!
//! [image of colored output in the console](assertion_function.png)
//!
//! Since version 0.13.0 of `asserting` it is possible to write a custom
//! assertion method with reusing the built-in assertions. This is demonstrated
//! in the example
//! [custom_assertion_reusing_existing.rs](custom_assertion_reusing_existing.rs)
//! which uses the same `Snake` struct and same assertions as this example.

// just to prevent some linter warnings
mod fixture;

use asserting::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

struct Snake {
    length: usize,
    head: Coord,
    body: Vec<Coord>,
}

/// Helper function for asserting a snake's state.
///
/// It takes a snake and an expected body and asserts that the snake's
/// length, body, and head are equal to the expected body.
///
/// It first does assertions on all fields of the snake without panicking so
/// that all found failures are printed at once and not the first one only.
///
/// The failure messages contain names of the fields like "snake.length",
/// "snake.body" and "snake.head".
#[track_caller]
fn assert_snake_body(snake: &Snake, expected_body: &[Coord]) {
    let mut failures = verify_that!(snake)
        .with_configured_diff_format()
        .extracting(|s| s.length)
        .named("snake.length")
        .is_equal_to(expected_body.len())
        .display_failures();
    failures.extend(
        verify_that!(snake)
            .with_configured_diff_format()
            .extracting(|s| &s.body)
            .named("snake.body")
            .contains_exactly(expected_body)
            .display_failures(),
    );
    failures.extend(
        verify_that!(snake)
            .with_configured_diff_format()
            .extracting(|s| s.head)
            .named("snake.head")
            .is_equal_to(expected_body[0])
            .display_failures(),
    );
    assert!(
        failures.is_empty(),
        "assertion of snake body failed: \n\n{}",
        failures.join("\n")
    );
}

fn test() {
    let snake = Snake {
        length: 2,
        head: Coord { x: 3, y: 1 },
        body: vec![
            Coord { x: 2, y: 1 },
            Coord { x: 1, y: 2 },
            Coord { x: -1, y: 1 },
        ],
    };

    assert_snake_body(
        &snake,
        &[
            Coord { x: 2, y: 1 },
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 2 },
        ],
    );
}

fn main() {
    test();
}
