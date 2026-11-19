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

pub(crate) fn parse_hyperlink_reference(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    stream.consume();
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
    match &stream.consume().kind {
        TK::AnonymousHyperlinkReferenceEnd => {
            node.with_attr("type", "ahref");
        }
        TK::HyperlinkReferenceEnd => {
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
