// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    use anyhow as _;
    #[cfg(feature = "float")]
    use float_cmp as _;
    use hashbrown as _;
    use proptest as _;
    #[cfg(feature = "colored")]
    use sdiff as _;
    use serial_test as _;
    use version_sync as _;
}
