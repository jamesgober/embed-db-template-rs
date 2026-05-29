# STUBS — post-generation checklist

This file is generated alongside your new crate from `embed-db-template`.
Walk through every section below, tick the boxes, then **delete this
file** before your first commit.

The crate compiles, lints clean, and passes tests as generated — every
public method on `{{crate_name_pascal}}` is a stub that returns
`Error::NotImplemented`. That state is a starting point, not a finish
line.

---

## 1. Decisions

Lock these in before you write a single byte of engine code. Once made,
they propagate to dependents (`Cargo.toml` metadata, CI, REPS interpretation).

- [ ] **License chosen.** This template ships `LICENSE-APACHE` +
      `LICENSE-MIT` for the dual-license default (Apache-2.0 OR MIT).
      If you picked something else at `cargo generate` time:
  - [ ] `Cargo.toml` `license = ...` matches the actual SPDX or
        `LicenseRef-...` string.
  - [ ] Delete the LICENSE files you are not shipping (e.g. remove
        `LICENSE-MIT` if you went Apache-only; replace both with a
        `LICENSE` file for proprietary).
  - [ ] If proprietary, set `publish = false` in `Cargo.toml`.
  - [ ] README license footer matches reality.
- [ ] **MSRV locked.** `Cargo.toml`, `clippy.toml`, and `.github/workflows/ci.yml`
      all reference `{{min_rust_version}}`. If you want a different floor,
      change all three together — they are not derived.
- [ ] **Initial feature set.** The default `[features]` block is empty.
      Decide which capabilities will be feature-flagged (e.g. `async`,
      `encrypt`) before writing code so the gate goes around the right
      module boundary. Add matching matrix rows to `.github/workflows/ci.yml`.
- [ ] **Publish to crates.io? y/n.** If yes:
  - [ ] Reserve the name on crates.io before any code lands (squatters
        are a real problem).
  - [ ] Update `keywords` and `categories` in `Cargo.toml` — the
        defaults are placeholder. crates.io accepts up to 5 of each.
  - [ ] Confirm `documentation`, `repository`, `homepage` URLs are live.

---

## 2. Code stubs

The DB-specific module structure that emdb (KV) grew into looks like:
`storage/`, `index/`, `transaction.rs`, `namespace.rs`, `builder.rs`,
`async_impl.rs`. None of that ships in the template — each embedded DB
has its own data model and module shape. Build what your DB needs.

- [ ] **Storage backend chosen.** The org default substrate for
      embedded DBs is **`storage-io`** (the fsys rebrand) — it provides
      the lock-free journal primitive, NVMe passthrough flush,
      io_uring on Linux, atomic-replace `write_copy`. If you are
      building a {{db_kind}} DB and have no specific reason to roll
      your own journal, default to `storage-io`. Document the choice
      in `.dev/PROMPT.md` "Layers" section.
- [ ] **Builder pattern decision.** emdb ships `EmdbBuilder` because it
      has many configuration knobs (path, TTL defaults, flush policy,
      range-scan toggle, encryption keys). If your DB has more than
      ~2 open-time knobs, add a `src/builder.rs` modelled after
      emdb's. If not, keep `{{crate_name_pascal}}::open(path)` as the
      sole constructor.
- [ ] **Config struct decision.** Same logic — emdb does not have a
      standalone `Config` struct; configuration flows through the
      builder. Do not invent a `Config` type unless your DB really
      needs one separable from the constructor surface.
- [ ] **Public API stubs replaced.** `{{crate_name_pascal}}::open`,
      `open_in_memory`, `flush`, `close` all return
      `Error::NotImplemented` (except `open_in_memory` and `close`
      which return `Self` and `Ok(())`). Wire each to the real engine
      and remove the `NotImplemented` variant from `error.rs` once no
      call site can produce it.
- [ ] **Query surface added.** The template intentionally does not
      include `insert` / `get` / `range` / equivalents because they
      vary by DB type. Add the {{db_kind}}-appropriate verbs:
      vector → `upsert(id, vec)` / `search(query, k)`; graph →
      `add_node` / `add_edge` / `neighbors`; document → `insert(doc)`
      / `find(query)`.
