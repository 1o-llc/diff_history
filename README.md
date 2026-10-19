# diff_history

The difference between two readings of the same question. A reading answers one question at one time: its rows
(each a kind, a label, a file and the line's text) and its counts. `compare` keys a row by its kind, label, file and
line text, so a site that only moved lines is the same site; what is in one reading and not the other was added
or removed, and every count is shown before and after. `history` compares a spec's readings one to the next;
`render` writes the page (counts that moved first, then the rows added and removed); `load` reads a reading's
record (`readings/<name>/<stamp>.json`).

```rust,no_run
# fn demo() -> diff_history::Result<()> {
let a = diff_history::load("readings/env/20261019T110000Z.json".as_ref())?;
let b = diff_history::load("readings/env/20261019T120000Z.json".as_ref())?;
print!("{}", diff_history::render(&diff_history::compare(&a, &b)));
# Ok(()) }
```

The comparison came out of custodian_kit's `diff`, which now reads a `ReadingReport` into a `Reading` and calls it.

## Refusals

`DIFH0001` a reading's record cannot be read.

## Releasing

Bump `version` in Cargo.toml, add a dated `## <version> (YYYY-MM-DD)` entry to CHANGELOG.md, and push to
`main`. CI (.github/workflows/ci.yml) runs the tests on stable and Rust 1.89 and clippy, then publishes that
version to crates.io through trusted publishing; a version already on crates.io publishes nothing.

## Ownership

Developed and published by James Hunt. The repository is held by the 1o organization
(https://github.com/1o-llc). Assignment of the copyright to 1o is pending documentation. The license texts
(LICENSE-MIT, LICENSE-APACHE) are left as they were found. Every file new in 0.1.0 carries the notice
Copyright © 2026 1o. All rights reserved.

## License

MIT OR Apache-2.0
