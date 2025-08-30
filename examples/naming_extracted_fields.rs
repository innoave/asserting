//! Better names in failure results when extracting fields from structs.

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
    assert!(failures.is_empty(), "{}", failures.join("\n"));
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
