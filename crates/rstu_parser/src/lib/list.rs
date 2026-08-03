// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef};

use crate::parser_errors::FindElementError;
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::{find_next_kind, skip_kinds, tokens_to_text};

pub(crate) fn try_parse_field_list(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let list = AstNode::new_ref(NodeClass::List);
    let mut index = start_at;

    if index >= tokens.len() {
        return Err(FindElementError::StartAtOutOfBounds {
            start_at,
            token_count: tokens.len(),
        });
    }

    if tokens[index].kind != TK::Field {
        let first_field = find_next_kind(tokens, &[TK::Field], index).map_err(|_| {
            FindElementError::UnexpectedToken {
                expected: "Field".to_owned(),
                found: format!("{:?}", tokens[index].kind),
            }
        })?;
        AstNode::with_text(&list, tokens_to_text(&tokens[index..first_field]));
        index = first_field;
    }

    while index < tokens.len() && tokens[index].kind == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_name = tokens[index]
            .lexeme
            .trim_start_matches(':')
            .trim_end_matches(':')
            .to_string();
        AstNode::with_attr(&item, "fieldname", field_name);

        let body_start = skip_kinds(tokens, &[TK::Spaces], index + 1).unwrap_or(index + 1);
        let (paragraph, next_index) = super::try_parse_paragraph(tokens, body_start)?;
        AstNode::push_child(&item, paragraph);
        AstNode::push_child(&list, item);

        index = next_index;
    }

    Ok((list, index))
}
