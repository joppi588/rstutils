// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;
use regex::{Captures, Regex};
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};
use std::sync::LazyLock;

static EMBEDDED_REF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?s)^(?P<label>.*?)(?:(?:(?P<separator>\s+)|^)<(?P<ref>[^<>\s](?:[^<>]*[^<>\s])?)>)?$",
    )
    .unwrap()
});

static REFERENCE_ESCAPE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\(?s:(.))|\\$").unwrap());
static URI_NORMALIZATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\(?P<escaped>(?s:.))|\\$|\s").unwrap());

pub(crate) fn normalize_reference_name(reference: &str) -> String {
    REFERENCE_ESCAPE
        .replace_all(reference, "$1")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_uri(uri: &str) -> String {
    URI_NORMALIZATION
        .replace_all(uri, |captures: &Captures<'_>| {
            match captures.name("escaped") {
                Some(escaped) if escaped.as_str().chars().next().unwrap().is_whitespace() => {
                    " ".to_owned()
                }
                Some(escaped) => escaped.as_str().to_owned(),
                None => String::new(),
            }
        })
        .into_owned()
}

pub(crate) fn parse_reference_token(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind.is(TC::INLINE_REFERENCE));

    let node = AstNode::new_ref(NodeClass::Reference);
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
                .with_attr("type", "simple_href");
        }
        TK::SimpleAnonymousHyperLinkReference => {
            node.with_attr("label", &lexeme[0..lexeme.len() - 2])
                .with_attr("type", "simple_ahref");
        }
        TK::Uri => {
            let uri = lexeme.replace("\\*", "*").replace("\\_", "_");
            node.with_attr("refuri", uri.as_str())
                .with_attr("raw", lexeme.as_str());
        }
        TK::Email => {
            node.with_attr("refuri", format!("mailto:{lexeme}"));
        }
        _ => unreachable!(),
    };
    Ok(node)
}

pub(crate) fn parse_reference_expression(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_eq!(stream.token_at_cursor().kind, TK::BackquoteStart);
    stream.consume(); // consume backquote
    let text = &stream.consume_text_until(
        &[
            &[
                TK::AnonymousHyperlinkReferenceEnd,
                TK::HyperlinkReferenceEnd,
            ],
            TC::PARAGRAPH_END,
        ]
        .concat(),
    );
    let node = AstNode::new_ref(NodeClass::Reference);
    match &stream.token_at_cursor().kind {
        TK::AnonymousHyperlinkReferenceEnd => {
            stream.consume();
            node.with_attr("type", "ahref");
        }
        TK::HyperlinkReferenceEnd => {
            stream.consume();
            node.with_attr("type", "href");
        }
        kind if kind.is(TC::PARAGRAPH_END) => {}
        _ => unreachable!(),
    };
    let captures = EMBEDDED_REF
        .captures(&text)
        .expect("There must be some match");

    if let Some(label) = captures
        .name("label")
        .filter(|label| !label.as_str().is_empty() || captures.name("ref").is_none())
    {
        node.with_attr("label", label.as_str());
    };

    let refname = captures
        .name("ref")
        .map_or(None, |value| Some(value.as_str()));
    match refname {
        Some(refname) if refname.ends_with("_") && !refname.ends_with(r"\_") => {
            let normalized_refname = normalize_reference_name(refname.strip_suffix('_').unwrap());
            node.with_attr("refname", normalized_refname)
                .with_attr("raw", text.to_owned());
        }
        Some(refuri) => {
            let normalized_refuri = normalize_uri(refuri);
            node.with_attr("refuri", normalized_refuri)
                .with_attr("raw", text.to_owned());
        }
        _ => {}
    };
    Ok(node)
}

pub(crate) fn parse_reference_plain(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let text = stream.consume_text_until(TC::PARAGRAPH_END);
    let node = AstNode::new_ref(NodeClass::Reference);
    node.with_attr("refname", normalize_reference_name(&text))
        .with_attr("raw", text);
    Ok(node)
}
