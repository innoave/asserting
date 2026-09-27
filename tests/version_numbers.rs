//! Check the version number of this crate specified in the crate root and the
//! README.
#![allow(unused_crate_dependencies)]

#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
