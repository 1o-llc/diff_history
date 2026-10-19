# Changelog

## 0.1.0 (2026-10-19)

The difference between two readings of the same question, as a crate of its
own. custodian_kit compared two of its own `ReadingReport`s, so only a
program linking custodian_kit, with klepto and tantivy behind it, could say
what moved between two readings. The comparison is moved here. It works on a
reading as a diff reads it, and a reading's record is read straight from its
JSON. custodian_kit's `diff` reads its report into that form and calls it.
Every link is pinned to 56db2e8.

- The comparison is `RowKey` ([src/diff.rs#L15](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L15)), `CountChange`,
  `ReadingDiff` ([src/diff.rs#L53](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L53)), `compare` ([src/diff.rs#L69](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L69)) and
  `render` ([src/diff.rs#L103](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L103)). It is moved from [custodian_kit src/diff.rs#L12](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/diff.rs#L12),
  [custodian_kit src/diff.rs#L30](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/diff.rs#L30), [custodian_kit src/diff.rs#L37](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/diff.rs#L37), [custodian_kit src/diff.rs#L56](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/diff.rs#L56) and
  [custodian_kit src/diff.rs#L85](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/src/diff.rs#L85). A row is still keyed by its kind, label, file and line
  text, so a site that only moved lines is the same site. The page is
  unchanged.
- `Reading` ([src/diff.rs#L34](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L34)) holds what a diff reads: the reading's
  name, its stamp, its counts and its rows.
- New: `load` ([src/diff.rs#L169](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L169)) reads a reading's record. It reads
  `readings/<name>/<stamp>.json` for those fields and nothing else.
- New: `history` ([src/diff.rs#L93](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L93)) compares a spec's readings one to the
  next.
- Proven by `a_site_that_only_moved_lines_is_the_same_site`
  ([src/diff.rs#L188](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L188)) and `a_reading_s_record_reads_back_its_rows_and_counts`
  ([src/diff.rs#L203](https://github.com/1o-llc/diff_history/blob/56db2e8deec1574db9cdaddbdf5d3087a29a9b95/src/diff.rs#L203)).
- custodian_kit's `the_diff_says_what_moved`
  ([custodian_kit tests/reading.rs#L229](https://github.com/1o-llc/custodian_kit/blob/b17adf76375f72bebfb5ae1d1d93de3826369aac/tests/reading.rs#L229))
  passes through it on two real readings of its fixture.

The refusal is the liaise code `DIFH0001`.

Proof: `cargo test`, 2 tests.
