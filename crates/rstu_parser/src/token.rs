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

    pub fn is(&self, kinds: &[TokenKind]) -> bool {
        self.kind.is(kinds)
    }
}

pub struct TokenCategory;

impl TokenCategory {
    pub const DIRECTIVE_LIKE: &'static [TokenKind] =
        &[TokenKind::DoubleDot, TokenKind::DoubleColon];
    pub const INLINE_MARKER: &'static [TokenKind] = &[
        TokenKind::StrongStart,
        TokenKind::EmphasisStart,
        TokenKind::InlineLiteralStart,
        TokenKind::BackquoteStart,
        TokenKind::InlineInternalTargetStart,
    ];
    pub const INLINE_TOKEN: &'static [TokenKind] = &[
        TokenKind::SubstitutionReference,
        TokenKind::FootnoteReference,
        TokenKind::SimpleHyperlinkReference,
        TokenKind::SimpleAnonymousHyperLinkReference,
    ];
    pub const STRUCTURAL: &'static [TokenKind] = &[TokenKind::Separator];
    pub const CONTROL: &'static [TokenKind] = &[
        TokenKind::Indent,
        TokenKind::Dedent,
        TokenKind::BlankLine,
        TokenKind::NewLine,
    ];
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
    BackquoteEnd,
    BackquoteStart,
    BlankLine,
    BulletListMarker,
    Dedent,
    DoubleColon,
    DoubleDot,
    EmphasisEnd,
    EmphasisStart,
    Field,
    FootnoteReference,
    HyperlinkReferenceEnd,
    Indent,
    InlineInternalTargetStart,
    InlineLiteralEnd,
    InlineLiteralStart,
    LiteralChar,
    NewLine,
    Punctuation,
    Separator,
    SimpleAnonymousHyperLinkReference,
    SimpleHyperlinkReference,
    Spaces,
    StrongEnd,
    StrongStart,
    SubstitutionReference,
    TableHorizontal,
    Word,
}

impl TokenKind {
    #[rustfmt::skip]
    token_kinds!(
        // IMPORTANT:
        // The order of the enum matters, as the first matching token will be picked.
        // Format (name, token regex)
        (Separator, format!(r"\n[{0}]{{4,}}\n", RECOMMENDED_SECTION_CHARS)),

        (Indent, r"\n[ \t]+[^ \t\n]"),
        (BlankLine, r"\n[ \t]*\n(.|\n)"),
        (NewLine, r"[^\n]\n(.|\n)"),

        // Directive-Like
        (DoubleDot, r"[\n\s]\.\.[\n\s]"),
        (DoubleColon, r"(.|\n)::(.|\n)"),

        // Lists
        (Field,r"[\n\s]:\w+:[\n\s]"),

        (TableHorizontal, r"\n=+(?:\s+=+)+\s*\n"),

        // Inline
        // Keep recognition order aligned with the spec: strong before emphasis,
        // inline literals and inline internal targets before backquote constructs.
        (StrongStart, format!(r"{0}\*\*[^\s]", INLINE_PRE_CHARS)),
        (StrongEnd, format!(r"[^\s]\*\*{0}", INLINE_POST_CHARS)),
        (EmphasisStart, format!(r"{0}\*[^\s]", INLINE_PRE_CHARS)),
        (EmphasisEnd, format!(r"[^\s]\*{0}", INLINE_POST_CHARS)),
        (InlineLiteralStart, format!(r"{0}``[^\s]", INLINE_PRE_CHARS)),
        (InlineLiteralEnd, format!(r"[^\s]``{0}", INLINE_POST_CHARS)),
        (InlineInternalTargetStart, format!(r"{0}_`[^\s]", INLINE_PRE_CHARS)),
        (BackquoteStart, format!(r"{0}`[^\s]", INLINE_PRE_CHARS)),
        (BackquoteEnd, format!(r"[^\s]`{0}", INLINE_POST_CHARS)),

        // Inline references
        (SubstitutionReference, format!(r"{0}\|.+\|{1}", INLINE_PRE_CHARS, INLINE_POST_CHARS)),
        // TODO SubsRefHyperLink rst l.3033
        // TODO SubRefAnonymousHyperlink
        (FootnoteReference, format!(r"{0}\[.+\]_{1}", INLINE_PRE_CHARS, INLINE_POST_CHARS)),
        (HyperlinkReferenceEnd, format!(r"(?:[^\s]`_|[^\s]_){}", INLINE_POST_CHARS)),
        (SimpleAnonymousHyperLinkReference,r"[\s\n]\w+__\s"),
        (SimpleHyperlinkReference,r"[\s\n]\w+_\s"),

        // Lists
        (BulletListMarker, r"(\s|\n)[\-\+\*•‣⁃]\s"),


        // Plain text
        (Spaces, r"[^ \t\n][ \t]+[^ \t]"),
        (Word, r"[^\w]\w+[^\w]"),
        (Punctuation, r"(.|\n)[[:punct:]](.|\n)"),

        (Dedent, r"\b\B"), // never matches, assigned by the lexer
        (LiteralChar, r"(.|\n).(.|\n)"),
    );

