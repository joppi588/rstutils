// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use regex::Regex;
use std::sync::LazyLock;
use strum::EnumCount;

static RECOMMENDED_SECTION_CHARS: &str = "=\\-`:.'\"~\\^_\\*\\+#"; // escaped =-`:.'"~^_*+#
static FOOTNOTE_LABEL: &str = r"(?:[0-9]+|#(?:[\p{L}\p{N}]+(?:[-_.][\p{L}\p{N}]+)*)?|\*)";
static INLINE_PRE_CHARS: &str = r#"(?:[\n\s\-:/'"<(\[{]|\p{Ps}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po})"#;
static INLINE_PRE_CHARS_NO_ASTERISK: &str =
    r#"(?:[\n\s\-:/'"<(\[{]|\p{Ps}|\p{Pi}|\p{Pf}|\p{Pd}|[\p{Po}&&[^*]])"#;
static INLINE_PRE_CHARS_NO_FINAL_QUOTE: &str =
    r#"(?:[\n\s\-:/'"<(\[{]|\p{Ps}|\p{Pi}|\p{Pd}|[\p{Po}&&[^*]])"#;
static INLINE_POST_CHARS: &str =
    r#"(?:[\n\s\-\.,:;!?\\/'")\]}>]|\p{Pe}|\p{Pi}|\p{Pf}|\p{Pd}|\p{Po}|\p{Ps})"#;
static INLINE_POST_CHARS_NO_ASTERISK: &str =
    r#"(?:[\n\s\-\.,:;!?\\/'")\]}>]|\p{Pe}|\p{Pi}|\p{Pf}|\p{Pd}|[\p{Po}&&[^*]]|\p{Ps})"#;

fn enclosed_inline_start_pattern() -> String {
    let starts = [r"\*\*", r"\*", r"``", r"_`", r"`", r"\|"].join("|");
    let starts = format!("(?:{starts})");
    let bracket_pairs = [
        (0x0028, 0x0029),
        (0x003C, 0x003E),
        (0x005B, 0x005D),
        (0x007B, 0x007D),
        (0x0F3A, 0x0F3B),
        (0x0F3C, 0x0F3D),
        (0x169B, 0x169C),
        (0x2045, 0x2046),
        (0x207D, 0x207E),
        (0x208D, 0x208E),
        (0x2308, 0x2309),
        (0x230A, 0x230B),
        (0x2329, 0x232A),
        (0x2768, 0x2769),
        (0x276A, 0x276B),
        (0x276C, 0x276D),
        (0x276E, 0x276F),
        (0x2770, 0x2771),
        (0x2772, 0x2773),
        (0x2774, 0x2775),
        (0x27C5, 0x27C6),
        (0x27E6, 0x27E7),
        (0x27E8, 0x27E9),
        (0x27EA, 0x27EB),
        (0x27EC, 0x27ED),
        (0x27EE, 0x27EF),
        (0x2983, 0x2984),
        (0x2985, 0x2986),
        (0x2987, 0x2988),
        (0x2989, 0x298A),
        (0x298B, 0x298C),
        (0x298D, 0x2990),
        (0x298F, 0x298E),
        (0x2991, 0x2992),
        (0x2993, 0x2994),
        (0x2995, 0x2996),
        (0x2997, 0x2998),
        (0x29D8, 0x29D9),
        (0x29DA, 0x29DB),
        (0x29FC, 0x29FD),
        (0x2E22, 0x2E23),
        (0x2E24, 0x2E25),
        (0x2E26, 0x2E27),
        (0x2E28, 0x2E29),
        (0x2E55, 0x2E56),
        (0x2E57, 0x2E58),
        (0x2E59, 0x2E5A),
        (0x2E5B, 0x2E5C),
        (0x3008, 0x3009),
        (0x300A, 0x300B),
        (0x300C, 0x300D),
        (0x300E, 0x300F),
        (0x3010, 0x3011),
        (0x3014, 0x3015),
        (0x3016, 0x3017),
        (0x3018, 0x3019),
        (0x301A, 0x301B),
        (0xFE59, 0xFE5A),
        (0xFE5B, 0xFE5C),
        (0xFE5D, 0xFE5E),
        (0xFF08, 0xFF09),
        (0xFF3B, 0xFF3D),
        (0xFF5B, 0xFF5D),
        (0xFF5F, 0xFF60),
        (0xFF62, 0xFF63),
    ];
    let bracket_patterns = bracket_pairs
        .iter()
        .map(|(open, close)| format!(r"\x{{{open:X}}}{starts}\x{{{close:X}}}"));
    let quote_chars = [
        '\'', '"', '\u{00AB}', '\u{00BB}', '\u{2039}', '\u{203A}', '\u{2E42}', '\u{FF02}',
        '\u{FF07}',
    ]
    .into_iter()
    .chain('\u{2018}'..='\u{201F}')
    .chain('\u{300C}'..='\u{300F}')
    .chain('\u{301D}'..='\u{301F}')
    .chain('\u{FE41}'..='\u{FE44}')
    .chain(['\u{FF62}', '\u{FF63}'])
    .collect::<Vec<_>>();
    let mut quote_patterns = Vec::new();
    for open in quote_chars.iter().copied() {
        for close in quote_chars.iter().copied() {
            if (open, close) != ('\u{201E}', '\u{2019}')
                && (open, close) != ('\u{00BB}', '\u{2039}')
            {
                quote_patterns.push(format!(
                    r"\x{{{:X}}}{starts}\x{{{:X}}}",
                    open as u32, close as u32
                ));
            }
        }
    }
    let enclosed_patterns = bracket_patterns
        .chain(quote_patterns)
        .collect::<Vec<_>>()
        .join("|");

    format!(r"(.|\n)(?:{enclosed_patterns})(.|\n)")
}

