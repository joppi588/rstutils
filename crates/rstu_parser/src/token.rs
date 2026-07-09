// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;

macro_rules! token_regex {
    ($pattern:expr) => {
        LazyLock::new(|| Regex::new($pattern).unwrap())
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    HeadingUnderline,
    Indent,
    Directive,
    Comment,
    TableHorizontal,
    BlankLine,
    NewLine,
    LiteralString,
}

// Token regexp have three parts: pre-context, token, post-context. Contexts are non-matching groups.

static HEADING_UNDERLINE_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|\n)(=+)(?:\n|$)");

static INDENT_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|\n)([ \t]+)(?:\n|$)");

static DIRECTIVE_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|\n)(\.\.\s+[A-Za-z_-]+::.*)(?:\n|$)");

static COMMENT_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|\n)(\.\.\s(?:[^:\n]|:[^:\n])*)(?:\n|$)");

static TABLE_HORIZONTAL_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|\n)(=+(?:\s+=+)+\s*)(?:\n|$)");

static BLANK_LINE_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|\n)([ \t]*)(?:\n|$)");

static NEW_LINE_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|[^\n])(\n)(?:$|[^\n])");

static LITERAL_STRING_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|\n)([^\s`\n][^`\n]*)(?:\n|$)");

impl TokenKind {
    pub fn name(self) -> &'static str {
        match self {
            TokenKind::HeadingUnderline => "heading_underline",
            TokenKind::Indent => "indent",
            TokenKind::Directive => "directive",
            TokenKind::Comment => "comment",
            TokenKind::TableHorizontal => "table_horizontal",
            TokenKind::BlankLine => "blank_line",
            TokenKind::NewLine => "new_line",
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
            TokenKind::NewLine => &NEW_LINE_RE,
            TokenKind::LiteralString => &LITERAL_STRING_RE,
        }
    }

    pub fn inner_match<'a>(self, input: &'a str) -> Option<&'a str> {
        self.regex()
            .captures(input)
            .and_then(|captures| captures.get(1).map(|m| m.as_str()))
    }

    pub fn is_match(self, input: &str) -> bool {
        self.inner_match(input).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::TokenKind;

    #[test]
    fn heading_underline_matches() {
        assert!(TokenKind::HeadingUnderline.is_match("===="));
    }

    #[test]
    fn heading_underline_non_matching() {
        assert!(!TokenKind::HeadingUnderline.is_match("==a="));
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
    fn literal_string_non_matching_with_backtick_inside() {
        assert!(!TokenKind::LiteralString.is_match("Hello`world"));
    }

    #[test]
    fn literal_string_non_matching_whitespace_prefix() {
        assert!(!TokenKind::LiteralString.is_match(" hello"));
    }

    #[test]
    fn literal_string_non_matching_backtick_prefix() {
        assert!(!TokenKind::LiteralString.is_match("`hello"));
    }

    #[test]
    fn regex_captures_token_with_context_chars() {
        assert_eq!(TokenKind::Directive.inner_match(".. note::"), Some(".. note::"));
    }
}

