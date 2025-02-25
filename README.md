# Asserting

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Apache-2.0 licensed][license-badge]][license-url]
![MSRV][msrv-badge]
[![code coverage][code-coverage-badge]][code-coverage-url]

A fluent assertion library for the Rust programming language.

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

    BLUE:    HEX #005AB5
             R 0 G 90 B 181
    RED:     HEX #DC3220
             R 220 G 50 B 32

<!-- Badges and related URLs -->

[crates-badge]: https://img.shields.io/crates/v/asserting.svg

[crates-url]: https://crates.io/crates/asserting

[docs-badge]: https://docs.rs/asserting/badge.svg

[docs-url]: https://docs.rs/asserting

[license-badge]: https://img.shields.io/github/license/innoave/asserting?color=blue

[license-url]: https://github.com/innoave/asserting/blob/main/LICENSE-MIT

[msrv-badge]: https://img.shields.io/crates/msrv/asserting?color=chocolate

[code-coverage-badge]: https://codecov.io/github/innoave/asserting/graph/badge.svg?token=o0w7R7J0Op

[code-coverage-url]: https://codecov.io/github/innoave/asserting
 