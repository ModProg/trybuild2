//! [![github]](https://github.com/GuillaumeGomez/trybuild2)&ensp;[![crates-io]](https://crates.io/crates/trybuild2)&ensp;[![docs-rs]](https://docs.rs/trybuild2)
//!
//! <br>
//!
//! `trybuild2` is a fork of [trybuild](https://github.com/dtolnay/trybuild)
//! which allows to have inline tests.
//!
//! #### &emsp;A compiler diagnostics testing library in just 3 functions.
//!
//! Trybuild2 is a test harness for invoking rustc on a set of test cases and
//! asserting that any resulting error messages are the ones intended.
//!
//! Such tests are commonly useful for testing error reporting involving
//! procedural macros. We would write test cases triggering either errors
//! detected by the macro or errors detected by the Rust compiler in the
//! resulting expanded code, and compare against the expected errors to ensure
//! that they remain user-friendly.
//!
//! This style of testing is sometimes called *ui tests* because they test
//! aspects of the user's interaction with a library outside of what would be
//! covered by ordinary API tests.
//!
//! Nothing here is specific to macros; trybuild2 would work equally well for
//! testing misuse of non-macro APIs.
//!
//! <br>
//!
//! # Compile-fail tests
//!
//! A minimal trybuild2 setup looks like this:
//!
//! ```
//! #[test]
//! fn ui() {
//!     let t = trybuild2::TestCases::new();
//!     t.compile_fail("tests/ui/*.rs");
//! }
//! ```
//!
//! The test can be run with `cargo test`. It will individually compile each of
//! the source files matching the glob pattern, expect them to fail to compile,
//! and assert that the compiler's error message matches an adjacently named
//! _*.stderr_ file containing the expected output (same file name as the test
//! except with a different extension). If it matches, the test case is
//! considered to succeed.
//!
//! Dependencies listed under `[dev-dependencies]` in the project's Cargo.toml
//! are accessible from within the test cases.
//!
//! <p align="center">
//! <img src="https://user-images.githubusercontent.com/1940490/57186574-76469e00-6e96-11e9-8cb5-b63b657170c9.png" width="700">
//! </p>
//!
//! Failing tests display the expected vs actual compiler output inline.
//!
//! <p align="center">
//! <img src="https://user-images.githubusercontent.com/1940490/57186575-79418e80-6e96-11e9-9478-c9b3dc10327f.png" width="700">
//! </p>
//!
//! A compile_fail test that fails to fail to compile is also a failure.
//!
//! <p align="center">
//! <img src="https://user-images.githubusercontent.com/1940490/57186576-7b0b5200-6e96-11e9-8bfd-2de705125108.png" width="700">
//! </p>
//!
//! <br>
//!
//! # Pass tests
//!
//! The same test harness is able to run tests that are expected to pass, too.
//! Ordinarily you would just have Cargo run such tests directly, but being able
//! to combine modes like this could be useful for workshops in which
//! participants work through test cases enabling one at a time. Trybuild was
//! originally developed for my [procedural macros workshop at Rust
//! Latam][workshop].
//!
//! [workshop]: https://github.com/dtolnay/proc-macro-workshop
//!
//! ```
//! #[test]
//! fn ui() {
//!     let t = trybuild2::TestCases::new();
//!     t.pass("tests/01-parse-header.rs");
//!     t.pass("tests/02-parse-body.rs");
//!     t.compile_fail("tests/03-expand-four-errors.rs");
//!     t.pass("tests/04-paste-ident.rs");
//!     t.pass("tests/05-repeat-section.rs");
//!     //t.pass("tests/06-make-work-in-function.rs");
//!     //t.pass("tests/07-init-array.rs");
//!     //t.compile_fail("tests/08-ident-span.rs");
//!     t.compile_fail_inline("name", "fn main() {}", "path-to-stderr");
//!     t.compile_fail_check_sub("tests/03-expand-four-errors.rs", "I want to find this!");
//! }
//! ```
//!
//! Pass tests are considered to succeed if they compile successfully and have a
//! `main` function that does not panic when the compiled binary is executed.
//!
//! <p align="center">
//! <img src="https://user-images.githubusercontent.com/1940490/57186580-7f376f80-6e96-11e9-9cae-8257609269ef.png" width="700">
//! </p>
//!
//! <br>
//!
//! # Details
//!
//! That's the entire API.
//!
//! <br>
//!
//! # Workflow
//!
//! There are two ways to update the _*.stderr_ files as you iterate on your
//! test cases or your library; handwriting them is not recommended.
//!
//! First, if a test case is being run as compile_fail but a corresponding
//! _*.stderr_ file does not exist, the test runner will save the actual
//! compiler output with the right filename into a directory called *wip* within
//! the directory containing Cargo.toml. So you can update these files by
//! deleting them, running `cargo test`, and moving all the files from *wip*
//! into your testcase directory.
//!
//! <p align="center">
//! <img src="https://user-images.githubusercontent.com/1940490/57186579-7cd51580-6e96-11e9-9f19-54dcecc9fbba.png" width="700">
//! </p>
//!
//! Alternatively, run `cargo test` with the environment variable
//! `TRYBUILD2=overwrite` to skip the *wip* directory and write all compiler
//! output directly in place. You'll want to check `git diff` afterward to be
//! sure the compiler's output is what you had in mind.
//!
//! <br>
//!
//! # What to test
//!
//! When it comes to compile-fail tests, write tests for anything for which you
//! care to find out when there are changes in the user-facing compiler output.
//! As a negative example, please don't write compile-fail tests simply calling
//! all of your public APIs with arguments of the wrong type; there would be no
//! benefit.
//!
//! A common use would be for testing specific targeted error messages emitted
//! by a procedural macro. For example the derive macro from the [`ref-cast`]
//! crate is required to be placed on a type that has either `#[repr(C)]` or
//! `#[repr(transparent)]` in order for the expansion to be free of undefined
//! behavior, which it enforces at compile time:
//!
//! [`ref-cast`]: https://github.com/dtolnay/ref-cast
//!
//! ```console
//! error: RefCast trait requires #[repr(C)] or #[repr(transparent)]
//!  --> $DIR/missing-repr.rs:3:10
//!   |
//! 3 | #[derive(RefCast)]
//!   |          ^^^^^^^
//! ```
//!
//! Macros that consume helper attributes will want to check that unrecognized
//! content within those attributes is properly indicated to the caller. Is the
//! error message correctly placed under the erroneous tokens, not on a useless
//! call\_site span?
//!
//! ```console
//! error: unknown serde field attribute `qqq`
//!  --> $DIR/unknown-attribute.rs:5:13
//!   |
//! 5 |     #[serde(qqq = "...")]
//!   |             ^^^
//! ```
//!
//! Declarative macros can benefit from compile-fail tests too. The [`json!`]
//! macro from serde\_json is just a great big macro\_rules macro but makes an
//! effort to have error messages from broken JSON in the input always appear on
//! the most appropriate token:
//!
//! [`json!`]: https://docs.rs/serde_json/1.0/serde_json/macro.json.html
//!
//! ```console
//! error: no rules expected the token `,`
//!  --> $DIR/double-comma.rs:4:38
//!   |
//! 4 |     println!("{}", json!({ "k": null,, }));
//!   |                                      ^ no rules expected this token in macro call
//! ```
//!
//! Sometimes we may have a macro that expands successfully but we count on it
//! to trigger particular compiler errors at some point beyond macro expansion.
//! For example the [`readonly`] crate introduces struct fields that are public
//! but readable only, even if the caller has a &mut reference to the
//! surrounding struct. If someone writes to a readonly field, we need to be
//! sure that it wouldn't compile:
//!
//! [`readonly`]: https://github.com/dtolnay/readonly
//!
//! ```console
//! error[E0594]: cannot assign to data in a `&` reference
//!   --> $DIR/write-a-readonly.rs:17:26
//!    |
//! 17 |     println!("{}", s.n); s.n += 1;
//!    |                          ^^^^^^^^ cannot assign
//! ```
//!
//! In all of these cases, the compiler's output can change because our crate or
//! one of our dependencies broke something, or as a consequence of changes in
//! the Rust compiler. Both are good reasons to have well conceived compile-fail
//! tests. If we refactor and mistakenly cause an error that used to be correct
//! to now no longer be emitted or be emitted in the wrong place, that is
//! important for a test suite to catch. If the compiler changes something that
//! makes error messages that we care about substantially worse, it is also
//! important to catch and report as a compiler issue.
//!
//! # Inline checks
//!
//! It's possible to provide a code to compile directly through
//! `compile_fail_inline` and `pass_inline`. It allows you to have some more
//! generic checks which can be generated at runtime if needed.
//!
//! # Sub-string checks
//!
//! If you need  more control over the stderr check as well, you can take a
//! look at `compile_fail_check_sub` and `compile_fail_inline_check_sub`.

