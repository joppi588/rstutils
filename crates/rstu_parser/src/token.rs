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
macro_rules! compiled_regex {
    ($pattern:expr) => {{
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(format!(r"^{}", $pattern).as_ref()).unwrap());
        &RE
    }};
}
macro_rules! token_kinds {
    ($(($kind:ident, $pattern:expr, $category:ident)),+ $(,)?) => {
        pub const ALL: [TokenKind; count_idents!($($kind),+)] = [
            $(TokenKind::$kind),+
        ];

        pub fn regex(self) -> &'static Regex {
            match self {
                $(TokenKind::$kind => compiled_regex!(format!(r"^{}",$pattern)),)+
            }
        }

        pub fn category(self) -> TokenCategory {
            match self {
                $(TokenKind::$kind => TokenCategory::$category,)+
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

    pub fn category(&self) -> TokenCategory {
        self.kind.category()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenCategory {
    DirectiveLike,
    Inline,
    Structural,
    Control,
    Plain,
    Table,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Separator,
    Indent,
    Spaces,
    DoubleDot,
    DoubleColon,
    TableHorizontal,
    BlankLine,
    NewLine,
    Word,
    Strong,
    LiteralChar,
}

impl TokenKind {
    token_kinds!(
        // IMPORTANT:
        // The order of the enum matters, as the first matching token will be picked.
        // Format (name, context length, context regex, token regex)
        (
            Separator,
            format!(r"\n[{0}]{{4,}}\n", RECOMMENDED_SECTION_CHARS),
            Structural
        ),
        (Indent, r"\n[ \t]+[^ \t\n]", Control),
        (Spaces, r"[^ \t\n][ \t]+[^ \t]", Plain),
        (DoubleDot, r"[\n\s]\.\.[\n\s]", DirectiveLike),
        (DoubleColon, r"(.|\n)::(.|\n)", DirectiveLike),
        (TableHorizontal, r"\n=+(?:\s+=+)+\s*\n", Table),
        (BlankLine, r"\n[ \t]*\n(.|\n)", Control),
        (NewLine, r"[^\n]\n(.|\n)", Control),
        (Word, r"[^A-Za-z0-9_][A-Za-z0-9_]+[^A-Za-z0-9_]", Plain),
        (Strong, r"(.|\n)\*\*(.|\n)", Inline),
        (LiteralChar, r"(.|\n)[\s\S](.|\n)", Plain)
    );

    pub fn find(self, input: &str) -> Option<&str> {
        self.regex().find(input).map(|m| m.as_str())
    }

    pub fn is_match(self, input: &str) -> bool {
        let result = self.find(input);
        result.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::TokenKind;

    #[test]
    fn transition_matches() {
        assert!(TokenKind::Separator.is_match("\n====\n"));
        assert!(!TokenKind::Separator.is_match("\n==a=\n"));
        assert!(!TokenKind::Separator.is_match("\n===\n"));
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
        assert!(TokenKind::Strong.is_match("\n**\n"));
    }

    #[test]
    fn bold_non_matching() {
        assert!(!TokenKind::Strong.is_match("*"));
    }

    #[test]
    fn doublecolon_matches() {
        assert!(TokenKind::DoubleColon.is_match("e::\n"));
    }

    #[test]
    fn doublecolon_non_matching() {
        assert!(!TokenKind::DoubleColon.is_match("e:\n"));
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
