// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;

macro_rules! token_regex {
    ($pattern:expr) => {{
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
        &RE
    }};
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

impl TokenKind {
    pub const ALL: [TokenKind; 13] = [
        // IMPORTANT: The order of the enum matters, as the first matching token will be picked.
        TokenKind::Transition,
        TokenKind::SectionTitlePrefix,
        TokenKind::SectionTitleSuffix,
        TokenKind::Indent,
        TokenKind::Spaces,
        TokenKind::DoubleDot,
        TokenKind::DoubleColon,
        TokenKind::TableHorizontal,
        TokenKind::BlankLine,
        TokenKind::NewLine,
        TokenKind::Word,
        TokenKind::Bold,
        TokenKind::LiteralString,
    ];

    pub fn regex(self) -> &'static Regex {
        // Token regexp have three parts: pre-context, token, post-context. Contexts are non-matching groups.

        match self {
            TokenKind::Transition => token_regex!(r"(?:^|\n)\n([=~#]+)\n(?:\n|$)"),
            TokenKind::SectionTitlePrefix => token_regex!(r"(?:^|\n)\n([=~#]+)(?:\n|$)"),
            TokenKind::SectionTitleSuffix => token_regex!(r"(?:^|\n)([=~#]+)(?:\n|$)"),

            TokenKind::Indent => token_regex!(r"(?:^|\n)([ \t]+)(?:[^ \t\n])"),
            TokenKind::Spaces => token_regex!(r"(?:[^ \t\n])([ \t]+)([^ \t]|$)"),
            TokenKind::DoubleDot => token_regex!(r"(?:^|\n|\s)(\.\.)(?:\n|$|\s)"),
            TokenKind::DoubleColon => token_regex!(r"(?:.|\n)(::)(.|\n)"),
            TokenKind::TableHorizontal => token_regex!(r"(?:^|\n)(=+(?:\s+=+)+\s*)(?:\n|$)"),
            TokenKind::BlankLine => token_regex!(r"(?:\n)([ \t]*\n)(?:.|\n)"),
            TokenKind::NewLine => token_regex!(r"(?:[^\n])(\n)(?:.|\n)"),
            TokenKind::Word => {
                token_regex!(r"(?:^|[^A-Za-z0-9_])([A-Za-z0-9_]+)(?:$|[^A-Za-z0-9_])")
            }
            TokenKind::Bold => token_regex!(r"(?:.|\n)(\*\*)(?:.|\n)"),
            TokenKind::LiteralString => token_regex!(r"(?:^|\n)(.*)(?:\n|$)"),
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
        assert!(TokenKind::Transition.is_match("\n====\n"));
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
