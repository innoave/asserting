use crate::prelude::*;

#[derive(PartialEq)]
struct Foo(i32);

#[derive(PartialEq)]
struct Bar(i32);

impl PartialEq<Bar> for Foo {
    fn eq(&self, other: &Bar) -> bool {
        self.0 == other.0
    }
}

#[test]
fn non_debug_is_equal_to_same_non_debug() {
    let foo = Foo(-1);

    assert_that(foo)
        .represented_as(|v, f| write!(f, "{}", v.0))
        .is_equal_to(Foo(-1));
}
