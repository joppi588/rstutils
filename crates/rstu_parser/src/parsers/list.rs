// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::block::parse_block_hanging_indent;
use crate::parser_errors::ParserError;
use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenSliceExt;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_bullet_list(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let list = AstNode::new_ref(NodeClass::BulletList);
    let mut index = start_at;
    let mut marker: Option<String> = None;

    while tokens.kind_at(index) == TK::BulletListMarker {
        let item = AstNode::new_ref(NodeClass::BulletListItem);
        item.with_attr("marker", tokens[index].lexeme.clone());
        let current_marker = tokens[index].lexeme.clone();
        if let Some(existing_marker) = &marker {
            if existing_marker != &current_marker {
                return Err(ParserError::ListStyleError {
                    marker: existing_marker.clone(),
                    conflicting_marker: current_marker,
                });
            }
        } else {
            marker = Some(current_marker.clone());
        }
        index += 1;
        if tokens.kind_at(index) == TK::Spaces {
            item.push_spaces(tokens[index].lexeme.len());
            index += 1;
        }

        let (block, new_index) =
            parse_block_hanging_indent(tokens, index).map_err(|_| ParserError::ListEndError {})?;
        index = new_index;
        item.push_child(block);
        list.push_child(item);
        if tokens.kind_at(index) == TK::BlankLine {
            list.push_blank_lines(tokens[index].lexeme.len());
            index += 1;
        }
    }

    Ok((list, index))
}

pub(crate) fn parse_field_list(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let list = AstNode::new_ref(NodeClass::FieldList);
    let mut index = start_at;

    while tokens.kind_at(index) == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_name = tokens[index]
            .lexeme
            .trim_start_matches(':')
            .trim_end_matches(':')
            .to_string();
        item.with_attr("fieldname", field_name);
        index += 1;

        if tokens.kind_at(index) == TK::Spaces {
            item.push_spaces(tokens[index].lexeme.len());
            index += 1;
        }
        let (block, new_index) =
            parse_block_hanging_indent(tokens, index).map_err(|_| ParserError::ListEndError {})?;
        item.push_child(block);
        index = new_index;
        list.push_child(item);
        if tokens.kind_at(index) == TK::BlankLine {
            list.push_blank_lines(tokens[index].lexeme.len());
            index += 1;
        }
    }

    Ok((list, index))
}
