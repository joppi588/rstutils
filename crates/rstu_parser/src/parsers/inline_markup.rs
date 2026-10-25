// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

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
            node.with_attr("text", &lexeme[1..lexeme.len() - 2])
                .with_attr("type", "citation");
        }
        TK::FootnoteReference => {
            node.with_attr("text", &lexeme[1..lexeme.len() - 2])
                .with_attr("type", "footnote");
        }
        TK::SubstitutionReference => {
            node.with_attr("text", &lexeme[1..lexeme.len() - 1])
                .with_attr("type", "sub");
        }
        TK::SimpleHyperlinkReference => {
            node.with_attr("text", &lexeme[0..lexeme.len() - 1])
                .with_attr("type", "simple_ref");
        }
        TK::SimpleAnonymousHyperLinkReference => {
            node.with_attr("text", &lexeme[0..lexeme.len() - 2])
                .with_attr("type", "simple_anonymous_ref");
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
            &[TK::BackquoteEnd, TK::HyperlinkReferenceEnd],
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
        (TK::BackquoteStart, TK::HyperlinkReferenceEnd) => "hyperlink_reference",
        (TK::BackquoteStart, TK::BackquoteEnd) => "interpreted_text",
        _ => markup,
    };

    let inline = AstNode::new_ref(NodeClass::InlineMarkup);
    inline
        .with_attr("markup", effective_markup)
        .with_attr("text", text);
    Ok(inline)
}
