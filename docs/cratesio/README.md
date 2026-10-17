# Asserting

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
![MSRV][msrv-badge]
[![code coverage][code-coverage-badge]][code-coverage-url]

Fluent assertions for tests in Rust. An extensive collection of assertions to express the intent of
the expected result in a test.

Fluent assertions help with expressing the intent of a test. Other developers quickly understand
what the expected result of a test should be. One remembers more easily what could be wrong in the
case of a test fails some months later due to a regression. The autocomplete feature of a coding
editor or IDE helps with writing assertions as it suggests possible assertion methods. Distinct and
more helpful error messages help to spot the difference between the expected value and the actual
result. Fluent assertions read like natural English.

Features of `asserting`:

* assertions are convenient to write and easy to read
* helpful error messages in case of failing assertions
* colored diffs between expected and actual values
  (see ["Highlighted differences"](#highlighted-differences))
* chaining of multiple assertions on the same subject (see ["Chaining assertions"])
* concise and expressive assertions for more complex types like collections
* field-by-field recursive comparison (see ["Field-by-field recursive comparison"])
* soft assertions (execute multiple assertions before panicking) (see ["Soft assertions"])
* support for asserting custom types with provided assertions (see ["Asserting custom types"])
* custom representation of values in failure reports or if a (foreign) type does not implement
  [`Debug`] (see ["Type formatting (aka Representation)"])
* provide a reasonable number of assertions out of the box
* do not require that asserted types have to implement traits if it is not absolutely necessary
* custom assertions (see ["Custom assertions"](#custom-assertions))
* support no-std environments

For an overview of the provided features and many examples on how to use `asserting` see the
[crate-level documentation][docs-url].

The assertion methods are mostly inspired by the [AssertJ] library for Java but adopted to Rust's
syntax and idioms.

To see what is changed with each release, see the [changelog](CHANGELOG.md).

## no-std support

To use `asserting` in a no-std environment, disable the default features. Features that do not
require std can still be added.

```toml
[dev-dependencies]
asserting = { version = "0.17", default-features = false, features = ["colored", "float-cmp", "recursive", "regex"] }
```

Note that an allocator is needed for no-std.

## Crate Features

Overview of the crate features of `asserting`. The column "no-std" specifies if this feature is
available in no-std environments. The column "default" specifies if this feature is enabled by
default.

| Feature        | Description                                                                                                                         | no-std | default |
|----------------|-------------------------------------------------------------------------------------------------------------------------------------|:------:|:-------:|
| `std`          | Use the `std` library                                                                                                               |   no   |   yes   |
| `colored`      | Colored highlighting of differences between actual and expected value                                                               |  yes   |   yes   |
| `recursive`    | Field-by-field recursive comparison mode                                                                                            |  yes   |   yes   |
| `float-cmp`    | Floating point comparison (`ìs_close_to`)                                                                                           |  yes   |   yes   |
| `regex`        | String matches Regex assertions (`matching`)                                                                                        |  yes   |   yes   |
| `panic`        | Assert that code panics (with the expected message)                                                                                 |   no   |   yes   |
| `num-bigint`   | Enhanced support for `num-bigint::BigInt`                                                                                           |  yes   |   no    |
| `bigdecimal`   | Enhanced support for `bigdecimal::BigDecimal`                                                                                       |  yes   |   no    |
| `rust-decimal` | Enhanded support for `rust_decimal::Decimal`                                                                                        |  yes   |   no    |
| `temporal`     | Support for writing assertions for temporal types                                                                                   |  yes   |   no    |
| `chrono`       | `is_close_to_with_margin` assertion for `chrono` types:<br/> `NaiveDate`, `NaiveTime`, `NaiveDateTime` and `DateTime`               |  yes   |   no    |
| `jiff`         | `is_close_to_with_margin` assertion for `jiff` types:<br/> `Timestamp`, `civil::Date`, `civil::Time`, `civil::DateTime` and `Zoned` |  yes   |   no    |
| `time`         | `is_close_to_with_margin` assertion for `time` types:<br/> `Timestamp`, `Date`, `Time`, `PlainDateTime` and `OffsetDateTime`        |  yes   |   no    |

## Highlighted differences

`asserting` can highlight the differences between the expected value (s) and the actual value (s)
when printing assertion failures to the terminal. The colored diffs in assertion failures look like
this:

![colored diffs in terminal](examples/colored_diffs.png)

Highlighted differences in the failure message help with spotting the difference between actual and
expected values and make finding the reason for a failing test much easier.

It supports different variants of how differences are highlighted.

| Mode       | Effect                                                                                                                            |
|------------|-----------------------------------------------------------------------------------------------------------------------------------|
| bold       | Differences are printed in bold letters, without coloring.                                                                        | 
| red-green  | Differences are printed in the colors <span style="color: green">green</span> and <span style="color: red">red</span>.            | 
| red-blue   | Differences are printed in the CVD-friendly colors <span style="color: blue">blue</span> and <span style="color: red">red</span>. | 
| red-yellow | Differences are printed in the colors <span style="color: yellow">yellow</span> and <span style="color: red">red</span>.          | 
| off        | Switches off highlighting. The differences are not highlighted at all.                                                            | 

The mode can be configured by setting the environment variable `ASSERTING_HIGHLIGHT_DIFFS` to one of
the modes in the table above. The value is case-insensitive. E.g., setting the environment variable
to values like `Red-Blue`, `Bold` or `OFF` works as well.

The intended way for configuring the highlighting mode is to set the environment variable in the
configuration for `Cargo` by adding it to the `[env]` section in your `~/.cargo/config.toml` file:

```toml,no_sync
[env]
ASSERTING_HIGHLIGHT_DIFFS = "red-blue"
```

By default, the mode `red-green` is used. Differences are colored in
<span style="color: green">green</span> and <span style="color: red">red</span>.

Differences are only highlighted if the crate feature `colored` is enabled. The configuration via
the environment variable only works when the crate feature `std` is enabled too. In no-std projects,
the default colors red and green are used.

*Asserting* respects the [`NO_COLOR`] environment variable. If the `NO_COLOR` environment variable
is set, no colors are used regardless of the configured highlight mode.

## Available Assertions

The `asserting` crate provides a broad set of assertions for testing and validation purposes. An
overview of the available assertions can be found in the
[Available Assertions](https://github.com/innoave/asserting/tree/v0.17.0/docs/available-assertions.md) document.

All provided assertions are defined by an assertion-trait in the [`assertions`] module. Each
assertion method comes with a short explanation of how to use it and examples in the API
documentation. So the relevant documentation is at your hand within your editor or IDE when you
are writing assertions in tests.

The [crate-level documentation][docs-url] contains lots of examples as a quick introduction.

## Custom assertions

`asserting` provides three kinds of custom assertions:

1. use any predicate function as a custom assertion (see "[predicate as custom assertion]")
2. property-based assertions can be used with any type that implements the related property (see
   "[property-based assertions]")
3. write custom assertion methods by defining and implementing an extension trait (see
   "[custom assertions]")

The mentioned references link to a chapter in the crate's documentation that describes the
possibilities for custom assertions, including examples.

<!-- Badges and related URLs -->

[crates-badge]: https://img.shields.io/crates/v/asserting.svg

[crates-url]: https://crates.io/crates/asserting

[docs-badge]: https://docs.rs/asserting/badge.svg

[docs-url]: https://docs.rs/asserting

[msrv-badge]: https://img.shields.io/crates/msrv/asserting?color=chocolate

[code-coverage-badge]: https://codecov.io/github/innoave/asserting/graph/badge.svg?token=o0w7R7J0Op

[code-coverage-url]: https://codecov.io/github/innoave/asserting

<!-- External Links -->

["Asserting custom types"]: https://docs.rs/asserting/latest/asserting/#asserting-custom-types

["Chaining assertions"]: https://docs.rs/asserting/latest/asserting/#chaining-assertions-on-the-same-subject

["Field-by-field recursive comparison"]: https://docs.rs/asserting/latest/asserting/#field-by-field-recursive-comparison

["Type formatting (aka Representation)"]: https://docs.rs/asserting/latest/asserting/#type-formatting-aka-representation

["soft assertions"]: https://docs.rs/asserting/#soft-assertions

[custom assertions]: https://docs.rs/asserting/#custom-assertions

[predicate as custom assertion]: https://docs.rs/asserting/#predicate-as-custom-assertion

[property-based assertions]: https://docs.rs/asserting/#property-based-assertions

[`assertions`]: https://docs.rs/asserting/latest/asserting/assertions/

[`NO_COLOR`]: https://no-color.org/

[AssertJ]: https://assertj.github.io/doc/
