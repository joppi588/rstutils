// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::block::parse_block_hanging_indent;
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_bullet_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let list = AstNode::new_ref(NodeClass::BulletList);
    let mut marker: Option<String> = None;

    while stream.kind_at_cursor() == TK::BulletListMarker {
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
        if stream.kind_at_cursor() == TK::Spaces {
            let spaces_token = stream.consume();
            item.push_spaces(spaces_token.lexeme.len());
        }

        let block = parse_block_hanging_indent(stream).map_err(|_| ParserError::ListEndError {})?;
        item.push_child(block);
        list.push_child(item);
        if stream.kind_at_cursor() == TK::BlankLine {
            let blank_token = stream.consume();
            list.push_blank_lines(blank_token.lexeme.len());
        }
    }

    Ok(list)
}

pub(crate) fn parse_field_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let list = AstNode::new_ref(NodeClass::FieldList);

    while stream.kind_at_cursor() == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_token = stream.consume();
        let field_name = field_token
            .lexeme
            .trim_start_matches(':')
            .trim_end_matches(':')
            .to_string();
        item.with_attr("fieldname", field_name);

        if stream.kind_at_cursor() == TK::Spaces {
            let spaces_token = stream.consume();
            item.push_spaces(spaces_token.lexeme.len());
        }
        let block = parse_block_hanging_indent(stream).map_err(|_| ParserError::ListEndError {})?;
        item.push_child(block);
        list.push_child(item);
        if stream.kind_at_cursor() == TK::BlankLine {
            let blank_token = stream.consume();
            list.push_blank_lines(blank_token.lexeme.len());
        }
    }

    Ok(list)
}
