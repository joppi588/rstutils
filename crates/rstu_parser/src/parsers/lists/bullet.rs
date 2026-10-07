// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::{consume_trailing_blank_lines, parse_item_body};
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};
use std::debug_assert_matches;

pub(crate) fn parse_bullet_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::BulletListMarker);

    let list = AstNode::new_ref(NodeClass::BulletList);
    let mut marker: Option<String> = None;

    while stream.token_at_cursor().kind == TK::BulletListMarker {
        let item = AstNode::new_ref(NodeClass::BulletListItem);
        let marker_token = stream.consume();
        item.with_attr("marker", marker_token.lexeme.clone());
        if let Some(existing_marker) = &marker {
            if existing_marker != &marker_token.lexeme {
                return Err(ParserError::ListStyleError {
                    marker: existing_marker.clone(),
                    conflicting_marker: marker_token.lexeme,
                });
            }
        } else {
            marker = Some(marker_token.lexeme);
        }

        item.push_child(parse_item_body(stream, 1)?);
        list.push_child(item);
        consume_trailing_blank_lines(stream, &list);
    }

    Ok(list)
}
