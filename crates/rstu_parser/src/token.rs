// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;

static RECOMMENDED_SECTION_CHARS: &str = "=\\-`:.'\"~\\^_\\*\\+#"; // escaped =-`:.'"~^_*+#
static INLINE_PRE_CHARS: &str = r#"(?:[\n\s\-:/'"<(\[{]|\p{Ps}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po})"#;
static INLINE_POST_CHARS: &str =
    r#"(?:[\n\s\-\.,:;!?\\/'")\]}>]|\p{Pe}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po})"#;

// Match groups are used in the parser, but not the lexer (performance)
// Content is the same, update together.
pub(crate) static SHORT_OPTIONS_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s?([\-\+][a-z]{1})([\s=]?)(.*)$").unwrap());
pub(crate) static LONG_OPTIONS_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s?(\-\-[a-z]+(?:[\-_][a-z]+)*)([\s=]?)(.*)$").unwrap());
pub(crate) static DOS_OPTIONS_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s?(/[A-Z]+)([\s=]?)([a-z]*)$").unwrap());
static OPTION: &str = concat!(
    r"\n(?:",
    r"[\-\+][a-z](?:\s[a-z]+)?", // short and old GNU (+)
    "|",
    r"\-\-[a-z]+(?:[\-_][a-z]+)*(?:(=|\s)[a-z]+)?", // long
    "|",
    r"/[A-Z]+(?:[\s=][a-z]+)?", // DOS
    r")(?:\s|,|\n)"
);

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

