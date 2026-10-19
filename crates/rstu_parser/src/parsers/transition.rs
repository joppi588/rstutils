// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_transition(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_eq!(stream.token_at_cursor().kind, TK::Separator);
    let marker = stream.consume();
    let transition = AstNode::new_ref(NodeClass::Transition);
    transition
        .with_attr(
            "marker_type",
            marker.lexeme.chars().next().unwrap().to_string(),
        )
        .with_attr("marker_len", marker.lexeme.chars().count());
    stream.consume_newline();
    Ok(transition)
}