#![doc(html_root_url = "https://docs.rs/trybuild2/1.0.2")]
#![allow(
    clippy::collapsible_if,
    clippy::default_trait_access,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::iter_not_returning_iterator, // https://github.com/rust-lang/rust-clippy/issues/8285
    clippy::let_underscore_untyped, // https://github.com/rust-lang/rust-clippy/issues/10410
    clippy::manual_assert,
    clippy::manual_range_contains,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::non_ascii_literal,
    clippy::range_plus_one,
    clippy::similar_names,
    clippy::single_match_else,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref,
    clippy::unused_self,
    clippy::while_let_on_iterator,
)]
#![deny(clippy::clone_on_ref_ptr)]

#[macro_use]
mod term;

#[macro_use]
mod path;

mod cargo;
mod dependencies;
mod diff;
mod directory;
mod env;
mod error;
mod expand;
mod features;
mod flock;
mod inherit;
mod manifest;
mod message;
mod normalize;
mod run;
mod rustflags;

use std::cell::RefCell;
use std::panic::RefUnwindSafe;
use std::path::{Path, PathBuf};
use std::thread;

#[derive(Debug)]
pub struct TestCases {
    runner: RefCell<Runner>,
}

#[derive(Debug)]
struct Runner {
    tests: Vec<Test>,
}

