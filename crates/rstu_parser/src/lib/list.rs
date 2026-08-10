// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::paragraph;
use crate::parser_errors::FindElementError;
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::skip_kinds;

pub(crate) fn try_parse_bullet_list(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let list = AstNode::new_ref(NodeClass::BulletList);
    let mut index = start_at;
    let mut marker: Option<String> = None;

    // TODO
    // list starts by blankline + (optional indent) + Bullet
    // List ends by blankline+dedent
    // Wrong indent causes error
    while index < tokens.len() && tokens[index].kind == TK::BulletListMarker {
        let item = AstNode::new_ref(NodeClass::BulletListItem);
        item.with_attr("marker", tokens[index].lexeme.clone());
        let current_marker = tokens[index].lexeme.clone();
        if let Some(existing_marker) = &marker {
            if existing_marker != &current_marker {
                return Err(FindElementError::ListStyleError {
                    marker: existing_marker.clone(),
                    conflicting_marker: current_marker,
                });
            }
        } else {
            marker = Some(current_marker.clone());
        }

        let (paragraph, new_index) = paragraph::try_parse_paragraph(tokens, index + 2)?;
        index = new_index;
        item.push_child(paragraph);
        if tokens[index].kind == TK::BlankLine {
            index = skip_kinds(tokens, &[TK::BlankLine], index);
            item.push_child(AstNode::new_ref(NodeClass::BlankLine))
        }
        list.push_child(item);
    }

    Ok((list, index))
}

pub(crate) fn try_parse_field_list(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let list = AstNode::new_ref(NodeClass::FieldList);
    let mut index = start_at;

    while index < tokens.len() && tokens[index].kind == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_name = tokens[index]
            .lexeme
            .trim_start_matches(':')
            .trim_end_matches(':')
            .to_string();
        item.with_attr("fieldname", field_name);

        let (paragraph, next_index) = paragraph::try_parse_paragraph(tokens, index + 2)?;
        item.push_child(paragraph);
        list.push_child(item);

        index = next_index;
    }

    Ok((list, index))
}
