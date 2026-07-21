// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;

static RECOMMENDED_SECTION_CHARS: &str = "=\\-`:.'\"~\\^_\\*\\+#"; // escaped =-`:.'"~^_*+#

macro_rules! count_idents {
    ($($ident:ident),* $(,)?) => {
        <[()]>::len(&[$(count_idents!(@sub $ident)),*])
    };
    (@sub $ident:ident) => {
        ()
    };
}
macro_rules! token_regex {
    ($pattern:expr) => {{
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern.as_ref()).unwrap());
        &RE
    }};
}
macro_rules! token_kinds {
    ($(($kind:ident, $leading:expr, $pattern:expr, $trailing:expr)),+ $(,)?) => {
        pub const ALL: [TokenKind; count_idents!($($kind),+)] = [
            $(TokenKind::$kind),+
        ];

        pub fn regex(self) -> &'static Regex {
            match self {
                $(TokenKind::$kind => token_regex!(format!(r"{}({}){}", $leading, $pattern, $trailing)),)+
            }
        }
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

impl TokenKind {
    token_kinds!(
        // IMPORTANT:
        // The order of the enum matters, as the first matching token will be picked.
        // When using capture groups, use the non-matching style (?:___)
        (
            Transition,
            r"\n\n",
            format!(r"[{0}]{{4,}}", RECOMMENDED_SECTION_CHARS),
            r"\n\n"
        ),
        (
            SectionTitlePrefix,
            r"\n\n",
            format!(r"[{0}]+", RECOMMENDED_SECTION_CHARS),
            r"\n"
        ),
        (
            SectionTitleSuffix,
            r"\n",
            format!(r"[{0}]+", RECOMMENDED_SECTION_CHARS),
            r"\n"
        ),
        (Indent, r"\n", r"[ \t]+", r"[^ \t\n]"),
        (Spaces, r"[^ \t\n]", r"[ \t]+", r"[^ \t]"),
        (DoubleDot, r"[\n\s]", r"\.\.", r"[\n\s]"),
        (DoubleColon, r"(?:.|\n)", r"::", r"(?:.|\n)"),
        (TableHorizontal, r"\n", r"=+(?:\s+=+)+\s*", r"\n"),
        (BlankLine, r"\n", r"[ \t]*\n", r"(?:.|\n)"),
        (NewLine, r"[^\n]", r"\n", r"(?:.|\n)"),
        (Word, r"[^A-Za-z0-9_]", r"[A-Za-z0-9_]+", r"[^A-Za-z0-9_]"),
        (Bold, r"(?:.|\n)", r"\*\*", r"(?:.|\n)"),
        (LiteralString, r"\n", r".*", r"\n")
    );

    pub fn inner_match<'a>(self, input: &'a str) -> Option<&'a str> {
        self.regex()
            .captures(input)
            .and_then(|captures| captures.get(1).map(|m| m.as_str()))
    }

    pub fn is_match(self, input: &str) -> bool {
        let result = self.inner_match(input);
        result.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::TokenKind;

    #[test]
    fn transition_matches() {
        assert!(TokenKind::Transition.is_match("\n\n====\n\n"));
        assert!(!TokenKind::Transition.is_match("\n\n==a=\n\n"));
        assert!(!TokenKind::Transition.is_match("\n\n===\n\n"));
    }

    #[test]
    fn section_title_prefix_matches() {
        assert!(TokenKind::SectionTitlePrefix.is_match("\n\n====\nTitle"));
        assert!(!TokenKind::SectionTitlePrefix.is_match("\n\n==a=\nTitle"));
        assert!(!TokenKind::SectionTitlePrefix.is_match("Title\n====\n"));
    }

    #[test]
    fn section_title_suffix_matches() {
        assert!(TokenKind::SectionTitleSuffix.is_match("Title\n=====\nParagraph"));
        assert!(!TokenKind::SectionTitleSuffix.is_match("Title\n==a=\n\n"));
        assert!(
            TokenKind::SectionTitlePrefix.is_match("\n\n====\nTitle")
                && TokenKind::SectionTitleSuffix.is_match("\n\n====\nTitle")
        ); // but Prefix is catched first!
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
        assert!(!TokenKind::DoubleColon.is_match("\n.. note:\n"));
    }

    #[test]
    fn doubledot_matches() {
        assert!(TokenKind::DoubleDot.is_match("\n.. this is a comment\n"));
    }

    #[test]
    fn doubledot_non_matching() {
        assert!(!TokenKind::DoubleDot.is_match("\nwarning...\n"));
    }

    #[test]
    fn table_horizontal_matches() {
        assert!(TokenKind::TableHorizontal.is_match("\n==== =====\n"));
    }

    #[test]
    fn table_horizontal_non_matching() {
        assert!(!TokenKind::TableHorizontal.is_match("\n========\n"));
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
        assert!(TokenKind::LiteralString.is_match("\nHello world\n"));
    }

    #[test]
    fn word_matches_alphanumeric_and_underscore() {
        assert!(TokenKind::Word.is_match(" alpha_123 "));
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