    pub fn is(self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self)
    }

    pub fn match_token(input: &str) -> Option<(Self, &str)> {
        Self::ALL
            .iter()
            .find_map(|&kind| kind.find_lexeme(input).map(|lexeme| (kind, lexeme)))
    }

    pub fn find_lexeme(self, input: &str) -> Option<&str> {
        self.regex()
            .find(input)
            .map(|m| &(m.as_str())[1..m.len() - 1])
    }

    pub fn is_match(self, input: &str) -> bool {
        let result = self.find_lexeme(input);
        result.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenCategory, TokenKind as TK};

    #[test]
    fn match_token_uses_centralized_token_list() {
        let (kind, lexeme) = TK::match_token("\nHello\n").unwrap();

        assert_eq!(kind, TK::Word);
        assert_eq!(lexeme, "Hello");
    }

    #[test]
    fn transition_matches() {
        assert!(TK::Separator.is_match("\n====\n"));
        assert!(!TK::Separator.is_match("\n==a=\n"));
        assert!(!TK::Separator.is_match("\n===\n"));
    }

    #[test]
    fn indent_matches() {
        assert!(TK::Indent.is_match("\n \t  W"));
    }

    #[test]
    fn indent_non_matching() {
        assert!(!TK::Indent.is_match("abc"));
    }

    #[test]
    fn spaces_matches() {
        assert!(TK::Spaces.is_match("x \t x"));
    }

    #[test]
    fn spaces_non_matching() {
        assert!(!TK::Spaces.is_match("xabcx"));
    }

    #[test]
    fn strong_matches() {
        assert!(TK::StrongStart.is_match(" **x"));
        assert!(TK::StrongEnd.is_match("x** "));
        assert!(!TK::StrongStart.is_match("*"));
        assert!(!TK::StrongEnd.is_match("*"));
    }

    #[test]
    fn inline_markup_tokens_match_common_delimiters() {
        assert!(TK::EmphasisStart.is_match(" *x"));
        assert!(TK::EmphasisEnd.is_match("x* "));
        assert!(TK::BackquoteStart.is_match(" `x"));
        assert!(TK::BackquoteEnd.is_match("x` "));
        assert!(TK::InlineLiteralStart.is_match(" ``x"));
        assert!(TK::InlineLiteralEnd.is_match("x`` "));
    }

    #[test]
    fn inline_references() {
        assert!(TK::SubstitutionReference.is_match(" |x| "));
        assert!(TK::HyperlinkReferenceEnd.is_match("x_ "));
        assert!(TK::HyperlinkReferenceEnd.is_match("x`_ "));
        assert!(TK::FootnoteReference.is_match(" [x]_ "));
        assert_eq!(
            TK::SimpleHyperlinkReference.find_lexeme(" simple_ref_ text"),
            Some("simple_ref_")
        );
    }

    // TODO: exclude escaped characters

    #[test]
    fn emphasis_non_matching_for_strong_delimiters() {
        assert!(!TK::EmphasisStart.is_match("**"));
    }

    #[test]
    fn interpreted_text_non_matching_for_double_backticks() {
        assert!(!TK::BackquoteStart.is_match("``"));
    }

    #[test]
    fn doublecolon_matches() {
        assert!(TK::DoubleColon.is_match("e::\n"));
    }

    #[test]
    fn doublecolon_non_matching() {
        assert!(!TK::DoubleColon.is_match("e:\n"));
    }

    #[test]
    fn doubledot_matches() {
        assert!(TK::DoubleDot.is_match("\n.. this is a comment\n"));
    }

    #[test]
    fn doubledot_non_matching() {
        assert!(!TK::DoubleDot.is_match("\nwarning...\n"));
    }

    #[test]
    fn bullet_list_marker_matches() {
        assert!(TK::BulletListMarker.is_match("\n- item\n"));
        assert!(TK::BulletListMarker.is_match("\n+ item\n"));
    }

    #[test]
    fn bullet_list_marker_non_matching() {
        assert!(!TK::BulletListMarker.is_match("x-y"));
    }

    #[test]
    fn table_horizontal_matches() {
        assert!(TK::TableHorizontal.is_match("\n==== =====\n"));
    }

    #[test]
    fn table_horizontal_non_matching() {
        assert!(!TK::TableHorizontal.is_match("\n========\n"));
    }

    #[test]
    fn blank_line_matches_empty() {
        assert!(TK::BlankLine.is_match("\n\n\n"));
    }

    #[test]
    fn blank_line_matches_whitespace_only() {
        assert!(TK::BlankLine.is_match("\n \t\n\n"));
    }

    #[test]
    fn blank_line_non_matching_text() {
        assert!(!TK::BlankLine.is_match("text"));
    }

    #[test]
    fn word_matches_alphanumeric_and_underscore() {
        assert!(TK::Word.is_match(" alpha_123 "));
    }

    #[test]
    fn word_matches_with_newline_boundary() {
        assert!(TK::Word.is_match("\nalpha_123\n"));
    }

    #[test]
    fn word_non_matching_without_word_chars() {
        assert!(!TK::Word.is_match("---\n***"));
    }

    #[test]
    fn punctuation_matches_ascii_non_alphanumeric() {
        assert!(TK::Punctuation.is_match("x,x"));
        assert!(TK::Punctuation.is_match("x!x"));
        assert!(TK::Punctuation.is_match("x_x"));
    }

    #[test]
    fn punctuation_non_matching_for_alphanumeric() {
        assert!(!TK::Punctuation.is_match("xax"));
        assert!(!TK::Punctuation.is_match("x1x"));
    }

    #[test]
    fn kind_is_matches_category_membership() {
        assert!(TK::StrongStart.is(TokenCategory::INLINE_MARKER));
        assert!(TK::Word.is(TokenCategory::PLAIN));
        assert!(TK::Punctuation.is(TokenCategory::PLAIN));
        assert!(!TK::Separator.is(TokenCategory::PLAIN));
    }

    #[test]
    fn field_token() {
        assert_eq!(TK::Field.find_lexeme("\n:field1:\n"), Some(":field1:"));
        assert_eq!(TK::Field.find_lexeme(" :F_2: Some value"), Some(":F_2:"));
        assert_eq!(TK::Field.find_lexeme(" :F_2:Some value"), None);
        assert_eq!(TK::Field.find_lexeme("\n:F$x: "), None);
    }
}
