//! This example demonstrates how to write a custom assertion for a custom
//! struct by reusing existing assertion methods.
//!
//! We define the extension trait `AssertSnake` with the custom assertion method
//! `has_body`. This trait is implemented for `Spec`. The implementation reuses
//! built-in assertions on each field of the snake. In case of failing
//! assertions, the names of the fields are printed with the failure message.
//!
//! The custom assertion method does not fail on the first failure but collects
//! all failures first and then fails, according to the current
//! `FailingStrategy` of the `Spec`.
//!
//! Running the example will print a failed assertion to the console:
//!
//! ```console
//! thread 'main' (28052) panicked at examples\custom_assertion_reusing_existing.rs:122:25:
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
//! [image of colored output in the console](custom_assertion_reusing_existing.png)

// just to prevent some linter warnings
mod fixture;

use asserting::prelude::*;
use asserting::spec::{FailingStrategy, Spec};
use std::borrow::Borrow;

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

/// Custom assertion methods for `Snake`.
trait AssertSnake {
    #[track_caller]
    fn has_body(self, expected_body: &[Coord]) -> Self;
}

// we implement the `AssertSnake` trait for a generic `S: Borrow<Snake>` so that
// the assertion method `has_body` can be called on owned and borrowed `Snake`
// instances.
impl<S, R> AssertSnake for Spec<'_, S, R>
where
    S: Borrow<Snake>,
    R: FailingStrategy,
{
    fn has_body(mut self, expected_body: &[Coord]) -> Self {
        let actual_body = self.subject().borrow();
        // we first collect all failures using the "soft assertion" mode of
        // asserting, which is started by using the `verify_that` function.
        let mut failures;
        failures = verify_that(actual_body)
            // `verify_that` does not highlight differences by default, so we
            // switch on highlighting using the configured `DiffFormat`
            .with_configured_diff_format()
            .extracting(|s| s.length)
            .named("snake.length")
            .is_equal_to(expected_body.len())
            .display_failures();
        failures.extend(
            verify_that(actual_body)
                .with_configured_diff_format()
                .extracting(|s| &s.body)
                .named("snake.body")
                .contains_exactly(expected_body)
                .display_failures(),
        );
        failures.extend(
            verify_that(actual_body)
                .with_configured_diff_format()
                .extracting(|s| s.head)
                .named("snake.head")
                .is_equal_to(expected_body[0])
                .display_failures(),
        );
        // if there are failures, we fail the whole assertion according to the
        // current `FailingStrategy`.
        if !failures.is_empty() {
            self.do_fail_with_message(format!(
                "assertion of snake body failed: \n\n{}",
                failures.join("\n")
            ));
        }
        self
    }
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

    assert_that!(snake).has_body(&[
        Coord { x: 2, y: 1 },
        Coord { x: 1, y: 1 },
        Coord { x: 1, y: 2 },
    ]);
}

fn main() {
    test();
}
