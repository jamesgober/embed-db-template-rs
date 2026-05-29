//! Integration-test scaffold.
//!
//! Exercises the public API surface of the crate. Add real
//! integration tests here as functionality lands.

use {{crate_name_snake}}::{{crate_name_pascal}};

#[test]
fn open_in_memory_and_close() {
    let db = {{crate_name_pascal}}::open_in_memory();
    db.close().expect("close on in-memory handle should succeed");
}
