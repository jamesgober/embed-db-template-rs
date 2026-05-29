// pre-generate.rhai
//
// Runs once before placeholder substitution. Derives the three values
// that can be computed from the user's other inputs, so the user does
// not have to type them:
//
//   - crate_name_snake   ("vec-db"   -> "vec_db")
//   - crate_name_pascal  ("vec-db"   -> "VecDb")
//   - year               ("2026-05-29" -> "2026")
//
// Set on the cargo-generate variable table so every template file can
// reference them as `{{crate_name_snake}}` / `{{crate_name_pascal}}` /
// `{{year}}` without an extra prompt.

let name = variable::get("crate_name");

// snake: kebab -> underscore. crate_name is validated kebab-case in
// cargo-generate.toml, so the only character that needs swapping is
// the hyphen.
let snake = name.replace("-", "_");
variable::set("crate_name_snake", snake);

// pascal: split on `-`, upper-case the first byte of each segment,
// concatenate. Rhai strings are UTF-8; for our kebab-case crate
// names (lower ASCII only) `to_upper()` is a one-byte operation
// and the slice math is unambiguous.
let parts = name.split("-");
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
