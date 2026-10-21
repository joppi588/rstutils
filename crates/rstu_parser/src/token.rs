// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;

static RECOMMENDED_SECTION_CHARS: &str = "=\\-`:.'\"~\\^_\\*\\+#"; // escaped =-`:.'"~^_*+#
static INLINE_PRE_CHARS: &str = r#"(?:[\n\s\-:/'"<(\[{]|\p{Ps}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po})"#;
static INLINE_PRE_CHARS_NO_ASTERISK: &str =
    r#"(?:[\n\s\-:/'"<(\[{]|\p{Ps}|\p{Pi}|\p{Pf}|\p{Pd}|[\p{Po}&&[^*]])"#;
static INLINE_POST_CHARS: &str =
    r#"(?:[\n\s\-\.,:;!?\\/'")\]}>]|\p{Pe}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po}|\p{Ps})"#;
static INLINE_POST_CHARS_NO_ASTERISK: &str =
    r#"(?:[\n\s\-\.,:;!?\\/'")\]}>]|\p{Pe}|\p{Pi}|\p{Pf}|\p{Pd}|[\p{Po}&&[^*]]|\p{Ps})"#;

// Match groups are used in the parser, but not the lexer (performance)
// Content is the same, update together.
pub(crate) static SHORT_OPTIONS_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s?([\-\+][a-z]{1})([ \t=]?)(.*)$").unwrap());
pub(crate) static LONG_OPTIONS_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s?(\-\-[a-z][a-z0-9]*(?:[\-_][a-z][a-z0-9]*)*)([ \t=]?)(.*)$").unwrap()
});
pub(crate) static DOS_OPTIONS_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s?(/[A-Z]+)([ \t=]?)([a-z]*)$").unwrap());
static OPTION: &str = concat!(
    r"\n(?:",
    r"[\-\+][a-z](?:[ \t][a-z]+)?", // short and old GNU (+)
    "|",
    r"\-\-[a-z][a-z0-9]*(?:[\-_][a-z][a-z0-9]*)*(?:[ \t=][a-z][a-z0-9]*)?", // long
    "|",
    r"/[A-Z]+(?:[ \t=][a-z]+)?", // DOS
    r")[ ,\n]"
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
    pub const DEFINITION_LIST: &'static [TokenKind] = &[
        TokenKind::BackquoteStart,
        TokenKind::EmphasisStart,
        TokenKind::EscapedChar,
        TokenKind::FootnoteReference,
        TokenKind::InlineInternalTargetStart,
        TokenKind::InlineLiteralStart,
        TokenKind::LiteralChar,
        TokenKind::Punctuation,
        TokenKind::SimpleAnonymousHyperLinkReference,
        TokenKind::SimpleHyperlinkReference,
        TokenKind::StrongStart,
        TokenKind::SubstitutionReference,
        TokenKind::Word,
    ];
    pub const EOL: &'static [TokenKind] = &[
        TokenKind::BlankLine,
        TokenKind::EoF,
        TokenKind::LiteralBlock,
        TokenKind::LiteralBlockMinimized,
        TokenKind::LiteralBlockPartiallyMinimized,
        TokenKind::NewLine,
    ];

    pub const INLINE_MARKER: &'static [TokenKind] = &[
        TokenKind::BackquoteStart,
        TokenKind::EmphasisStart,
        TokenKind::InlineInternalTargetStart,
        TokenKind::InlineLiteralStart,
        TokenKind::StrongStart,
    ];
    pub const INLINE_TOKEN: &'static [TokenKind] = &[
        TokenKind::FootnoteReference,
        TokenKind::SimpleAnonymousHyperLinkReference,
        TokenKind::SimpleHyperlinkReference,
        TokenKind::SubstitutionReference,
    ];

    pub const LIST_MARKER: &'static [TokenKind] = &[
        TokenKind::BulletListMarker,
        TokenKind::EnumeratedListMarker,
        TokenKind::Field,
        TokenKind::Option,
    ];

    pub const PARAGRAPH_END: &'static [TokenKind] = &[
        TokenKind::BlankLine,
        TokenKind::Dedent,
        TokenKind::EoF,
        TokenKind::Indent,
        TokenKind::LineBlock,
        TokenKind::LiteralBlock,
        TokenKind::LiteralBlockMinimized,
        TokenKind::LiteralBlockPartiallyMinimized,
        TokenKind::Separator,
    ];
    pub const PLAIN: &'static [TokenKind] = &[
        TokenKind::DoctestStart,
        TokenKind::EscapedChar,
        TokenKind::LiteralChar,
        TokenKind::NewLine,
        TokenKind::Punctuation,
        TokenKind::Spaces,
        TokenKind::Word,
    ];

    pub const OPTION_MARKER: &'static [TokenKind] = &[TokenKind::Option];

    pub const STRUCTURAL: &'static [TokenKind] = &[TokenKind::Separator];

    pub const TABLE: &'static [TokenKind] = &[TokenKind::TableHorizontal];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Attribution,
    BackquoteEnd,
    BackquoteStart,
    BlankLine,
    BulletListMarker,
    ClassifierSeparator,
    Dedent,
    Directive,
    DoctestStart,
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
    LineBlock,
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
        (Attribution, r" (?:—|--|---) +(?:.|\n)"),

        (Indent, r"\n[ \t]+[^ \t\n]"),
        (BlankLine, r"\n[ \t]*\n+(.|\n)"),
        (NewLine, r"[^\n]\n(.|\n)"),

        (Directive, r"[\n\s]\.\.\s(?:\|[^|\n]+\|\s)?[\p{L}\p{N}]+(?:[-_+:.][\p{L}\p{N}]+)*::\s"),

        (DoctestStart, r"\s>>>\s"),

        // Comments
        (DoubleDot, r"\s\.\. *\s"),

        (LineBlock,r"\n\|(?: +[^ ]|\n)"),

        // Literal block
        (LiteralBlockMinimized, r"[^\s]::\n"),
        (LiteralBlockPartiallyMinimized, r". ::\n"),
        (LiteralBlock, r"\s::\n"),

        (TableHorizontal, r"\n=+(?:\s+=+)+\s*\n"),

        // Inline
        // Keep recognition order aligned with the spec: strong before emphasis,
        // inline literals and inline internal targets before backquote constructs.
        (StrongStart, format!(r"{0}\*\*[^\s]", INLINE_PRE_CHARS_NO_ASTERISK)),
        (StrongEnd, format!(r"[^\s\\]\*\*{0}", INLINE_POST_CHARS_NO_ASTERISK)),
        (EmphasisStart, format!(r"{0}\*[^\s]", INLINE_PRE_CHARS_NO_ASTERISK)),
        (EmphasisEnd, format!(r"[^\s\\]\*{0}", INLINE_POST_CHARS_NO_ASTERISK)),
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
        (Field,r"\s:[[:word:] ]+:\s"),
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

    pub fn match_token(input: &str) -> Option<(Self, &str)> {
        Self::ALL
            .iter()
            .find_map(|&kind| kind.find_lexeme(input).map(|lexeme| (kind, lexeme)))
    }

    fn find_lexeme(self, input: &str) -> Option<&str> {
        self.regex().find(input).and_then(|matched| {
            let text = matched.as_str();
            let first_len = text.chars().next()?.len_utf8();
            let last_start = text.char_indices().next_back()?.0;
            text.get(first_len..last_start)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, TokenCategory, TokenKind as TK};
    use rstest::rstest;

    #[rstest]
    #[case("\n>>> x")]
    #[case(" >>> x")]
    #[case("\n>>>\n")]
    fn doctest_start_matches(#[case] input: &str) {
        assert_eq!(TK::DoctestStart.find_lexeme(input), Some(">>>"));
    }

    #[rstest]
    #[case("\n>>>x")]
    #[case("\n>> x")]
    #[case("a>>> x")]
    fn doctest_start_rejects_non_matching_text(#[case] input: &str) {
        assert_eq!(TK::DoctestStart.find_lexeme(input), None);
    }

    #[test]
    fn line_block() {
        assert_eq!(TK::LineBlock.find_lexeme("\n| T"), Some("| "));
    }

    #[rstest]
    #[case("--")]
    #[case("---")]
    #[case("—")]
    fn attribution_matches_supported_markers(#[case] marker: &str) {
        assert_eq!(
            TK::Attribution.find_lexeme(&format!(" {marker} Holmes")),
            Some(format!("{marker} ")).as_deref()
        );
    }

    #[test]
    fn attribution_rejects_other_dash_sequences() {
        assert_eq!(TK::Attribution.find_lexeme(" - Holmes"), None);
        assert_eq!(TK::Attribution.find_lexeme(" ---- Holmes"), None);
        assert_eq!(TK::Attribution.find_lexeme("word -- Holmes"), None);
    }

    #[test]
    fn backquote_start_rejects_double_backticks() {
        assert_eq!(TK::BackquoteStart.find_lexeme("``"), None);
    }

    #[test]
    fn blank_line_matches_empty() {
        assert_eq!(TK::BlankLine.find_lexeme("\n\n\n"), Some("\n"));
    }

    #[test]
    fn blank_line_matches_whitespace_only() {
        assert_eq!(TK::BlankLine.find_lexeme("\n \t\n\n"), Some(" \t\n"));
    }

    #[test]
    fn blank_line_rejects_non_matching_text() {
        assert_eq!(TK::BlankLine.find_lexeme("text"), None);
    }

    #[test]
    fn bullet_list_marker_matches() {
        assert_eq!(TK::BulletListMarker.find_lexeme("\n- item\n"), Some("-"));
        assert_eq!(TK::BulletListMarker.find_lexeme("\n+ item\n"), Some("+"));
    }

    #[test]
    fn bullet_list_marker_rejects_non_matching_text() {
        assert_eq!(TK::BulletListMarker.find_lexeme("x-y"), None);
    }

    #[test]
    fn directive_matches_supported_syntax() {
        assert_eq!(
            TK::Directive.find_lexeme("\n.. image::\n"),
            Some(".. image::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. |name| replace:: text\n"),
            Some(".. |name| replace::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. |name surname| replace-text:: text\n"),
            Some(".. |name surname| replace-text::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. custom_name+type.v2:: text\n"),
            Some(".. custom_name+type.v2::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. domain:directive:: text\n"),
            Some(".. domain:directive::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. ImAgE:: text\n"),
            Some(".. ImAgE::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. | name| replace:: text\n"),
            Some(".. | name| replace::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. |name | replace:: text\n"),
            Some(".. |name | replace::")
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. | name | replace:: text\n"),
            Some(".. | name | replace::")
        );
    }

    #[test]
    fn directive_rejects_unsupported_syntax() {
        assert_eq!(TK::Directive.find_lexeme("\n.. image:\n"), None);
        assert_eq!(
            TK::Directive.find_lexeme("\n.. |name replace:: text\n"),
            None
        );
        assert_eq!(
            TK::Directive.find_lexeme("\n.. custom--type:: text\n"),
            None
        );
        assert_eq!(TK::Directive.find_lexeme("\n.. -custom:: text\n"), None);
        assert_eq!(TK::Directive.find_lexeme("\n.. custom_:: text\n"), None);
    }

    #[test]
    fn double_dot_matches_comment_start() {
        assert_eq!(
            TK::DoubleDot.find_lexeme("\n.. this is a comment\n"),
            Some("..")
        );
    }

    #[test]
    fn double_dot_rejects_non_comment_text() {
        assert_eq!(TK::DoubleDot.find_lexeme("\nwarning...\n"), None);
    }

    #[test]
    fn emphasis_start_and_related_markup_tokens_match_delimiters() {
        assert_eq!(TK::EmphasisStart.find_lexeme(" *x"), Some("*"));
        assert_eq!(TK::EmphasisEnd.find_lexeme("x* "), Some("*"));
        assert_eq!(TK::BackquoteStart.find_lexeme(" `x"), Some("`"));
        assert_eq!(TK::BackquoteEnd.find_lexeme("x` "), Some("`"));
        assert_eq!(TK::InlineLiteralStart.find_lexeme(" ``x"), Some("``"));
        assert_eq!(TK::InlineLiteralEnd.find_lexeme("x`` "), Some("``"));
    }

    /// GIVEN emphasis closed by a low quotation mark used as closing punctuation
    /// WHEN the emphasis end token is matched
    /// THEN the delimiter is recognized before following text
    #[test]
    fn emphasis_end_accepts_ps_quote_delimiters() {
        assert_eq!(TK::EmphasisEnd.find_lexeme("x*„ Greek"), Some("*"));
        assert_eq!(TK::EmphasisEnd.find_lexeme("x*‚ Greek"), Some("*"));
    }

    #[test]
    fn emphasis_end_does_not_treat_escaped_asterisk_as_delimiter() {
        assert_eq!(TK::EmphasisEnd.find_lexeme("**\n"), Some("*"));
    }

    // TODO: exclude escaped characters

    #[test]
    fn emphasis_start_rejects_strong_delimiters() {
        assert_eq!(TK::EmphasisStart.find_lexeme("**"), None);
    }

    #[test]
    fn enumerated_list_marker_matches() {
        assert_eq!(
            TK::EnumeratedListMarker.find_lexeme("\n1. item\n"),
            Some("1.")
        );
        assert_eq!(
            TK::EnumeratedListMarker.find_lexeme("\n(A) item\n"),
            Some("(A)")
        );
        assert_eq!(
            TK::EnumeratedListMarker.find_lexeme("\niv) item\n"),
            Some("iv)")
        );
        assert_eq!(
            TK::EnumeratedListMarker.find_lexeme("\n#. item\n"),
            Some("#.")
        );
    }

    #[test]
    fn enumerated_list_marker_rejects_non_matching_text() {
        assert_eq!(TK::EnumeratedListMarker.find_lexeme("x1. item"), None);
        assert_eq!(TK::EnumeratedListMarker.find_lexeme("\n1 item\n"), None);
    }

    #[test]
    fn field_matches_valid_names() {
        assert_eq!(TK::Field.find_lexeme("\n:field1:\n"), Some(":field1:"));
        assert_eq!(TK::Field.find_lexeme(" :F_2: Some value"), Some(":F_2:"));
        assert_eq!(TK::Field.find_lexeme(" :F_2:Some value"), None);
        assert_eq!(TK::Field.find_lexeme("\n:F$x: "), None);
    }

    #[test]
    fn footnote_reference_and_other_inline_references_match() {
        assert_eq!(TK::SubstitutionReference.find_lexeme(" |x| "), Some("|x|"));
        assert_eq!(TK::HyperlinkReferenceEnd.find_lexeme("x_ "), Some("_"));
        assert_eq!(TK::HyperlinkReferenceEnd.find_lexeme("x`_ "), Some("`_"));
        assert_eq!(TK::FootnoteReference.find_lexeme(" [x]_ "), Some("[x]_"));
        assert_eq!(
            TK::SimpleHyperlinkReference.find_lexeme(" simple_ref_ text"),
            Some("simple_ref_")
        );
    }

    #[test]
    fn indent_and_dedent_constructors_create_space_tokens() {
        assert_eq!(Token::indent(3), Token::new(TK::Indent, "   "));
        assert_eq!(Token::dedent(2), Token::new(TK::Dedent, "  "));
    }

    #[test]
    fn indent_matches() {
        assert_eq!(TK::Indent.find_lexeme("\n \t  W"), Some(" \t  "));
    }

    #[test]
    fn indent_rejects_non_indented_text() {
        assert_eq!(TK::Indent.find_lexeme("abc"), None);
    }

    #[test]
    fn kind_is_matches_category_membership() {
        assert!(TK::StrongStart.is(TokenCategory::INLINE_MARKER));
        assert!(TK::Word.is(TokenCategory::PLAIN));
        assert!(TK::Punctuation.is(TokenCategory::PLAIN));
        assert!(!TK::Separator.is(TokenCategory::PLAIN));
    }

    #[test]
    fn literal_block_matches() {
        assert_eq!(TK::LiteralBlockMinimized.find_lexeme("e::\nt"), Some("::"));
    }

    #[test]
    fn literal_block_rejects_non_matching_text() {
        assert_eq!(TK::LiteralBlockMinimized.find_lexeme("e:\n"), None);
    }

    #[test]
    fn match_token_uses_centralized_token_list() {
        let (kind, lexeme) = TK::match_token("\nHello\n").unwrap();

        assert_eq!(kind, TK::Word);
        assert_eq!(lexeme, "Hello");
    }

    /// GIVEN an option followed by a new line
    /// WHEN the option token is matched
    /// THEN only the option on that line is returned
    #[test]
    fn option_does_not_consume_the_next_line() {
        assert_eq!(
            TK::Option.find_lexeme("\n--option\nempty item\n"),
            Some("--option")
        );
    }

    /// GIVEN a long option with digits in its name and argument
    /// WHEN the option token is matched
    /// THEN the complete option is returned
    #[test]
    fn option_matches_long_alphanumeric_name_and_argument() {
        assert_eq!(
            TK::Option.find_lexeme("\n--long1=arg1\n"),
            Some("--long1=arg1")
        );
    }

    #[test]
    fn option_matches_old_gnu_style() {
        assert_eq!(TK::Option.find_lexeme("\n+a  "), Some("+a"));
        assert_eq!(TK::Option.find_lexeme("\n+b file  "), Some("+b file"));
        assert_eq!(TK::Option.find_lexeme("\n+1 "), None);
    }

    #[test]
    fn punctuation_matches_ascii_non_alphanumeric() {
        assert_eq!(TK::Punctuation.find_lexeme("x,x"), Some(","));
        assert_eq!(TK::Punctuation.find_lexeme("x!x"), Some("!"));
        assert_eq!(TK::Punctuation.find_lexeme("x_x"), Some("_"));
    }

    #[test]
    fn punctuation_rejects_alphanumeric() {
        assert_eq!(TK::Punctuation.find_lexeme("xax"), None);
        assert_eq!(TK::Punctuation.find_lexeme("x1x"), None);
    }

    #[test]
    fn separator_matches() {
        assert_eq!(TK::Separator.find_lexeme("\n====\n"), Some("===="));
        assert_eq!(TK::Separator.find_lexeme("\n==a=\n"), None);
        assert_eq!(TK::Separator.find_lexeme("\n===\n"), None);
    }

    #[test]
    fn spaces_matches() {
        assert_eq!(TK::Spaces.find_lexeme("x \t x"), Some(" \t "));
    }

    #[test]
    fn spaces_rejects_non_matching_text() {
        assert_eq!(TK::Spaces.find_lexeme("xabcx"), None);
    }

    #[test]
    fn strong_start_and_end_match() {
        assert_eq!(TK::StrongStart.find_lexeme(" **x"), Some("**"));
        assert_eq!(TK::StrongEnd.find_lexeme("x** "), Some("**"));
        assert_eq!(TK::StrongStart.find_lexeme("*"), None);
        assert_eq!(TK::StrongEnd.find_lexeme("*"), None);
    }

    #[test]
    fn table_horizontal_matches() {
        assert_eq!(
            TK::TableHorizontal.find_lexeme("\n==== =====\n"),
            Some("==== =====")
        );
    }

    #[test]
    fn table_horizontal_rejects_non_matching_text() {
        assert_eq!(TK::TableHorizontal.find_lexeme("\n========\n"), None);
    }

    /// GIVEN a token with a multibyte lexeme
    /// WHEN its length is queried
    /// THEN the byte length is returned
    #[test]
    fn token_len_returns_lexeme_byte_length() {
        let token = Token::new(TK::Word, "é");

        assert_eq!(token.len(), 2);
    }

    #[test]
    fn word_matches_alphanumeric_and_underscore() {
        assert_eq!(TK::Word.find_lexeme(" alpha_123 "), Some("alpha_123"));
    }

    #[test]
    fn word_matches_with_newline_boundary() {
        assert_eq!(TK::Word.find_lexeme("\nalpha_123\n"), Some("alpha_123"));
    }

    #[test]
    fn word_rejects_non_word_chars() {
        assert_eq!(TK::Word.find_lexeme("---\n***"), None);
    }
}
