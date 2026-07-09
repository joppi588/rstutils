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
    BlankLine,
    LiteralString,
}

static HEADING_UNDERLINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(=+)$").unwrap());

static INDENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([ \t]+)").unwrap());

static DIRECTIVE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\.\.\s+([A-Za-z_-]+)::.*)$").unwrap());

static COMMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\.\.\s(.*)$").unwrap());

static TABLE_HORIZONTAL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^=+(?:\s+=+)+\s*$").unwrap());

static BLANK_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[ \t]*$").unwrap());

static LITERAL_STRING_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([^\s`\n][^`\n]*)").unwrap());

impl TokenKind {
    pub fn name(self) -> &'static str {
        match self {
            TokenKind::HeadingUnderline => "heading_underline",
            TokenKind::Indent => "indent",
            TokenKind::Directive => "directive",
            TokenKind::Comment => "comment",
            TokenKind::TableHorizontal => "table_horizontal",
            TokenKind::BlankLine => "blank_line",
            TokenKind::LiteralString => "literal_string",
        }
    }

    pub fn regex(self) -> &'static Regex {
        match self {
            TokenKind::HeadingUnderline => &HEADING_UNDERLINE_RE,
            TokenKind::Indent => &INDENT_RE,
            TokenKind::Directive => &DIRECTIVE_RE,
            TokenKind::Comment => &COMMENT_RE,
            TokenKind::TableHorizontal => &TABLE_HORIZONTAL_RE,
            TokenKind::BlankLine => &BLANK_LINE_RE,
            TokenKind::LiteralString => &LITERAL_STRING_RE,
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

    #[test]
    fn blank_line_matches_empty() {
        assert!(TokenKind::BlankLine.is_match(""));
    }

    #[test]
    fn blank_line_matches_whitespace_only() {
        assert!(TokenKind::BlankLine.is_match(" \t"));
    }

    #[test]
    fn blank_line_non_matching_text() {
        assert!(!TokenKind::BlankLine.is_match("text"));
    }

    #[test]
    fn literal_string_matches() {
        assert!(TokenKind::LiteralString.is_match("Hello world"));
    }

    #[test]
    fn literal_string_matches_until_backtick() {
        assert!(TokenKind::LiteralString.is_match("Hello`world"));
    }

    #[test]
    fn literal_string_non_matching_whitespace_prefix() {
        assert!(!TokenKind::LiteralString.is_match(" hello"));
    }

    #[test]
    fn literal_string_non_matching_backtick_prefix() {
        assert!(!TokenKind::LiteralString.is_match("`hello"));
    }
}

