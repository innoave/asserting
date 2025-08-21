//! Implement a custom assertion for own struct, reusing existing assertions

// just to prevent some linter warnings
mod fixture;

use asserting::expectations::{is_equal_to, iterator_contains_exactly};
use asserting::prelude::*;
use asserting::spec::{DiffFormat, Expectation, Expression, FailingStrategy, Spec};

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

struct HasBody {
    expected: Vec<Coord>,
    length: bool,
    head: bool,
    body: bool,
}

impl HasBody {
    fn new(expected: Vec<Coord>) -> Self {
        Self {
            expected,
            length: false,
            head: false,
            body: false,
        }
    }
}

impl Expectation<Snake> for HasBody {
    fn test(&mut self, subject: &Snake) -> bool {
        self.length = subject.length == self.expected.len();
        self.head = subject.head == self.expected[0];
        self.body = subject.body == self.expected;
        self.length && self.head && self.body
    }

    fn message(
        &self,
        expression: &Expression<'_>,
        actual: &Snake,
        inverted: bool,
        format: &DiffFormat,
    ) -> String {
        let mut failure = String::new();
        if !self.length {
            failure.push_str(&is_equal_to(self.expected.len()).message(
                expression,
                &actual.length,
                inverted,
                format,
            ));
            failure.push('\n');
        }
        if !self.head {
            failure.push_str(&is_equal_to(&self.expected[0]).message(
                expression,
                &&actual.head,
                inverted,
                format,
            ));
            failure.push('\n');
        }
        if !self.body {
            failure.push_str(&iterator_contains_exactly(self.expected.clone()).message(
                expression,
                &actual.body,
                inverted,
                format,
            ));
            failure.push('\n');
        }
        failure
    }
}

trait AssertHasBody {
    fn has_body(self, expected_body: impl IntoIterator<Item = Coord>) -> Self;
}

impl<R> AssertHasBody for Spec<'_, Snake, R>
where
    R: FailingStrategy,
{
    fn has_body(self, expected_body: impl IntoIterator<Item = Coord>) -> Self {
        self.expecting(HasBody::new(Vec::from_iter(expected_body)))
    }
}

fn test() {
    let snake = Snake {
        length: 3,
        head: Coord { x: 2, y: 1 },
        body: vec![
            Coord { x: 2, y: 1 },
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 2 },
        ],
    };

    assert_that!(snake).has_body([
        Coord { x: 2, y: 1 },
        Coord { x: 1, y: 1 },
        Coord { x: 1, y: 2 },
    ]);
}

fn main() {
    test();
}
