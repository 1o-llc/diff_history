// Copyright © 2026 1o. All rights reserved.

//! `diff_history`: the difference between two readings of the same question.
//!
//! A reading answers one question at one time: its rows (each a kind, a
//! label, a file and the line's text) and its counts. Two readings of one
//! spec are compared row by row ([`compare`]): a row is the same row when its
//! kind, label, file and line text are the same (a site that only moved
//! lines is the same site); what is in one and not the other was added or
//! removed, and every count is shown before and after. [`history`] compares
//! a spec's readings one to the next. [`load`] reads a reading's record.

pub mod diff;
pub mod refusal;

pub use diff::{compare, history, load, render, CountChange, CountValue, Reading, ReadingDiff, RowKey};
pub use refusal::{Error, Refusal, Result};
