# Asserting

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
![MSRV][msrv-badge]
[![code coverage][code-coverage-badge]][code-coverage-url]

Fluent assertions for tests in Rust that are convenient to write and easy to extend.

The goals for `asserting` are:

1. assertions are convenient to write and easy to read
2. helpful error messages in case of failing assertions
3. provide a sensible amount of assertions out of the box
4. do not require from asserted types to implement traits if it is not absolutely necessary
5. support for asserting custom types with provided assertions
6. writing custom assertions requires minimal effort
7. no-std support

### Convenient to write

The expected value does not need to be exactly of the same type as the subject. For example, instead
of writing:

```rust
#[test]
fn the_message_is_right() {
    let message = "lorem consectetur ipsum exercitation".to_string();

    assert_that!(message).is_equal_to("lorem consectetur ipsum exercitation".to_string());
}
```

with `asserting` we can write:

```rust
#[test]
fn the_message_is_right() {
    let message = "lorem consectetur ipsum exercitation".to_string();

    assert_that!(message).is_equal_to("lorem consectetur ipsum exercitation");
}
```

Note that we do not convert the expected value to a `String`.

This might seem to be a minor advantage, but when writing assertions for a collection of `String`s,
converting every expected `&str` to `String` results in lots of noise.

### Easy to extend

Easy to extend means that we can write assertions for custom types with minimal effort.

`asserting` provides three kinds of custom assertions:

1. use any predicate function as a custom assertion
2. combine provided expectations to a custom assertion
3. write custom assertions by implementing two simple traits

## no-std support

To use `asserting` in a no-std environment disable the default features. Features that do not
require std can still be added.

```toml
[dev-dependencies]
asserting = { version = "0.1", default-features = false, features = "float" }
```

## Available Assertions

### Equality

for all types that implement `PartialEq<E>` with `E` being the type of the expected value.

| assertion       | description                                              |
|-----------------|----------------------------------------------------------|
| is_equal_to     | verify that the subject is equal to an expected value    |
| is_not_equal_to | verify that the subject is not equal to a specific value |                                                 

Example:

```rust
#[test]
fn the_message_is_right() {
    let message = "lorem consectetur ipsum exercitation".to_string();

    assert_that!(message).is_equal_to("lorem consectetur ipsum exercitation");
}
```

Note: we do not have to write `.to_string()` on the expected `&str`.

### Order

for all types that implement `PartialOrd<E>` with `E` being the type of the expected value.

| assertion       | description                                                            |
|-----------------|------------------------------------------------------------------------|
| is_greater_than | verify that the subject is greater than the expected value             |                                                 
| is_less_than    | verify that the subject is less than the expected value                |
| is_at_least     | verify that the subject is greater than or equal to the expected value |                                                 
| is_at_most      | verify that the subject is less than or equal to the expected value    |

### Range

for all types `T` that implement `PartialOrd<E>` and `E` implementing `PartialOrd<T>` with `E`
being the type of the expected value.

| assertion       | description                                                          |
|-----------------|----------------------------------------------------------------------|
| is_in_range     | verify that the subject is in the expected range (closed range)      |                                                 
| is_not_in_range | verify that the subject is not in the specified range (closed range) |

## Colored failure messages

Default colors are <span style="color: green">green</span> and <span style="color: red">red</span>.

### Switch off colored output

in `~/.cargo/config.toml` add:

```toml
[env]
ASSERTING_MESSAGES_COLORED = "off"
```

no coloring in failure messages.

### Use color vision deficiency (CVD) friendly colors:

in `~/.cargo/config.toml` add:

```toml
[env]
ASSERTING_MESSAGES_COLORED = "cvd"
```

uses <span style="color: blue">blue</span> and <span style="color: red">red</span>.

good choice for CVD friendly colors is:

```text
    BLUE:    HEX #005AB5
             R 0 G 90 B 181
    RED:     HEX #DC3220
             R 220 G 50 B 32
```

<!-- Badges and related URLs -->

[crates-badge]: https://img.shields.io/crates/v/asserting.svg

[crates-url]: https://crates.io/crates/asserting

[docs-badge]: https://docs.rs/asserting/badge.svg

[docs-url]: https://docs.rs/asserting

[msrv-badge]: https://img.shields.io/crates/msrv/asserting?color=chocolate

[code-coverage-badge]: https://codecov.io/github/innoave/asserting/graph/badge.svg?token=o0w7R7J0Op

[code-coverage-url]: https://codecov.io/github/innoave/asserting
 