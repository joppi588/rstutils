// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
pub(crate) mod block;
pub(crate) mod block_quote;
pub(crate) mod comments;
pub(crate) mod definition_list;
pub(crate) mod directives;
pub(crate) mod doctest_block;
pub(crate) mod footnote_citation;
pub(crate) mod inline_markup;
pub(crate) mod line_block;
pub(crate) mod lists;
pub(crate) mod literal_block;
pub(crate) mod paragraph;
pub(crate) mod section;
pub(crate) mod targets;
pub(crate) mod transition;

use regex::{Captures, Regex};
use std::sync::LazyLock;

static REFERENCE_ESCAPE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\(?s:(.))|\\$").unwrap());

static URI_NORMALIZATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\(?P<escaped>(?s:.))|\\$|\s").unwrap());

fn normalize_reference_name(reference: &str) -> String {
    REFERENCE_ESCAPE
        .replace_all(reference, "$1")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_uri(uri: &str) -> String {
    URI_NORMALIZATION
        .replace_all(uri, |captures: &Captures<'_>| {
            match captures.name("escaped") {
                Some(escaped) if escaped.as_str().chars().next().unwrap().is_whitespace() => {
                    " ".to_owned()
                }
                Some(escaped) => escaped.as_str().to_owned(),
                None => String::new(),
            }
        })
        .into_owned()
}
