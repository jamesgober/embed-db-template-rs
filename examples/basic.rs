//! Minimal example — open an in-memory instance and close it.
//!
//! Run with:
//! ```sh
//! cargo run --example basic
//! ```

use {{crate_name}}::{{crate_name_pascal}};

fn main() {
    let db = {{crate_name_pascal}}::open_in_memory();
    println!("{{project-name}} instance opened (stub)");
    match db.close() {
        Ok(()) => println!("closed cleanly"),
        Err(err) => eprintln!("failed to close: {err}"),
    }
}
