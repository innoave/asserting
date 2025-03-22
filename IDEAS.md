# Ideas

Unsorted and unweighted informal notes about ideas for `asserting` and related research.

## Colored failure messages

Default colors are <span style="color: green">green</span> and <span style="color: red">red</span>.

### Switch off colored output

in `~/.cargo/config.toml` add:

```toml,no_sync
[env]
ASSERTING_MESSAGES_COLORED = "off"
```

no coloring in failure messages.

### Use color vision deficiency (CVD) friendly colors:

in `~/.cargo/config.toml` add:

```toml,no_sync
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

