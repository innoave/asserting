//! Check the version number of this crate specified in the crate root and the
//! README.

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    use anyhow as _;
    use asserting as _;
    #[cfg(feature = "float")]
    use float_cmp as _;
    use hashbrown as _;
}

#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
