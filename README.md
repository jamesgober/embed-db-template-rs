<h1 align="center">
    <img width="99" alt="Rust logo" src="https://raw.githubusercontent.com/jamesgober/rust-collection/72baabd71f00e14aa9184efcb16fa3deddda3a0a/assets/rust-logo.svg">
    <br>
    <strong>{{project-name}}</strong>
    <br>
    <sup><sub>EMBEDDED {{db_kind | upcase}} DATABASE FOR RUST</sub></sup>
</h1>

<p align="center">
    <a href="https://crates.io/crates/{{project-name}}"><img alt="crates.io" src="https://img.shields.io/crates/v/{{project-name}}.svg"></a>
    <a href="https://crates.io/crates/{{project-name}}" alt="Download {{project-name}}"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/{{project-name}}?color=%230099ff"></a>
    <a href="https://docs.rs/{{project-name}}"><img alt="docs.rs" src="https://docs.rs/{{project-name}}/badge.svg"></a>
    <img alt="MSRV" src="https://img.shields.io/badge/MSRV-{{min_rust_version}}%2B-blue.svg?style=flat-square" title="Rust Version">
    <a href="https://github.com/{{author_handle}}/{{repo_name}}/actions"><img alt="CI" src="https://github.com/{{author_handle}}/{{repo_name}}/actions/workflows/ci.yml/badge.svg"></a>
</p>

<p align="center">
    {{crate_description}}
</p>

---

## Status

`{{project-name}}` is in early scaffolding. The public API is unstable and
every method on the top-level handle is a stub that returns
`Error::NotImplemented`. The crate compiles, lints clean, and ships
tests — but it does not yet do useful work. See `STUBS.md` (deleted on
first commit) and the CHANGELOG for the actual roadmap.

## Install

```toml
[dependencies]
{{project-name}} = "{{crate_version}}"
```

## Quick start

```rust,no_run
use {{crate_name}}::{{crate_name_pascal}};

fn main() -> Result<(), {{crate_name}}::Error> {
    let db = {{crate_name_pascal}}::open_in_memory();
    db.close()?;
    Ok(())
}
```

## Features

Cargo features will be added as subsystems land. See `Cargo.toml` for
the current set.

## Documentation

- [docs.rs/{{project-name}}](https://docs.rs/{{project-name}})
- Source: <https://github.com/{{author_handle}}/{{repo_name}}>

<br><br>

<!-- LICENSE
############################################ -->
<div id="license">
    <h2>LICENSE</h2>
    <p>Licensed under either of</p>
    <b>Apache License, Version 2.0</b>: <a href="./LICENSE-APACHE">LICENSE-APACHE</a>
      &mdash; <a href="http://www.apache.org/licenses/LICENSE-2.0" target="_blank">http://www.apache.org/licenses/LICENSE-2.0</a>
    <br><br>
    <b>MIT License</b>: <a href="./LICENSE-MIT">LICENSE-MIT</a> &mdash;
    <a href="http://opensource.org/licenses/MIT" target="_blank">http://opensource.org/licenses/MIT</a>
    <br><br>
    <p>At your option. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.</p>
</div>


<!-- COPYRIGHT
------------------------------>
<div align="center">
    <h2></h2>
    Copyright &copy; {{year}} {{copyright_name}}.
</div>
