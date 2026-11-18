// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::parsers::hyperlink_reference::{normalize_reference_name, parse_hyperlink_reference};
use crate::token::TokenCategory as TC;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
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
    let text = stream.consume_text_until(TC::PARAGRAPH_END);
    if text != "\n" {
        let link_block = parse_hyperlink_reference(&text)?;
        target.push_child(link_block);
    } // TODO: does the paragraph parsing parse correctly (internal = empty, external, indirect)
    Ok(target)
}
