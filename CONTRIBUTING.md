# Contributing to asserting

`asserting` welcomes contribution from everyone in the form of suggestions, bug reports,
pull requests, and feedback. This document gives some guidance if you consider helping us.

There are opportunities to contribute to `asserting` at any level. It doesn't matter if you are just
getting started with Rust or are the most weathered expert, we can use your help.

**No contribution is too small and all contributions are valued.**

This guide will help you get started. Do not let this guide intimidate you. It should be considered
a map to help you navigate the process.

## Code of Conduct

This project and everyone participating in it is governed by
the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected
to uphold this code.

## Questions and Feedback

If you have a question on using the library in your project or want to share feedback, feel free to
start a discussion at [Discussions](https://github.com/innoave/asserting/discussions).

## Issues and Feature Requests

### Submitting a Bug Report

If you are facing a problem or have an idea for a new feature, please open an issue at
[Issues](https://github.com/innoave/asserting/issues).

Please provide as much information as possible about the issue to help us diagnose and resolve it.

* Describe the problem or feature request in detail. What did happen? And what did you expect to
  happen?
* Include steps to reproduce the problem or feature request.
* Include any relevant code snippets or examples.
* Include any relevant error messages or logs.
* Include any relevant environment information, such as operating system, Rust version.
* Include any relevant links or resources that may be helpful in diagnosing or resolving the issue.

### Submitting a Feature Request

When making a feature request, please make it clear what problem you intend to solve with the
feature, any ideas for how `asserting` could support solving that problem, any possible
alternatives, and any disadvantages.

### Provide supporting details to existing issues

If you are experiencing a problem described in an existing issue, you may provide additional
information or context that might help us diagnose and resolve the issue.

If you have a similar idea for a new feature as already mentioned in an existing issue, please add
your thoughts to that feature request or describe your point of view.

### Resolving an Issue

In the majority of cases, issues are resolved by opening a Pull Request. Pull Requests need to be
reviewed and approved by official project maintainers. This ensures that the proposed changes meet
the quality standard of the project.

If you consider resolving an issue, please make sure that you have at least read and understood the
chapter [Pull Request](#pull-requests) of these contributing guidelines.

## Pull Requests

Pull Requests are the way concrete changes are made to the code, documentation, and dependencies in
the `asserting` repository.

Even tiny pull requests (e.g., one-character pull request fixing a typo in API documentation) are
greatly appreciated. Before making a large change, it is usually a good idea to first open an issue
describing the change to solicit feedback and guidance. This will increase the likelihood of the PR
getting merged.

### Code Style

This project uses `Clippy` and `Rustfmt` as mandatory code style tools. We consider compiler and
linter warnings as errors. All errors and warnings must be resolved.

Please do not use annotations to silence compiler and linter warnings. If you feel that silencing
a warning is the best way to proceed, please provide a reason why this is the best solution.

There is one exception to this rule: in test code it is allowed to use `expect` on `Option` and
`Result` types. The annotation `#![allow (clippy::expect_used)]` should be placed at the top of a
test file.

### Tests

If the change being proposed alters code (as opposed to only adding or improving documentation, for
example), it is either adding new functionality to the crate or it is fixing existing, broken
functionality.

In both cases it is mandatory to include tests that demonstrate the correctness of the change and
ensure that the crate does not regress in the future.

#### Documentation Tests

Ideally, every API has at least one documentation test that demonstrates how to use the API. The
tricky part to documentation tests is striking a balance between being succinct for a reader to
understand and actually testing the API.

### Commits

It is a recommended best practice to keep your changes as logically grouped as possible within
individual commits. There is no limit to the number of commits any single Pull Request may have,
and many contributors find it easier to review changes that are split across multiple commits.

The maintainers of the project adhere to
the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
specification. Though it is not a hard requirement to follow this specification for getting Pull
Requests merged.

If a commit introduces a breaking change, it is recommended to include a note in the commit message
explaining the reason for the change and how it affects the API. This will help reviewers and
contributors understand the impact of the change and make informed decisions about merging the
Pull Request.

### Submitting a Pull Request

Before submitting a Pull Request, it is recommended to ensure that all tests pass and that the
documentation is up to date. Additionally, it is recommended to include a clear and concise
description of the changes made in the Pull Request, including any relevant context or motivation
for the change.

This project uses the `Just` tool to help ensure code quality. The `justfile` defines the recipe
`continues-integration` that performs most of the tasks in the CI pipeline in your (local)
development environment. To execute the recipe, run `just continues-integration` or use the short
alias `just ci`.

We heavily use `Clippy` and `Rustfmt` to ensure code quality and consistency. Please run `just ci`
before submitting a Pull Request to ensure that the code meets the project's standards.

The CI pipeline runs on any Pull Request. Errors and warnings found by the CI pipeline must be
addressed before the Pull Request can be merged.

### Reviewing Pull Requests

**Any community member is welcome to review any Pull Request.**

All contributors who choose to review and provide feedback on Pull Requests have a responsibility
to both the project and the individual making the contribution. Reviews and feedback must be
helpful, insightful, and geared towards improving the contribution as opposed to simply blocking it.
If there are reasons why you feel the PR should not land, explain what those are. Do not expect to
be able to block a Pull Request from advancing simply because you say "No" without giving an
explanation. Be open to having your mind changed. Be open to working with the contributor to make
the Pull Request better.

Reviews that are dismissive or disrespectful of the contributor or any other reviewers are strictly
counter to the Code of Conduct.

When reviewing a Pull Request, the primary goals are for the codebase to improve and for the person
submitting the request to succeed. Even if a Pull Request does not land, the submitters should come
away from the experience feeling like their effort was not wasted or unappreciated. Every Pull
Request from a new contributor is an opportunity to grow the community.