// Option parsing, Lexer:
static OPTION: &str = concat!(
    r"\n(?:",
    r"[\-\+][a-z](?:[ \t][a-z]+)?", // short and old GNU (+)
    "|",
    r"\-\-[a-z][a-z0-9]*(?:[\-_][a-z][a-z0-9]*)*(?:[ \t=][a-z][a-z0-9]*)?", // long
    "|",
    r"/[A-Z]+(?:[ \t=][a-z]+)?", // DOS
    r")[ ,\n]"
);
// Option parsing, Parser:
// Regexes with match groups are only used in the parser, but not the lexer (performance)
pub(crate) static SHORT_OPTIONS_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s?([\-\+][a-z]{1})([ \t=]?)(.*)$").unwrap());
pub(crate) static LONG_OPTIONS_MATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s?(\-\-[a-z][a-z0-9]*(?:[\-_][a-z][a-z0-9]*)*)([ \t=]?)(.*)$").unwrap()
});
pub(crate) static DOS_OPTIONS_MATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s?(/[A-Z]+)([ \t=]?)([a-z]*)$").unwrap());

macro_rules! compiled_regex {
    ($pattern:expr) => {{
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(format!(r"^{}", $pattern).as_ref()).unwrap());
        &RE
    }};
}
macro_rules! token_kinds {
    ($(($kind:ident, $pattern:expr)),+ $(,)?) => {
        pub const ALL: [TokenKind; <TokenKind as strum::EnumCount>::COUNT] = [
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
        TokenKind::CitationReference,
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
        TokenKind::FootNote,
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
        TokenKind::CitationReference,
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
        TokenKind::FootNote,
        TokenKind::Indent,
        TokenKind::LineBlock,
        TokenKind::LiteralBlock,
        TokenKind::LiteralBlockMinimized,
        TokenKind::LiteralBlockPartiallyMinimized,
        TokenKind::Separator,
    ];
    pub const PLAIN: &'static [TokenKind] = &[
        TokenKind::DoctestStart,
        TokenKind::EnclosedInlineStart,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
pub enum TokenKind {
    Attribution,
    BackquoteEnd,
    BackquoteStart,
    BlankLine,
    BulletListMarker,
    ClassifierSeparator,
    CitationReference,
    Dedent,
    Directive,
    DoctestStart,
    DoubleDot,
    EmphasisEnd,
    EmphasisStart,
    EnclosedInlineStart,
    EnumeratedListMarker,
    EoF,
    EscapedChar,
    Field,
    FootNote,
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
        (Separator, format!(r"\s[{0}]{{4,}}\n", RECOMMENDED_SECTION_CHARS)),
        (Attribution, r" (?:—|--|---) +(?:.|\n)"),

        (Indent, r"\n[ \t]+[^ \t\n]"),
        (BlankLine, r"\n[ \t]*\n+(.|\n)"),
        (NewLine, r"[^\n]\n(.|\n)"),

        (Directive, r"[\n\s]\.\.\s(?:\|[^|\n]+\|\s)?[\p{L}\p{N}]+(?:[-_+:.][\p{L}\p{N}]+)*::\s"),

        (DoctestStart, r"\s>>>\s"),

        (FootNote, format!(r"(.|\n)(?:\.\. \[{0}\])(.|\n)", FOOTNOTE_LABEL)),

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
        (EmphasisStart, format!(r"(?:{0}\*[^\s]|\p{{Pf}}\*[^\s\p{{Pi}}])", INLINE_PRE_CHARS_NO_FINAL_QUOTE)),
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
        (FootnoteReference, format!(r"{0}\[{2}\]_{1}", INLINE_PRE_CHARS, INLINE_POST_CHARS, FOOTNOTE_LABEL)),
        (CitationReference, format!(r"{0}\[[\p{{L}}\p{{N}}]+(?:[-_.][\p{{L}}\p{{N}}]+)*\]_{1}", INLINE_PRE_CHARS, INLINE_POST_CHARS)),
        (HyperlinkReferenceEnd, format!(r"(?:[^\s]`_|[^\s]_){}", INLINE_POST_CHARS)),
        (SimpleAnonymousHyperLinkReference,r"[\W_][\w&&[^_]]+(?:[-_.:+][\w&&[^_]]+)*__\s"),
        (SimpleHyperlinkReference,r"[\W_][\w&&[^_]]+(?:[-_.:+][\w&&[^_]]+)*_\s"),
        (EnclosedInlineStart, enclosed_inline_start_pattern()),

        // Classifiers for definition list (must precede Field: " : x :" would match Field)
        (ClassifierSeparator,r"(.|\n)\s+:\s+(.|\n)"),

        // Lists
        (Field,r"\s:[[:word:] ]+:\s"),
        (EnumeratedListMarker, r"[\n\s](?:(?:#|[0-9]+|[A-Za-z]+|[IVXLCDMivxlcdm]+)(?:\.|\))[ \t]|\([A-Za-z0-9]+\)[ \t])"),
        (BulletListMarker, r"(\s|\n)[\-\+\*•‣⁃](\s|\n)"),
        (Option, OPTION),

        // Plain text
        (Spaces, r"[^ \t\n][ \t]+[^ \t]"),
        (Word, r"[^\w][\w&&[^_]]\w*[^\w]"),
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

    /// GIVEN an inline start delimiter immediately enclosed by matching brackets or quotes
    /// WHEN the enclosed-start token is matched
    /// THEN the entire enclosed delimiter is classified as plain text
    #[test]
    fn enclosed_inline_start_matches_matching_pairs() {
        for input in [" (*) ", " <**> ", " [``] ", " ⁅*⁆ ", " “*„ ", " '|' "] {
            assert!(
                TK::EnclosedInlineStart.find_lexeme(input).is_some(),
                "expected an enclosed start in {input:?}"
            );
        }
    }

    /// GIVEN mismatched brackets or a start not immediately followed by a closer
    /// WHEN the enclosed-start token is matched
    /// THEN it does not claim the text
    #[test]
    fn enclosed_inline_start_rejects_mismatches_and_nonadjacent_closers() {
        for input in [" (*] ", " (*x*) ", " ( * ) ", " „*’ ", " »*‹ "] {
            assert_eq!(
                TK::EnclosedInlineStart.find_lexeme(input),
                None,
                "{input:?}"
            );
        }
        assert_eq!(TK::EmphasisStart.find_lexeme("„*’ "), Some("*"));
        assert_eq!(TK::EmphasisEnd.find_lexeme("»*‹ "), Some("*"));
        assert_eq!(TK::EmphasisStart.find_lexeme("»*‹ "), None);
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
    fn footnote_reference_subtypes_match_and_belong_to_their_categories() {
        for (input, expected_kind, expected_lexeme) in [
            (" [1]_ ", TK::FootnoteReference, "[1]_"),
            (" [#]_ ", TK::FootnoteReference, "[#]_"),
            (" [#label]_ ", TK::FootnoteReference, "[#label]_"),
            (" [*]_ ", TK::FootnoteReference, "[*]_"),
        ] {
            let (kind, lexeme) = TK::match_token(input).unwrap();

            assert_eq!(kind, expected_kind);
            assert_eq!(lexeme, expected_lexeme);
            assert!(kind.is(TokenCategory::INLINE_TOKEN));
            assert!(kind.is(TokenCategory::DEFINITION_LIST));
        }
    }

    #[test]
    fn other_inline_references_match() {
        assert_eq!(TK::SubstitutionReference.find_lexeme(" |x| "), Some("|x|"));
        assert_eq!(TK::HyperlinkReferenceEnd.find_lexeme("x_ "), Some("_"));
        assert_eq!(TK::HyperlinkReferenceEnd.find_lexeme("x`_ "), Some("`_"));
        assert_eq!(TK::FootnoteReference.find_lexeme(" [citation]_ "), None);
        assert_eq!(
            TK::CitationReference.find_lexeme(" [citation]_ "),
            Some("[citation]_")
        );
        assert_eq!(
            TK::CitationReference.find_lexeme(" [cit-ation]_ "),
            Some("[cit-ation]_")
        );
        assert_eq!(
            TK::CitationReference.find_lexeme(" [cit.ation]_ "),
            Some("[cit.ation]_")
        );
        assert_eq!(
            TK::SimpleHyperlinkReference.find_lexeme(" simple_ref_ text"),
            Some("simple_ref_")
        );
        assert_eq!(
            TK::SimpleAnonymousHyperLinkReference.find_lexeme("_This__ "),
            Some("This__")
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
        assert_eq!(TK::Word.find_lexeme(" _alpha "), None);
    }
}
