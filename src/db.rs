// Copyright {{year}} {{copyright_name}}. Licensed under {{license}}.

//! Public handle for the `{{project-name}}` embedded {{db_kind}} database.
//!
//! Every method on [`{{crate_name_pascal}}`] is currently a stub. The
//! shape of the public API is intentionally minimal — `open` /
//! `open_in_memory` / `flush` / `close` cover the lifecycle most
//! embedded DB consumers expect, without committing to a query surface
//! that varies by DB kind. Implement real bodies as the storage engine
//! comes online; widen the surface (`insert`, `get`, `range`, etc.)
//! once the underlying data model is settled.

use std::path::Path;

use crate::error::{Error, Result};

/// Top-level handle for an open `{{project-name}}` database.
///
/// Construct one with [`{{crate_name_pascal}}::open`] for a file-backed
/// store or [`{{crate_name_pascal}}::open_in_memory`] for an ephemeral
/// instance.
#[derive(Debug)]
pub struct {{crate_name_pascal}} {
    _private: (),
}

impl {{crate_name_pascal}} {
    /// Open the database at the given path.
    ///
    /// # Errors
    ///
    /// Currently always returns [`Error::NotImplemented`]; replace with
    /// real open-path logic once the storage engine is wired up.
    pub fn open<P: AsRef<Path>>(_path: P) -> Result<Self> {
        Err(Error::NotImplemented)
    }

    /// Open an ephemeral, in-memory instance.
    ///
    /// In-memory instances do not touch disk and are dropped when the
    /// handle is dropped.
    pub fn open_in_memory() -> Self {
        Self { _private: () }
    }

    /// Flush all pending writes to durable storage.
    ///
    /// # Errors
    ///
    /// Currently always returns [`Error::NotImplemented`]; replace with
    /// the real durability path once the storage engine is wired up.
    pub fn flush(&self) -> Result<()> {
        Err(Error::NotImplemented)
    }

    /// Close the database handle, releasing any held resources.
    ///
    /// # Errors
    ///
    /// Currently always succeeds; widen as needed when close has real
    /// work to do (sync, advisory-lock release, etc.).
    pub fn close(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_in_memory_returns_handle() {
        let _db = {{crate_name_pascal}}::open_in_memory();
    }

    #[test]
    fn open_returns_not_implemented() {
        let result = {{crate_name_pascal}}::open("/tmp/{{crate_name}}-test");
        assert!(matches!(result, Err(Error::NotImplemented)));
    }

    #[test]
    fn flush_returns_not_implemented() {
        let db = {{crate_name_pascal}}::open_in_memory();
        let result = db.flush();
        assert!(matches!(result, Err(Error::NotImplemented)));
    }

    #[test]
    fn close_succeeds() {
        let db = {{crate_name_pascal}}::open_in_memory();
        assert!(db.close().is_ok());
    }
}
