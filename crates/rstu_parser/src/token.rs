// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;

static RECOMMENDED_SECTION_CHARS: &str = "=\\-`:.'\"~\\^_\\*\\+#"; // escaped =-`:.'"~^_*+#
static INLINE_PRE_CHARS: &str = r#"(?:[\n\s\-:/'"<(\[{]|\p{Ps}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po})"#;
static INLINE_POST_CHARS: &str =
    r#"(?:[\n\s\-\.,:;!?\\/'")\]}>]|\p{Pe}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po})"#;

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
    ($(($kind:ident, $pattern:expr)),+ $(,)?) => {
        pub const ALL: [TokenKind; count_idents!($($kind),+)] = [
            $(TokenKind::$kind),+
        ];

        pub fn regex(self) -> &'static Regex {
            match self {
                $(TokenKind::$kind => compiled_regex!(format!(r"^{}",$pattern)),)+
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

pub struct TokenCategory;

impl TokenCategory {
    pub const DIRECTIVE_LIKE: &'static [TokenKind] =
        &[TokenKind::DoubleDot, TokenKind::DoubleColon];
    pub const INLINE_MARKER: &'static [TokenKind] = &[
        TokenKind::Strong,
        TokenKind::Emphasis,
        TokenKind::InterpretedText,
        TokenKind::InlineLiteral,
        TokenKind::SubstitutionReference,
        TokenKind::HyperlinkReference,
        TokenKind::InlineInternalTarget,
    ];
    pub const STRUCTURAL: &'static [TokenKind] = &[TokenKind::Separator];
    pub const CONTROL: &'static [TokenKind] =
        &[TokenKind::Indent, TokenKind::BlankLine, TokenKind::NewLine];
    pub const PLAIN: &'static [TokenKind] = &[
        TokenKind::Spaces,
        TokenKind::Word,
        TokenKind::Punctuation,
        TokenKind::LiteralChar,
    ];
    pub const TABLE: &'static [TokenKind] = &[TokenKind::TableHorizontal];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Separator,
    Indent,
    Dedent,
    Spaces,
    DoubleDot,
    DoubleColon,
    TableHorizontal,
    BlankLine,
    NewLine,
    Word,
    Punctuation,
    Strong,
    Emphasis,
    InterpretedText,
    InlineLiteral,
    SubstitutionReference,
    InlineInternalTarget,
    FootnoteReferenceOpen,
    FootnoteReferenceClose,
    HyperlinkReference,
    LiteralChar,
}

