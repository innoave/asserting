//! This file is used to generate the README.md file for crates.io.
#![allow(clippy::expect_used)]

const README_PATH: &str = "README.md";
const CRATESIO_README_PATH: &str = "docs/cratesio/README.md";
const REPOSITORY_URL: &str = "https://github.com/innoave/asserting/tree/v";

use std::{env, fs};

fn main() {
    rewrite_readme_for_cratesio_with_absolute_urls();
}

fn rewrite_readme_for_cratesio_with_absolute_urls() {
    let version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION must be set");

    println!("cargo:rerun-if-changed={README_PATH}");

    if let Ok(content) = fs::read_to_string(README_PATH) {
        let target_url = format!("]({REPOSITORY_URL}{version}/docs/");
        let updated_content = content.replace("](docs/", &target_url);

        // Write README with absolute URLs for crates.io
        let _ = fs::write(CRATESIO_README_PATH, updated_content);
    }
}
