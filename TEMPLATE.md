# embed-db-template

A [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
template for spinning up new embedded-database crates in the Hive
ecosystem.

> **This is the template repo's own README.** It is not shipped to
> generated crates — `cargo-generate.toml` lists it in the `ignore`
> block. The README that lands in the generated crate is
> `README.md` (tokenized).

---

## What this is for

Every embedded DB in this org — `vec-db`, `graph-db`, `doc-db`,
`kv-db`, and any future ones — should share the same shape:

- Apache-2.0 OR MIT dual license
- REPS-compliant lint stack (`#![deny(...)]` block in `lib.rs`)
- `Error` / `Result` types with `EM`-style error-code prefix
- `[package].rust-version` MSRV pinned
- `[[bench]]` and `[[example]]` declarations
- `cargo-deny` + `cargo-audit` in CI
- Three-platform CI matrix
- `.dev/PROMPT.md` + `.dev/DIRECTIVES.md` AI development guard rails

This template is the structural shape, with all KV-specific semantics
stripped out. The reference implementation is
[`emdb`](https://github.com/jamesgober/emdb-rs); this template mines its
*patterns*, not its data model.

---

## Generating a new crate

```sh
cargo generate --path /path/to/embed-db-template
# or, once published:
cargo generate --git https://github.com/jamesgober/embed-db-template
```

You will be prompted for the values below. After generation, walk
through `STUBS.md` and delete it.

## Token reference

cargo-generate uses lowercase `snake_case` placeholder names. In
template files they appear as `{{placeholder_name}}` (Liquid syntax,
case-sensitive).

| Token | Prompted? | Example | Notes |
|---|---|---|---|
| `crate_name` | yes | `vec-db` | kebab-case, validated by regex |
| `crate_name_snake` | derived | `vec_db` | from `crate_name` via Rhai pre-script |
| `crate_name_pascal` | derived | `VecDb` | from `crate_name` via Rhai pre-script |
| `crate_description` | yes | `"Embedded vector database with HNSW indexing"` | shown on crates.io |
| `crate_version` | yes | `0.1.0` | default `0.1.0` |
| `db_kind` | yes | `vector` | choice: vector / graph / document / key-value / object |
| `error_prefix` | yes | `VD` | short uppercase, used as `VD-XXXXX` in Hive error registry |
| `author_name` | yes | `James Gober` | default `James Gober` |
| `author_email` | yes | `me@jamesgober.com` | default `me@jamesgober.com` |
| `author_handle` | yes | `jamesgober` | GitHub `OWNER` in `github.com/OWNER/repo` |
| `copyright_name` | yes | `James Gober` | usually same as `author_name` |
| `repo_name` | yes | `vec-db` | GitHub repo name (often == `crate_name`) |
| `start_date` | yes | `2026-05-29` | YYYY-MM-DD; first 4 chars become `{{year}}` |
| `year` | derived | `2026` | from `start_date` via Rhai pre-script |
| `license` | yes | `Apache-2.0 OR MIT` | choice: dual / Apache-only / MIT / Proprietary |
| `min_rust_version` | yes | `1.75` | MSRV |

`keywords` and `categories` ship with sensible defaults in `Cargo.toml`
(`["database", "embedded", "{{db_kind}}"]` and
`["database-implementations"]`) — customize per-crate before publishing.

---

## Post-generation

The generated crate ships with `STUBS.md` at its root — a checklist of
every decision and stub the generator can't make for you (license file
cleanup, query-surface design, GitHub setup, workspace registration,
CI verification). Walk through it, tick the boxes, delete the file.

Generated crates compile, lint clean, and pass tests out of the box.
Every method on the top-level `{{crate_name_pascal}}` handle is a stub
that returns `Error::NotImplemented` — honest about its state, not
pretending to work.

---

## What's tokenized vs verbatim

| File | Treatment |
|---|---|
| `Cargo.toml`, `clippy.toml`, `README.md`, `CHANGELOG.md`, `STUBS.md` | tokenized |
| `src/lib.rs`, `src/error.rs`, `src/db.rs` | tokenized |
| `tests/smoke.rs`, `benches/scaffold.rs`, `examples/basic.rs` | tokenized |
| `.dev/PROMPT.md`, `.dev/DIRECTIVES.md` | tokenized |
| `.github/workflows/ci.yml` | tokenized (MSRV only) |
| `deny.toml` | verbatim |
| `LICENSE-APACHE`, `LICENSE-MIT` | verbatim (legal text must not change) |
| `REPS.md` | verbatim (canonical compliance doc, sync target) |
| `.gitignore`, `.editorconfig`, `rustfmt.toml` | verbatim |
| `cargo-generate.toml`, `pre-generate.rhai`, `TEMPLATE.md` | **excluded from generated crate** |

---

## What's intentionally NOT in the template

The template is a structural scaffold, not a starter kit. These
emdb-isms stay out because each DB-kind will reinvent them differently:

- `builder.rs` — emdb's builder is KV-shaped (TTL, encryption,
  range-scans). Add one per DB if you need >2 open-time knobs.
- `data_dir.rs` — XDG/AppData path resolution. Candidate for shared
  `data-dir-lib` extraction once two consumers ship it.
- `lockfile.rs` — single-process-ownership advisory lock. Universal
  but heavy; candidate for shared extraction.
- `namespace.rs` — logical partitioning. The *concept* is structural
  (collections / subgraphs / named partitions), the emdb
  *implementation* is KV-specific.
- Anything in `src/storage/` — Bitcask-engine-specific.
- All `tests/`, `benches/`, `examples/` content — per-DB-type.
- `docs/` — per-DB-type prose; the generated crate writes its own.

See the `STUBS.md` "Code stubs" section for the per-decision flagging.

---

## Versioning policy for this template

The template uses semantic versioning, tagged at the repo root.

- **Patch** (`v1.0.0 → v1.0.1`): typo fixes, dependency-version bumps
  in `dev-dependencies`, comment clarifications. Existing
  cargo-generate prompts unchanged.
- **Minor** (`v1.0.x → v1.1.0`): new tokens added (with sensible
  defaults so existing generations stay valid), new optional files
  added, new lint deny-ed. Generations from older revisions stay
  source-compatible.
- **Major** (`v1.x → v2.0.0`): removed or renamed tokens, removed
  files, restructured layout. Consumers should pin
  `--rev <commit-or-tag>` when generating to control upgrades.

When generating, prefer pinning to a tag for reproducibility:

```sh
cargo generate --git https://github.com/jamesgober/embed-db-template --tag v1.0.0
```

---

## Contributing back

If you build a new embedded DB from this template and find yourself
copying the same shape into a second one, that's the signal to lift it
back into the template. Open a PR with:

- The pattern (file or block) you're adding.
- The two consumers that proved it generalizes.
- A note in `TEMPLATE.md`'s "What's intentionally NOT in the template"
  table moving the entry from "NOT included" to "included".