- [ ] **Namespace / collection concept.** emdb has `Namespace` for
      logical partitioning. If your {{db_kind}} DB wants the
      equivalent (vec-db collections, graph-db named subgraphs,
      doc-db collections), copy the *shape* of emdb's
      `namespace.rs`, not the KV-specific implementation. Candidate
      for portfolio extraction once two DBs ship it.
- [ ] **Path-resolution helper.** emdb has `data_dir.rs` for
      XDG/AppData resolution. If you want the same UX
      (`db.app_name("x").database_name("y").build()`), copy the
      shape. Candidate for a shared `data-dir-lib` crate later.
- [ ] **Single-process lockfile.** emdb has `lockfile.rs` for
      "only one process opens this DB at a time". Universal for
      embedded DBs. Copy the shape if you want it; flag for shared
      extraction later.
- [ ] **Tests written.** `tests/smoke.rs` and the in-source
      `#[cfg(test)] mod tests` blocks in `db.rs` / `error.rs` cover
      the stub surface. Add integration tests in `tests/` per the
      REPS-mandated happy / error / edge-case shape.
- [ ] **Benchmarks written.** `benches/scaffold.rs` is a placeholder.
      Rename it to match your engine's primary workload, replace the
      noop body, update `[[bench]] name` in `Cargo.toml`.
- [ ] **Examples written.** `examples/basic.rs` shows the stub
      lifecycle. Add a real-feature example per major capability as it
      lands; declare each one in `[[example]]` in `Cargo.toml`.

---

## 3. External setup

- [ ] **GitHub repository created** at
      `https://github.com/{{author_handle}}/{{repo_name}}`. Push the
      initial commit so the CI badge in `README.md` resolves.
- [ ] **Branch protection on `main`** — require CI green, require PR
      review, no force push.
- [ ] **crates.io account & ownership** confirmed for the publishing
      identity, if publishing.
- [ ] **docs.rs build verified** after first publish — the
      `[package.metadata.docs.rs]` block in `Cargo.toml` already sets
      `all-features = true` and the `docsrs` cfg.
- [ ] **Secrets configured** in GitHub Actions if any (none needed for
      the default CI workflow).

---

## 4. Project coordination

These tie the new crate back to the rest of the Hive ecosystem.

- [ ] **Workspace registration.** If this crate joins a multi-crate
      workspace, add a `members = [...]` entry in the workspace
      `Cargo.toml`.
- [ ] **Library inventory updated.** Add this crate to the
      org-internal library inventory / portfolio doc so future
      consumers can discover it.
- [ ] **REPS sync.** `REPS.md` is the canonical compliance doc and
      is byte-identical across crates. If you have a REPS-sync
      script in the wider org, register this crate with it so future
      REPS updates land here automatically.
- [ ] **Error registry.** Reserve the `{{error_prefix}}-` prefix
      range in the wider Hive error registry, if one exists.
- [ ] **CHANGELOG hooked up.** The `[Unreleased]` link in
      `CHANGELOG.md` points at the repo's commit log. Update the link
      on first tagged release.

---

## 5. Verification

Run all of these locally before pushing the first commit. CI will run
the same checks remotely.

- [ ] `cargo build` clean (no warnings)
- [ ] `cargo test` clean (unit + integration + doc tests pass)
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo doc --no-deps` produces no warnings
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo deny check` passes (or every finding has a justified
      `ignore` entry in `deny.toml`)
- [ ] `cargo audit` passes (or every finding has a justified
      `--ignore` flag in the CI step)
- [ ] CI green on the first push
- [ ] README renders correctly on GitHub
      (`https://github.com/{{author_handle}}/{{repo_name}}`)

---

## 6. Cleanup

- [ ] **Delete this file (`STUBS.md`)** — it has no role in the
      finished crate.
- [ ] Delete `pre-generate.rhai` if it somehow ended up in your
      working tree (the template's `ignore` block should prevent
      this).
- [ ] Confirm `cargo-generate.toml` is not in your working tree
      (same).
