//! Check the version number of this crate specified in the crate root and the
//! README.

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    use anyhow as _;
    use asserting as _;
    #[cfg(feature = "float-cmp")]
    use float_cmp as _;
    use hashbrown as _;
    #[cfg(feature = "num-bigint")]
    use lazy_static as _;
    #[cfg(feature = "num-bigint")]
    use num_bigint as _;
    use proptest as _;
    #[cfg(feature = "regex")]
    use regex as _;
    #[cfg(feature = "colored")]
    use sdiff as _;
    use serial_test as _;
    use time as _;
}

#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
