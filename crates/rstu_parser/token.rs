// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    HeadingUnderline,
    Indent,
    Directive,
    Comment,
    TableHorizontal,
}

static HEADING_UNDERLINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^=+\n$").expect("valid heading regex"));

static INDENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[ \t]+$").expect("valid indent regex"));

static DIRECTIVE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\.\.\s+([A-Za-z_-]+)::[ \t]*$").expect("valid directive regex"));

static COMMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\.\.\s+.*[^:\s](?:\s*)$").expect("valid comment regex"));

static TABLE_HORIZONTAL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^=+(?:\s+=+)+\s*$").expect("valid table horizontal regex"));

impl TokenKind {
    pub fn name(self) -> &'static str {
        match self {
            TokenKind::HeadingUnderline => "heading_underline",
            TokenKind::Indent => "indent",
            TokenKind::Directive => "directive",
            TokenKind::Comment => "comment",
            TokenKind::TableHorizontal => "table_horizontal",
        }
    }

    pub fn regex(self) -> &'static Regex {
        match self {
            TokenKind::HeadingUnderline => &HEADING_UNDERLINE_RE,
            TokenKind::Indent => &INDENT_RE,
            TokenKind::Directive => &DIRECTIVE_RE,
            TokenKind::Comment => &COMMENT_RE,
            TokenKind::TableHorizontal => &TABLE_HORIZONTAL_RE,
        }
    }

    pub fn is_match(self, input: &str) -> bool {
        self.regex().is_match(input)
    }
}

#[cfg(test)]
mod tests {
    use super::TokenKind;

    #[test]
    fn heading_underline_matches() {
        assert!(TokenKind::HeadingUnderline.is_match("====\n"));
    }

    #[test]
    fn heading_underline_non_matching() {
        assert!(!TokenKind::HeadingUnderline.is_match("==a=\n"));
    }

    #[test]
    fn indent_matches() {
        assert!(TokenKind::Indent.is_match(" \t  "));
    }

    #[test]
    fn indent_non_matching() {
        assert!(!TokenKind::Indent.is_match("abc"));
    }

    #[test]
    fn directive_matches() {
        assert!(TokenKind::Directive.is_match(".. note::"));
    }

    #[test]
    fn directive_non_matching() {
        assert!(!TokenKind::Directive.is_match(".. note:"));
    }

    #[test]
    fn comment_matches() {
        assert!(TokenKind::Comment.is_match(".. this is a comment"));
    }

    #[test]
    fn comment_non_matching() {
        assert!(!TokenKind::Comment.is_match(".. warning::"));
    }

    #[test]
    fn table_horizontal_matches() {
        assert!(TokenKind::TableHorizontal.is_match("==== ====="));
    }

    #[test]
    fn table_horizontal_non_matching() {
        assert!(!TokenKind::TableHorizontal.is_match("========"));
    }
}