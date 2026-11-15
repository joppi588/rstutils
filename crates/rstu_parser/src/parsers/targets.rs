// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;

use crate::token::TokenCategory as TC;

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
    stream.consume();
    let target = AstNode::new_ref(NodeClass::Target)
        .with_attr("target_type", "inline")
        .with_attr("label", text);
    Ok(target)
}
