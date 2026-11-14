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
    Regex::new(
        r"(?s)^(?P<label>.*?)(?:(?:(?P<separator>\s+)|^)<(?P<ref>[^<>\s](?:[^<>]*[^<>\s])?)>)?$",
    )
    .unwrap()
});

use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;

pub(crate) fn parse_inline_reference(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
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
        _ => unreachable!(),
    };
    Ok(node)
}

pub(crate) fn parse_interpreted_or_ref(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::BackquoteStart);

    let position = stream.position_at_cursor();
    stream.consume();
    let end_kind_candidates = &[
        TK::BackquoteEnd,
        TK::RolePostfix,
        TK::AnonymousHyperlinkReferenceEnd,
        TK::HyperlinkReferenceEnd,
    ];
    let text = stream.consume_text_until(&[end_kind_candidates, TC::PARAGRAPH_END].concat());
    let end_token = stream.consume();
    let node = match end_token.kind {
        TK::AnonymousHyperlinkReferenceEnd => parse_hyperlink_reference(&text, "ahref")?,
        TK::HyperlinkReferenceEnd => parse_hyperlink_reference(&text, "href")?,
        TK::BackquoteEnd => AstNode::new_ref(NodeClass::InterpretedText).with_attr("text", text),
        TK::RolePostfix => {
            let lexeme = &end_token.lexeme;
            AstNode::new_ref(NodeClass::InterpretedText)
                .with_attr("text", text)
                .with_attr("role", &lexeme[2..lexeme.len() - 1])
                .with_attr("pos", "postfix".to_owned())
        }
        _ => {
            return Err(ParserError::InlineMissingClosing {
                markup: "interpreted text or reference".to_owned(),
                position,
            });
        }
    };
    Ok(node)
}

pub(crate) fn parse_hyperlink_reference(
    text: &str,
    ref_type: &str,
) -> Result<NodeRef, ParserError> {
    let node = AstNode::new_ref(NodeClass::Reference).with_attr("type", ref_type);
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
            let normalized_refname = EMBEDDED_ALIAS_NORMALIZATION
                .replace_all(refname.strip_suffix('_').unwrap(), "$1")
                .into_owned()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            node.with_attr("refname", normalized_refname)
                .with_attr("raw", text);
        }
        Some(refuri) => {
            let normalized_refuri = EMBEDDED_URI_NORMALIZATION
                .replace_all(refuri, "$1")
                .into_owned();
            node.with_attr("refuri", normalized_refuri)
                .with_attr("raw", text);
        }
        _ => {}
    };
    Ok(node)
}

pub(crate) fn parse_prefix_role(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::RolePrefix);

    let position = stream.position_at_cursor();
    let token = stream.consume();
    let text = stream
        .consume_text_until(&[&[TK::BackquoteEnd, TK::RolePostfix], TC::PARAGRAPH_END].concat());
    let end_kind = stream.consume().kind;

    if end_kind == TK::RolePostfix {
        return Err(ParserError::RolePrefixAndPostfix { position });
    }
    if end_kind != TK::BackquoteEnd {
        return Err(ParserError::InlineMissingClosing {
            markup: format!("{:?}", token.kind),
            position,
        });
    }

    let role = &token.lexeme;
    Ok(AstNode::new_ref(NodeClass::InterpretedText)
        .with_attr("role", &role[1..role.len() - 2])
        .with_attr("pos", "prefix".to_owned())
        .with_attr("text", text))
}

pub(crate) fn parse_inline_markup(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind.is(TC::INLINE_MARKUP));

    let position = stream.position_at_cursor();
    let token = stream.consume();
    let expected_end_kind: &[TK] = match token.kind {
        TK::StrongStart => &[TK::StrongEnd],
        TK::EmphasisStart => &[TK::EmphasisEnd],
        TK::InlineLiteralStart => &[TK::InlineLiteralEnd],
        _ => unreachable!(),
    };

    let text = stream.consume_text_until(&[expected_end_kind, TC::PARAGRAPH_END].concat());
    let end_kind = stream.consume().kind;

    if !end_kind.is(expected_end_kind) {
        return Err(ParserError::InlineMissingClosing {
            markup: format!("{:?}", token.kind),
            position,
        });
    }

    let inline = match token.kind {
        TK::EmphasisStart => AstNode::new_ref(NodeClass::Emphasis),
        TK::StrongStart => AstNode::new_ref(NodeClass::Strong),
        TK::InlineLiteralStart => AstNode::new_ref(NodeClass::Literal),
        _ => unreachable!(),
    };

    inline.with_attr("text", text);
    Ok(inline)
}

pub(crate) fn parse_inline_target(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::InlineInternalTargetStart);

    let position = stream.position_at_cursor();
    stream.consume(); // inline target marker

    let text = stream.consume_text_until(&[&[TK::BackquoteEnd], TC::PARAGRAPH_END].concat());
    if stream.token_at_cursor().kind != TK::BackquoteEnd {
        return Err(ParserError::InlineMissingClosing {
            markup: "target".to_owned(),
            position,
        });
    }
    let target = AstNode::new_ref(NodeClass::Target).with_attr("label", text);
    Ok(target)
}
