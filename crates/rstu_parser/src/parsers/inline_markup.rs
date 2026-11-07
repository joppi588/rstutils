// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use regex::Regex;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};
use std::sync::LazyLock;

static EMBEDDED_URI_NORMALIZATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\(.)|[ \n]").unwrap());
static EMBEDDED_ALIAS_NORMALIZATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\(.)").unwrap());
static EMBEDDED_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)^(?P<before>.*\s)?<(?P<ref>[^<>\s](?:[^<>]*[^<>\s])?)>$").unwrap()
});

use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;

pub(crate) fn parse_inline_token(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind.is(TC::INLINE_TOKEN));

    let node = AstNode::new_ref(NodeClass::Reference);
    let position = stream.position_at_cursor();
    let token = stream.consume();
    let kind = token.kind;
    let lexeme = &token.lexeme;
    match kind {
        TK::CitationReference => {
            node.with_attr("label", &lexeme[1..lexeme.len() - 2])
                .with_attr("type", "citation");
        }
        TK::FootnoteReference => {
            node.with_attr("label", &lexeme[1..lexeme.len() - 2])
                .with_attr("type", "footnote");
        }
        TK::SubstitutionReference => {
            node.with_attr("label", &lexeme[1..lexeme.len() - 1])
                .with_attr("type", "sub_ref");
        }
        TK::SubRefHyperlink => {
            node.with_attr("label", &lexeme[1..lexeme.len() - 2])
                .with_attr("type", "sub_href");
        }
        TK::SubRefAnonymousHyperlink => {
            node.with_attr("label", &lexeme[1..lexeme.len() - 3])
                .with_attr("type", "sub_ahref");
        }
        TK::SimpleHyperlinkReference => {
            node.with_attr("label", &lexeme[0..lexeme.len() - 1])
                .with_attr("type", "simple_ref");
        }
        TK::SimpleAnonymousHyperLinkReference => {
            node.with_attr("label", &lexeme[0..lexeme.len() - 2])
                .with_attr("type", "simple_anonymous_ref");
        }
        TK::Uri => {
            let uri = lexeme.replace("\\*", "*").replace("\\_", "_");
            node.with_attr("refuri", uri.as_str());
            let text = AstNode::new_ref(NodeClass::PlainText);
            text.with_attr("text", uri);
            node.push_child(text);
        }
        TK::Email => {
            node.with_attr("refuri", format!("mailto:{lexeme}"));
            let text = AstNode::new_ref(NodeClass::PlainText);
            text.with_attr("text", lexeme.as_str());
            node.push_child(text);
        }

        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "Reference token".to_owned(),
                found: format!("{:?}", kind),
                position,
            });
        }
    };
    Ok(node)
}

pub(crate) fn parse_inline(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind.is(TC::INLINE_MARKER));

    let position = stream.position_at_cursor();
    let kind = stream.consume().kind;
    let (markup, end_kind_candidates): (&str, &[TK]) = match kind {
        TK::StrongStart => ("strong", &[TK::StrongEnd]),
        TK::EmphasisStart => ("emphasis", &[TK::EmphasisEnd]),
        TK::InlineLiteralStart => ("inline_literal", &[TK::InlineLiteralEnd]),
        TK::InlineInternalTargetStart => ("inline_internal_target", &[TK::BackquoteEnd]),
        TK::BackquoteStart => (
            "interpreted_or_hyperlink",
            &[
                TK::BackquoteEnd,
                TK::AnonymousHyperlinkReferenceEnd,
                TK::HyperlinkReferenceEnd,
            ],
        ),
        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "Inline start token".to_owned(),
                found: format!("{:?}", kind),
                position,
            });
        }
    };

    let text = stream.consume_text_until(&[end_kind_candidates, TC::PARAGRAPH_END].concat());
    if !stream.token_at_cursor().kind.is(end_kind_candidates) {
        return Err(ParserError::InlineMissingClosing {
            markup: markup.to_owned(),
            position,
        });
    }

    let effective_markup = match (kind, stream.consume().kind) {
        (TK::BackquoteStart, TK::HyperlinkReferenceEnd) => "href",
        (TK::BackquoteStart, TK::AnonymousHyperlinkReferenceEnd) => "ahref",
        (TK::BackquoteStart, TK::BackquoteEnd) => "interpreted_text",
        _ => markup,
    };

    let inline = match effective_markup {
        "inline_literal" => AstNode::new_ref(NodeClass::Literal),
        _ => AstNode::new_ref(NodeClass::InlineMarkup).with_attr("markup", effective_markup),
    };

    let embedded_ref = if matches!(effective_markup, "href" | "ahref") {
        EMBEDDED_REF.captures(&text).map(|captures| {
            let before = captures.name("before").map_or("", |value| value.as_str());
            let reference = captures.name("ref").unwrap().as_str();
            (before, reference)
        })
    } else {
        None
    };
    match embedded_ref {
        Some((before, refname)) if refname.ends_with("_") && !refname.ends_with(r"\_") => {
            let trimmed = before.trim_end();
            let normalized_refname = EMBEDDED_ALIAS_NORMALIZATION
                .replace_all(refname.strip_suffix('_').unwrap(), "$1")
                .into_owned()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            let label = if trimmed.is_empty() {
                &normalized_refname
            } else {
                trimmed
            };
            inline
                .with_attr("raw", text.clone())
                .with_attr("label", label)
                .with_attr("refname", normalized_refname);
        }
        Some((before, refuri)) => {
            let trimmed = before.trim_end();
            let normalized_refuri = EMBEDDED_URI_NORMALIZATION
                .replace_all(refuri, "$1")
                .into_owned();
            let label = if trimmed.is_empty() {
                &normalized_refuri
            } else {
                trimmed
            };
            inline
                .with_attr("raw", text.clone())
                .with_attr("label", label)
                .with_attr("refuri", normalized_refuri);
        }
        _ => {
            inline.with_attr("text", text);
        }
    }
    Ok(inline)
}
