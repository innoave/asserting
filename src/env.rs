//! Fakeable `env` module for tests.
//!
//! inspired by
//! [Testing code that uses environment variables](https://www.reddit.com/r/rust/comments/1jd8sxg/testing_code_that_uses_environment_variables/)
//! - post on Reddit

pub use std::env::VarError;

#[cfg(not(test))]
pub use std::env::var;

#[cfg(test)]
pub use fake_env::*;

#[cfg(test)]
mod fake_env {
    use fakeenv::EnvStore;
    use std::cell::RefCell;
    use std::env::VarError;

    thread_local! {
        static ENV_STORE: RefCell<EnvStore> = RefCell::new({
            let env = EnvStore::fake();
            env.remove_var("ASSERTING_HIGHLIGHT_DIFFS");
            env.remove_var("NO_COLOR");
            env
        });
    }

    pub fn var(key: &str) -> Result<String, VarError> {
        ENV_STORE.with(|env| env.borrow().var(key))
    }

    pub fn set_var(key: &str, value: &str) {
        ENV_STORE.with(|env| env.borrow_mut().set_var(key, value));
    }

    pub fn remove_var(key: &str) {
        ENV_STORE.with(|env| env.borrow_mut().remove_var(key));
    }
}
