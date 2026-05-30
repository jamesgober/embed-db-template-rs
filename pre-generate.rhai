// pre-generate.rhai
//
// Runs once before placeholder substitution. Derives the two values
// that can be computed from the user's other inputs, so the user does
// not have to type them:
//
//   - crate_name_pascal  (kebab "vec-db" -> "VecDb")
//   - year               ("2026-05-29"   -> "2026")
//
// Note: `crate_name_snake` is NOT derived here — cargo-generate's
// built-in `crate_name` placeholder is already the snake_case version
// of `project-name`. Use `{{crate_name}}` directly in templates where
// the Rust-identifier form is needed; use `{{project-name}}` for the
// kebab-case form (Cargo.toml `name`, crates.io URLs, prose).

// Pascal-case derivation needs the kebab form so it can split on `-`.
// `project-name` is the built-in kebab placeholder; `crate_name` is
// the snake derivation and would no-op a split-on-dash.
let kebab = variable::get("project-name");

let parts = kebab.split("-");
let pascal = "";
for part in parts {
    if part.len() > 0 {
        let head = part.sub_string(0, 1).to_upper();
        let tail = part.sub_string(1);
        pascal += head;
        pascal += tail;
    }
}
variable::set("crate_name_pascal", pascal);

// year: the user supplied `start_date` as YYYY-MM-DD; the first four
// characters are the year. Avoids dragging in a date-parsing helper.
let start = variable::get("start_date");
let year = start.sub_string(0, 4);
variable::set("year", year);
