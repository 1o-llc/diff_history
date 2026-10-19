// Copyright © 2026 1o. All rights reserved.

//! The reading diff: two readings of one spec compared row by row. A row is
//! the same row when its kind, label, file and line text are the same (a site
//! that only moved lines is the same site); what is in one and not the other
//! was added or removed, and every count is shown before and after.

use crate::refusal::{Refusal, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// A row as a diff keys it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RowKey {
    pub kind: String,
    pub label: String,
    pub path: Option<String>,
    pub context: String,
    /// The line it is on in the reading it comes from.
    pub line: Option<u32>,
}

/// One count of a reading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountValue {
    pub what: String,
    pub n: usize,
}

/// A reading as a diff reads it: its name, its stamp, its counts, its rows
/// (the results, then the unresolved).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reading {
    pub name: String,
    pub stamp: String,
    pub counts: Vec<CountValue>,
    pub rows: Vec<RowKey>,
}

fn same(k: &RowKey) -> (String, String, Option<String>, String) {
    (k.kind.clone(), k.label.clone(), k.path.clone(), k.context.clone())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountChange {
    pub what: String,
    pub before: Option<usize>,
    pub after: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadingDiff {
    pub name: String,
    pub before: String,
    pub after: String,
    pub counts: Vec<CountChange>,
    pub added: Vec<RowKey>,
    pub removed: Vec<RowKey>,
}

impl ReadingDiff {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.counts.iter().all(|c| c.before == c.after)
    }
}

/// Two readings compared.
pub fn compare(before: &Reading, after: &Reading) -> ReadingDiff {
    let (b, a) = (&before.rows, &after.rows);
    let bs: BTreeSet<_> = b.iter().map(same).collect();
    let as_: BTreeSet<_> = a.iter().map(same).collect();
    let added = a.iter().filter(|k| !bs.contains(&same(k))).cloned().collect();
    let removed = b.iter().filter(|k| !as_.contains(&same(k))).cloned().collect();
    let mut whats: Vec<String> = before.counts.iter().map(|c| c.what.clone()).collect();
    for c in &after.counts {
        if !whats.contains(&c.what) {
            whats.push(c.what.clone());
        }
    }
    let counts = whats
        .into_iter()
        .map(|w| CountChange {
            before: before.counts.iter().find(|c| c.what == w).map(|c| c.n),
            after: after.counts.iter().find(|c| c.what == w).map(|c| c.n),
            what: w,
        })
        .collect();
    ReadingDiff { name: after.name.clone(), before: before.stamp.clone(), after: after.stamp.clone(), counts, added, removed }
}

/// A spec's readings, oldest first, each compared with the one before it.
pub fn history(readings: &[Reading]) -> Vec<ReadingDiff> {
    readings.windows(2).map(|w| compare(&w[0], &w[1])).collect()
}

fn n(v: Option<usize>) -> String {
    v.map(|n| n.to_string()).unwrap_or_else(|| "-".into())
}

/// The diff as a page: counts that moved first, then rows added and removed,
/// by kind (a crate pair removed from a duplicates reading is a copy gone).
pub fn render(d: &ReadingDiff) -> String {
    let mut s = format!("# {} — what changed\n\n- before: {}\n- after: {}\n\n## counts\n\n", d.name, d.before, d.after);
    for c in &d.counts {
        let mark = if c.before == c.after { "" } else { " (changed)" };
        s.push_str(&format!("- {}: {} → {}{mark}\n", c.what, n(c.before), n(c.after)));
    }
    let list = |s: &mut String, title: &str, rows: &[RowKey]| {
        s.push_str(&format!("\n## {title} ({})\n\n", rows.len()));
        for r in rows {
            let at = match (&r.path, r.line) {
                (Some(p), Some(l)) => format!("`{p}:{l}` · "),
                (Some(p), None) => format!("`{p}` · "),
                _ => String::new(),
            };
            let gone = if title == "removed" && r.kind == "crate-pair" { " (copies gone)" } else { "" };
            s.push_str(&format!("- {at}{} **{}**{gone}\n", r.kind, r.label));
        }
    };
    list(&mut s, "added", &d.added);
    list(&mut s, "removed", &d.removed);
    if d.is_empty() {
        s.push_str("\nnothing changed.\n");
    }
    s
}

/// A row as a reading's record holds it (its other fields are the reading's own).
#[derive(Debug, Clone, Deserialize)]
struct RecordRow {
    kind: String,
    label: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    line: Option<u32>,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct RecordQuery {
    #[serde(default)]
    results: Vec<RecordRow>,
}

/// What a diff reads from a reading's record (`readings/<name>/<stamp>.json`).
#[derive(Debug, Clone, Deserialize)]
struct Record {
    name: String,
    #[serde(default)]
    stamp: String,
    #[serde(default)]
    counts: Vec<CountValue>,
    #[serde(default)]
    query: RecordQuery,
    #[serde(default)]
    unresolved: Vec<RecordRow>,
}

impl RecordRow {
    fn key(self) -> RowKey {
        RowKey { context: self.attributes.get("context").cloned().unwrap_or_default(), kind: self.kind, label: self.label, path: self.path, line: self.line }
    }
}

/// A reading's record, as a diff reads it.
pub fn load(record: &Path) -> Result<Reading> {
    let text = std::fs::read_to_string(record).map_err(|e| Refusal::Unreadable.refuse(format!("{}: {e}", record.display())))?;
    let r: Record = serde_json::from_str(&text).map_err(|e| Refusal::Unreadable.refuse(format!("{}: {e}", record.display())))?;
    Ok(Reading { name: r.name, stamp: r.stamp, counts: r.counts, rows: r.query.results.into_iter().chain(r.unresolved).map(RecordRow::key).collect() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(kind: &str, label: &str, path: &str, line: u32, context: &str) -> RowKey {
        RowKey { kind: kind.into(), label: label.into(), path: Some(path.into()), context: context.into(), line: Some(line) }
    }

    fn reading(stamp: &str, rows: Vec<RowKey>, n: usize) -> Reading {
        Reading { name: "use_sites".into(), stamp: stamp.into(), counts: vec![CountValue { what: "use sites".into(), n }], rows }
    }

    #[test]
    fn a_site_that_only_moved_lines_is_the_same_site() {
        let a = reading("1", vec![row("use-site", "serve_metrics", "a.rs", 3, "serve_metrics();"), row("use-site", "serve_metrics", "b.rs", 9, "x()")], 2);
        let b = reading("2", vec![row("use-site", "serve_metrics", "a.rs", 7, "serve_metrics();"), row("use-site", "serve_metrics", "c.rs", 1, "y()")], 2);
        let d = compare(&a, &b);
        assert_eq!(d.added.iter().map(|r| r.path.clone().unwrap()).collect::<Vec<_>>(), ["c.rs"]);
        assert_eq!(d.removed.iter().map(|r| r.path.clone().unwrap()).collect::<Vec<_>>(), ["b.rs"]);
        assert!(!d.is_empty());
        assert!(compare(&a, &a).is_empty());
        let page = render(&d);
        assert!(page.contains("- use sites: 2 → 2\n"), "{page}");
        assert!(page.contains("## added (1)\n\n- `c.rs:1` · use-site **serve_metrics**"), "{page}");
        assert_eq!(history(&[a.clone(), b, a]).len(), 2);
    }

    #[test]
    fn a_reading_s_record_reads_back_its_rows_and_counts() {
        let dir = std::env::temp_dir().join(format!("diff-history-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("r.json");
        std::fs::write(&p, r#"{"schema":"custodian.reading.v1","name":"env","stamp":"20261019T120000Z","counts":[{"what":"keys","n":2}],"query":{"results":[{"id":"1","kind":"env-key","label":"NODEPLAN_X","path":"a.rs","line":4,"attributes":{"context":"env(\"NODEPLAN_X\")"}}]},"unresolved":[{"id":"2","kind":"unresolved","label":"y"}]}"#).unwrap();
        let r = load(&p).unwrap();
        assert_eq!((r.name.as_str(), r.rows.len(), r.counts[0].n), ("env", 2, 2));
        assert_eq!(r.rows[0].context, "env(\"NODEPLAN_X\")");
        assert_eq!(load(&dir.join("none.json")).unwrap_err().refusal, Refusal::Unreadable);
    }
}
