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

    pub fn as_tuple(&self) -> (TokenKind, &str) {
        (self.kind, &self.lexeme)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Transition,
    SectionTitlePrefix,
    SectionTitleSuffix,
    Indent,
    Spaces,
    DoubleDot,
    DoubleColon,
    TableHorizontal,
    BlankLine,
    NewLine,
    Word,
    Bold,
    LiteralString,
}

static INDENT_RE: LazyLock<Regex> = token_regex!(r"(?:^|\n)([ \t]+)(?:[^ \t\n])");

static SPACES_RE: LazyLock<Regex> = token_regex!(r"(?:[^ \t\n])([ \t]+)([^ \t]|$)");

static DOUBLE_DOT_RE: LazyLock<Regex> = token_regex!(r"(?:^|\n|\s)(\.\.)(?:\n|$|\s)");

static DOUBLE_COLON_RE: LazyLock<Regex> = token_regex!(r"(?:.|\n)(::)(.|\n)");

static TABLE_HORIZONTAL_RE: LazyLock<Regex> = token_regex!(r"(?:^|\n)(=+(?:\s+=+)+\s*)(?:\n|$)");

static BLANK_LINE_RE: LazyLock<Regex> = token_regex!(r"(?:\n)([ \t]*\n)(?:.|\n)");

static NEW_LINE_RE: LazyLock<Regex> = token_regex!(r"(?:[^\n])(\n)(?:.|\n)");

static WORD_RE: LazyLock<Regex> =
    token_regex!(r"(?:^|[^A-Za-z0-9_])([A-Za-z0-9_]+)(?:$|[^A-Za-z0-9_])");

static BOLD_RE: LazyLock<Regex> = token_regex!(r"(?:.|\n)(\*\*)(?:.|\n)");

static LITERAL_STRING_RE: LazyLock<Regex> = token_regex!(r"(?:^|\n)(.*)(?:\n|$)");

static TRANSITION_RE: LazyLock<Regex> = token_regex!(r"(?:^|\n)\n([=~#]+)\n(?:\n|$)");

static SECTION_TITLE_PREFIX_RE: LazyLock<Regex> = token_regex!(r"(?:^|\n)\n([=~#]+)(?:\n|$)");

static SECTION_TITLE_SUFFIX_RE: LazyLock<Regex> = token_regex!(r"(?:^|\n)([=~#]+)(?:\n|$)");

impl TokenKind {
    pub fn regex(self) -> &'static Regex {
        // Token regexp have three parts: pre-context, token, post-context. Contexts are non-matching groups.
        // IMPORTANT: The order of the enum matters, as the first matching regexp will be picked.

        match self {
            TokenKind::Transition => &TRANSITION_RE,
            TokenKind::SectionTitlePrefix => &SECTION_TITLE_PREFIX_RE,
            TokenKind::SectionTitleSuffix => &SECTION_TITLE_SUFFIX_RE,

            TokenKind::Indent => &INDENT_RE,
            TokenKind::Spaces => &SPACES_RE,
            TokenKind::DoubleDot => &DOUBLE_DOT_RE,
            TokenKind::DoubleColon => &DOUBLE_COLON_RE,
            TokenKind::TableHorizontal => &TABLE_HORIZONTAL_RE,
            TokenKind::BlankLine => &BLANK_LINE_RE,
            TokenKind::NewLine => &NEW_LINE_RE,
            TokenKind::Word => &WORD_RE,
            TokenKind::Bold => &BOLD_RE,
            TokenKind::LiteralString => &LITERAL_STRING_RE,
        }
    }

    // TODO: Delete
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
    fn transition_matches() {
        assert!(TokenKind::Transition.is_match("===="));
    }

    #[test]
    fn transition_non_matching() {
        assert!(!TokenKind::Transition.is_match("==a="));
    }

    #[test]
    fn indent_matches() {
        assert!(TokenKind::Indent.is_match("\n \t  W"));
    }

    #[test]
    fn indent_non_matching() {
        assert!(!TokenKind::Indent.is_match("abc"));
    }

    #[test]
    fn spaces_matches() {
        assert!(TokenKind::Spaces.is_match("x \t x"));
    }

    #[test]
    fn spaces_non_matching() {
        assert!(!TokenKind::Spaces.is_match("xabcx"));
    }

    #[test]
    fn bold_matches() {
        assert!(TokenKind::Bold.is_match("\n**\n"));
    }

    #[test]
    fn bold_non_matching() {
        assert!(!TokenKind::Bold.is_match("*"));
    }

    #[test]
    fn doublecolon_matches() {
        assert!(TokenKind::DoubleColon.is_match("\n.. note::\n"));
    }

    #[test]
    fn doublecolon_non_matching() {
        assert!(!TokenKind::DoubleColon.is_match(".. note:"));
    }

    #[test]
    fn doubledot_matches() {
        assert!(TokenKind::DoubleDot.is_match(".. this is a comment"));
    }

    #[test]
    fn doubledot_non_matching() {
        assert!(!TokenKind::DoubleDot.is_match("warning..."));
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
        assert!(TokenKind::BlankLine.is_match("\n\n\n"));
    }

    #[test]
    fn blank_line_matches_whitespace_only() {
        assert!(TokenKind::BlankLine.is_match("\n \t\n\n"));
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
    fn word_matches_alphanumeric_and_underscore() {
        assert!(TokenKind::Word.is_match("alpha_123"));
    }

    #[test]
    fn word_matches_with_newline_boundary() {
        assert!(TokenKind::Word.is_match("\nalpha_123\n"));
    }

    #[test]
    fn word_non_matching_without_word_chars() {
        assert!(!TokenKind::Word.is_match("---\n***"));
    }
}