#[derive(Clone, Debug)]
struct Test {
    expected: Expected,
    inner: TestKind,
    path: PathBuf,
}

#[derive(Clone, Debug)]
enum TestKind {
    File,
    Inline(InlineTest),
}

#[derive(Clone, Debug)]
struct InlineTest {
    code: String,
    name: String,
    stderr_path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
enum Expected {
    Pass,
    PassSubString(String),
    CompileFail,
    CompileFailSubString(String),
}

impl TestCases {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        TestCases {
            runner: RefCell::new(Runner { tests: Vec::new() }),
        }
    }

    pub fn pass<P: AsRef<Path>>(&self, path: P) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::Pass,
            path: path.as_ref().to_owned(),
            inner: TestKind::File,
        });
    }

    pub fn pass_inline(&self, name: &str, code: &str) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::Pass,
            path: PathBuf::from(name),
            inner: TestKind::Inline(InlineTest {
                code: code.to_owned(),
                name: name.to_owned(),
                stderr_path: None,
            }),
        });
    }

    pub fn pass_check_sub<P: AsRef<Path>>(&self, path: P, sub_string: &str) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::PassSubString(sub_string.to_owned()),
            path: path.as_ref().to_owned(),
            inner: TestKind::File,
        });
    }

    pub fn pass_inline_check_sub(&self, name: &str, code: &str, sub_string: &str) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::PassSubString(sub_string.to_owned()),
            path: PathBuf::from(name),
            inner: TestKind::Inline(InlineTest {
                code: code.to_owned(),
                name: name.to_owned(),
                stderr_path: None,
            }),
        });
    }

    pub fn compile_fail<P: AsRef<Path>>(&self, path: P) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::CompileFail,
            path: path.as_ref().to_owned(),
            inner: TestKind::File,
        });
    }

    pub fn compile_fail_inline<P: AsRef<Path>>(&self, name: &str, code: &str, stderr_path: P) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::CompileFail,
            path: PathBuf::from(name),
            inner: TestKind::Inline(InlineTest {
                code: code.to_owned(),
                name: name.to_owned(),
                stderr_path: Some(stderr_path.as_ref().to_owned()),
            }),
        });
    }

    pub fn compile_fail_check_sub<P: AsRef<Path>>(&self, path: P, sub_string: &str) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::CompileFailSubString(sub_string.to_owned()),
            path: path.as_ref().to_owned(),
            inner: TestKind::File,
        });
    }

    pub fn compile_fail_inline_check_sub(&self, name: &str, code: &str, sub_string: &str) {
        self.runner.borrow_mut().tests.push(Test {
            expected: Expected::CompileFailSubString(sub_string.to_owned()),
            path: PathBuf::from(name),
            inner: TestKind::Inline(InlineTest {
                code: code.to_owned(),
                name: name.to_owned(),
                stderr_path: None,
            }),
        });
    }
}

impl RefUnwindSafe for TestCases {}

#[doc(hidden)]
impl Drop for TestCases {
    fn drop(&mut self) {
        if !thread::panicking() {
            self.runner.borrow_mut().run();
        }
    }
}
