// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::parsers::hyperlink_reference::{normalize_reference_name, parse_hyperlink_reference};
use crate::token::TokenCategory as TC;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use crate::with_hanging_indent;

pub(crate) fn parse_inline_target(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::InlineInternalTargetStart);

    let position = stream.position_at_cursor();
    stream.consume(); // inline target marker

    let text = stream.consume_text_until(&[&[TK::BackquoteEnd], TC::PARAGRAPH_END].concat());
    if stream.token_at_cursor().kind != TK::BackquoteEnd {
        return Err(ParserError::TargetMissingClosing {
            markup: "target".to_owned(),
            position,
        });
    }
    stream.consume();
    let target = AstNode::new_ref(NodeClass::Target)
        .with_attr("markup", "inline")
        .with_attr("refname", text);
    Ok(target)
}

pub(crate) fn parse_hyperlink_target(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let marker = stream.consume();
    debug_assert!(matches!(
        marker.kind,
        TK::HyperlinkTarget | TK::HyperlinkTargetAnonymous | TK::HyperlinkTargetAnonymousShort
    ));

    let target = AstNode::new_ref(NodeClass::Target);
    match marker.kind {
        TK::HyperlinkTargetAnonymous => {
            target.with_attr("markup", "standalone_anonymous");
        }
        TK::HyperlinkTargetAnonymousShort => {
            target.with_attr("markup", "standalone_anonymous_short");
        }
        _ => {
            let label = marker
                .lexeme
                .strip_prefix(".. _")
                .and_then(|label| label.strip_suffix(':'))
                .expect("There is a label.");
            target
                .with_attr(
                    "label",
                    normalize_reference_name(label.trim().trim_matches('`')),
                )
                .with_attr("label_raw", label);
        }
    }
    match (
        stream.token_at_cursor().kind,
        stream.token_at_nextline().kind,
    ) {
        (TK::NewLine | TK::Spaces, TK::Indent) => {
            target.push_child(
                AstNode::new_ref(NodeClass::PlainText).with_attr("text", stream.consume().lexeme),
            );
            let link_block = with_hanging_indent(stream, parse_hyperlink_reference)?;
            target.push_child(link_block);
        }
        (_, TK::Indent) => {
            let link_block = with_hanging_indent(stream, parse_hyperlink_reference)?;
            target.push_child(link_block);
        }
        (TK::NewLine, kind) if kind.is(TC::PARAGRAPH_END) => {
            stream.consume();
        }
        (TK::Spaces, kind) if kind.is(TC::PARAGRAPH_END) => {
            target.push_child(
                AstNode::new_ref(NodeClass::PlainText).with_attr("text", stream.consume().lexeme),
            );
            let link_block = parse_hyperlink_reference(stream)?;
            target.push_child(link_block);
        }
        _ => {}
    }
    Ok(target)
}
