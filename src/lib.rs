// Copyright {{year}} {{copyright_name}}.
// Licensed under {{license}}.

//! # {{crate_name}}
//!
//! {{crate_description}}
//!
//! This crate is in early scaffolding. The public API is unstable; every
//! method on the top-level [`{{crate_name_pascal}}`] handle is a stub that
//! returns [`Error::NotImplemented`]. Generated crates compile and lint
//! cleanly out of the box, but you must implement the real engine before
//! they do useful work.
//!
//! See `STUBS.md` at the repo root for the post-generation checklist.

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(unused_results)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::unreachable)]
#![deny(clippy::undocumented_unsafe_blocks)]
// Test code is allowed to use the convenience panickers — the strict
// lint profile above is for production library code, not assertion
// scaffolding inside `#[cfg(test)] mod tests` blocks.
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::print_stdout,
        clippy::print_stderr
    )
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod db;
mod error;

pub use db::{{crate_name_pascal}};
pub use error::{Error, Result};
