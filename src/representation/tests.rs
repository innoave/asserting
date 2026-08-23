use crate::prelude::*;
use crate::std::fmt;

#[derive(PartialEq)]
struct Foo(i32);

#[derive(PartialEq)]
struct Bar(i32);

impl PartialEq<Bar> for Foo {
    fn eq(&self, other: &Bar) -> bool {
        self.0 == other.0
    }
}

#[derive(Clone, Copy)]
pub struct FooBarRepresentation;

impl Represent<Foo> for FooBarRepresentation {
    fn represent(&self, value: &Foo, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", value.0)
    }
}

impl Represent<Bar> for FooBarRepresentation {
    fn represent(&self, value: &Bar, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", value.0)
    }
}

#[test]
fn represented_as_non_debug_is_equal_to_same_non_debug() {
    let foo = Foo(-1);

    assert_that(foo)
        .represented_as(|v, f| write!(f, "{}", v.0))
        .is_equal_to(Foo(-1));
}

#[test]
fn represented_by_non_debug_is_equal_to_same_non_debug() {
    let foo = Foo(-1);

    assert_that(foo)
        .represented_by(FooBarRepresentation)
        .is_equal_to(Foo(-1));
}

#[test]
fn represented_by_non_debug_is_equal_to_some_other_non_debug() {
    let foo = Foo(-1);

    assert_that(foo)
        .represented_by(FooBarRepresentation)
        .is_equal_to(Bar(-1));
}