impl TokenKind {
    pub fn is(self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self)
    }

    pub fn match_token(input: &str) -> Option<(Self, &str)> {
        Self::ALL.iter().find_map(|&kind| {
            kind.find(input)
                .map(|lexeme| (kind, &lexeme[1..lexeme.len() - 1]))
        })
    }

    token_kinds!(
        // IMPORTANT:
        // The order of the enum matters, as the first matching token will be picked.
        // Format (name, context length, context regex, token regex)
        (
            Separator,
            format!(r"\n[{0}]{{4,}}\n", RECOMMENDED_SECTION_CHARS)
        ),
        (Indent, r"\n[ \t]+[^ \t\n]"),
        (Spaces, r"[^ \t\n][ \t]+[^ \t]"),
        (DoubleDot, r"[\n\s]\.\.[\n\s]"),
        (DoubleColon, r"(.|\n)::(.|\n)"),
        (TableHorizontal, r"\n=+(?:\s+=+)+\s*\n"),
        (BlankLine, r"\n[ \t]*\n(.|\n)"),
        (NewLine, r"[^\n]\n(.|\n)"),
        (Word, r"[^A-Za-z0-9_][A-Za-z0-9_]+[^A-Za-z0-9_]"),
        (
            Strong,
            format!(
                r"(?:{0}\*\*[^\s]|[^\s]\*\*{1})",
                INLINE_PRE_CHARS, INLINE_POST_CHARS
            )
        ),
        (
            Emphasis,
            format!(
                r"(?:{0}\*[^\s]|[^\s]\*{1})",
                INLINE_PRE_CHARS, INLINE_POST_CHARS
            )
        ),
        (
            InlineLiteral,
            format!(
                r"(?:{0}``[^\s]|[^\s]``{1})",
                INLINE_PRE_CHARS, INLINE_POST_CHARS
            )
        ),
        (
            InterpretedText,
            format!(
                r"(?:{0}`[^\s]|[^\s]`{1})",
                INLINE_PRE_CHARS, INLINE_POST_CHARS
            )
        ),
        (
            SubstitutionReference,
            format!(
                r"(?:{0}\|[^\s]|[^\s]\|{1})",
                INLINE_PRE_CHARS, INLINE_POST_CHARS
            )
        ),
        (
            InlineInternalTarget,
            format!(r"{0}_`[^\s]", INLINE_PRE_CHARS)
        ),
        (
            FootnoteReferenceOpen,
            format!(r"{0}\[[^\s]", INLINE_PRE_CHARS)
        ),
        (
            FootnoteReferenceClose,
            format!(r"[^\s]\]_{0}", INLINE_POST_CHARS)
        ),
        (HyperlinkReference, format!(r"[^\s]_{0}", INLINE_POST_CHARS)),
        (Punctuation, r"(.|\n)[[:punct:]](.|\n)"),
        (LiteralChar, r"(.|\n).(.|\n)"),
        (Dedent, r"") // never matches, assigned by the lexer
    );

    // tests according to inline markup recognition rules.

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
    use super::{TokenCategory, TokenKind};

    #[test]
    fn match_token_uses_centralized_token_list() {
        let (kind, lexeme) = TokenKind::match_token("\nHello\n").unwrap();

        assert_eq!(kind, TokenKind::Word);
        assert_eq!(lexeme, "Hello");
    }

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
        assert!(TokenKind::Strong.is_match(" **x"));
        assert!(TokenKind::Strong.is_match("x** "));
    }

    #[test]
    fn bold_non_matching() {
        assert!(!TokenKind::Strong.is_match("*"));
    }

    #[test]
    fn inline_markup_tokens_match_common_delimiters() {
        assert!(TokenKind::Emphasis.is_match(" *x"));
        assert!(TokenKind::InterpretedText.is_match(" `x"));
        assert!(TokenKind::InlineLiteral.is_match(" ``x"));
        assert!(TokenKind::SubstitutionReference.is_match(" |x"));
        assert!(TokenKind::HyperlinkReference.is_match("x_ "));
        assert!(TokenKind::FootnoteReferenceOpen.is_match(" [x"));
    }

    // TODO: exclude escaped characters

    #[test]
    fn emphasis_non_matching_for_strong_delimiters() {
        assert!(!TokenKind::Emphasis.is_match("**"));
    }

    #[test]
    fn interpreted_text_non_matching_for_double_backticks() {
        assert!(!TokenKind::InterpretedText.is_match("``"));
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

    #[test]
    fn punctuation_matches_ascii_non_alphanumeric() {
        assert!(TokenKind::Punctuation.is_match("x,x"));
        assert!(TokenKind::Punctuation.is_match("x!x"));
        assert!(TokenKind::Punctuation.is_match("x_x"));
    }

    #[test]
    fn punctuation_non_matching_for_alphanumeric() {
        assert!(!TokenKind::Punctuation.is_match("xax"));
        assert!(!TokenKind::Punctuation.is_match("x1x"));
    }

    #[test]
    fn kind_is_matches_category_membership() {
        assert!(TokenKind::Strong.is(TokenCategory::INLINE_MARKER));
        assert!(TokenKind::Word.is(TokenCategory::PLAIN));
        assert!(TokenKind::Punctuation.is(TokenCategory::PLAIN));
        assert!(!TokenKind::Separator.is(TokenCategory::PLAIN));
    }
}