macro_rules! is_token_category {
    ($cat:expr) => {
        |kind: &TokenKind| -> bool { $cat.iter().any(|inner_cat| inner_cat.contains(kind)) }
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

    pub fn indent(width: usize) -> Self {
        Self::new(TokenKind::Indent, space!(width))
    }

    pub fn dedent(width: usize) -> Self {
        Self::new(TokenKind::Dedent, space!(width))
    }

    pub fn len(&self) -> usize {
        self.lexeme.len()
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
    pub const PLAIN: &'static [TokenKind] = &[
        TokenKind::Spaces,
        TokenKind::Word,
        TokenKind::EscapedChar,
        TokenKind::Punctuation,
        TokenKind::LiteralChar,
        TokenKind::NewLine,
    ];

    pub const PARAGRAPH_END: &'static [TokenKind] = &[
        TokenKind::BlankLine,
        TokenKind::Separator,
        TokenKind::Indent,
        TokenKind::Dedent,
        TokenKind::LiteralBlock,
        TokenKind::LiteralBlockMinimized,
        TokenKind::LiteralBlockPartiallyMinimized,
        TokenKind::EoF,
    ];

    pub const NEWLINE: &'static [TokenKind] = &[
        TokenKind::BlankLine,
        TokenKind::EoF,
        TokenKind::LiteralBlock,
        TokenKind::LiteralBlockMinimized,
        TokenKind::LiteralBlockPartiallyMinimized,
        TokenKind::NewLine,
    ];

    pub const LIST_MARKER: &'static [TokenKind] = &[
        TokenKind::BulletListMarker,
        TokenKind::EnumeratedListMarker,
        TokenKind::Field,
        TokenKind::Option,
    ];

    pub const OPTION_MARKER: &'static [TokenKind] = &[TokenKind::Option];

    pub const TABLE: &'static [TokenKind] = &[TokenKind::TableHorizontal];

    // nested categories

    pub const PARAGRAPH: &'static [&[TokenKind]] = &[
        TokenCategory::INLINE_MARKER,
        TokenCategory::INLINE_TOKEN,
        TokenCategory::PLAIN,
    ];

    pub const BODY_ELEMENTS: &'static [&[TokenKind]] = &[
        TokenCategory::LIST_MARKER,
        // PARAGRAPH (type system does not allow nesting of a nested list)
        TokenCategory::INLINE_MARKER,
        TokenCategory::INLINE_TOKEN,
        TokenCategory::PLAIN,
        // LITERAL_BLOCK
        &[
            TokenKind::LiteralBlock,
            TokenKind::LiteralBlockMinimized,
            TokenKind::LiteralBlockPartiallyMinimized,
        ],
        &[TokenKind::Directive],
        // COMMENT
        &[TokenKind::DoubleDot],
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    BackquoteEnd,
    BackquoteStart,
    BlankLine,
    BulletListMarker,
    ClassifierSeparator,
    Dedent,
    Directive,
    DoubleDot,
    EmphasisEnd,
    EmphasisStart,
    EnumeratedListMarker,
    EoF,
    EscapedChar,
    Field,
    FootnoteReference,
    HyperlinkReferenceEnd,
    Indent,
    InlineInternalTargetStart,
    InlineLiteralEnd,
    InlineLiteralStart,
    LiteralBlock,
    LiteralBlockMinimized,
    LiteralBlockPartiallyMinimized,
    LiteralChar,
    NewLine,
    Option,
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
        (BlankLine, r"\n[ \t]*\n+(.|\n)"),
        (NewLine, r"[^\n]\n(.|\n)"),

        (Directive, r"[\n\s]\.\.\s(?:\|[^|\n]+\|\s)?[\p{L}\p{N}]+(?:[-_+:.][\p{L}\p{N}]+)*::\s"),

        // Comments
        (DoubleDot, r"[\n\s]\.\.[\n\s]"),

        // Literal block
        (LiteralBlockMinimized, r"[^\s\t]::(.|\n)"),
        (LiteralBlockPartiallyMinimized, r".\s::(.|\n)"),
        (LiteralBlock, r"(.|\n)::(.|\n)"),

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
        (SubstitutionReference, format!(r"{0}\|.+?\|{1}", INLINE_PRE_CHARS, INLINE_POST_CHARS)),
        // TODO SubsRefHyperLink rst l.3033
        // TODO SubRefAnonymousHyperlink
        (FootnoteReference, format!(r"{0}\[.+?\]_{1}", INLINE_PRE_CHARS, INLINE_POST_CHARS)),
        (HyperlinkReferenceEnd, format!(r"(?:[^\s]`_|[^\s]_){}", INLINE_POST_CHARS)),
        (SimpleAnonymousHyperLinkReference,r"[\s\n]\w+__\s"),
        (SimpleHyperlinkReference,r"[\s\n]\w+_\s"),

        // Classifiers for definition list (must precede Field: " : x :" would match Field)
        (ClassifierSeparator,r"(.|\n)\s+:\s+(.|\n)"),

        // Lists
        (Field,r"[\n\s]:[\w\s]+:[\n\s]"),
        (EnumeratedListMarker, r"[\n\s](?:(?:#|[0-9]+|[A-Za-z]+|[IVXLCDMivxlcdm]+)(?:\.|\))[ \t]|\([A-Za-z0-9]+\)[ \t])"),
        (BulletListMarker, r"(\s|\n)[\-\+\*•‣⁃](\s|\n)"),
        (Option, OPTION),

        // Plain text
        (Spaces, r"[^ \t\n][ \t]+[^ \t]"),
        (Word, r"[^\w]\w+[^\w]"),
        (EscapedChar,r"(.|\n)\\.(.|\n)"),
        (Punctuation, r"(.|\n)[[:punct:]](.|\n)"),

        (Dedent, r"\b\B"), // never matches, assigned by the lexer
        (EoF, r"\b\B"), // never matches, only assigned by lexer / returned by TokenStream::kind_at
        (LiteralChar, r"(.|\n).(.|\n)"),
    );

    pub fn is(self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self)
    }

    pub fn nested_is(self, categories: &[&'static [TokenKind]]) -> bool {
        is_token_category!(categories)(&self)
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
    use super::{Token, TokenCategory, TokenKind as TK};

    /// GIVEN a token with a multibyte lexeme
    /// WHEN its length is queried
    /// THEN the byte length is returned
    #[test]
    fn token_len_returns_lexeme_byte_length() {
        let token = Token::new(TK::Word, "é");

        assert_eq!(token.len(), 2);
    }

    #[test]
    fn indent_and_dedent_constructors_create_space_tokens() {
        assert_eq!(Token::indent(3), Token::new(TK::Indent, "   "));
        assert_eq!(Token::dedent(2), Token::new(TK::Dedent, "  "));
    }

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
    fn literal_block_matches() {
        assert!(TK::LiteralBlock.is_match("e::\nt"));
    }

    #[test]
    fn literal_block_non_matching() {
        assert!(!TK::LiteralBlock.is_match("e:\n"));
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
    fn directive_matches() {
        assert!(TK::Directive.is_match("\n.. image::\n"));
        assert!(TK::Directive.is_match("\n.. |name| replace:: text\n"));
        assert!(TK::Directive.is_match("\n.. |name surname| replace-text:: text\n"));
        assert!(TK::Directive.is_match("\n.. custom_name+type.v2:: text\n"));
        assert!(TK::Directive.is_match("\n.. domain:directive:: text\n"));
        assert!(TK::Directive.is_match("\n.. ImAgE:: text\n"));
        assert!(TK::Directive.is_match("\n.. | name| replace:: text\n"));
        assert!(TK::Directive.is_match("\n.. |name | replace:: text\n"));
        assert!(TK::Directive.is_match("\n.. | name | replace:: text\n"));
    }

    #[test]
    fn directive_non_matching() {
        assert!(!TK::Directive.is_match("\n.. image:\n"));
        assert!(!TK::Directive.is_match("\n.. |name replace:: text\n"));
        assert!(!TK::Directive.is_match("\n.. custom--type:: text\n"));
        assert!(!TK::Directive.is_match("\n.. -custom:: text\n"));
        assert!(!TK::Directive.is_match("\n.. custom_:: text\n"));
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
    fn enumerated_list_marker_matches() {
        assert!(TK::EnumeratedListMarker.is_match("\n1. item\n"));
        assert!(TK::EnumeratedListMarker.is_match("\n(A) item\n"));
        assert!(TK::EnumeratedListMarker.is_match("\niv) item\n"));
        assert!(TK::EnumeratedListMarker.is_match("\n#. item\n"));
    }

    #[test]
    fn enumerated_list_marker_non_matching() {
        assert!(!TK::EnumeratedListMarker.is_match("x1. item"));
        assert!(!TK::EnumeratedListMarker.is_match("\n1 item\n"));
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
    fn option_group_matches_old_gnu_style() {
        assert!(TK::Option.is_match("\n+a  "));
        assert_eq!(TK::Option.find_lexeme("\n+b file  "), Some("+b file"));
        assert!(!TK::Option.is_match("\n+1 "));
    }

    #[test]
    fn field_token() {
        assert_eq!(TK::Field.find_lexeme("\n:field1:\n"), Some(":field1:"));
        assert_eq!(TK::Field.find_lexeme(" :F_2: Some value"), Some(":F_2:"));
        assert_eq!(TK::Field.find_lexeme(" :F_2:Some value"), None);
        assert_eq!(TK::Field.find_lexeme("\n:F$x: "), None);
    }
}
