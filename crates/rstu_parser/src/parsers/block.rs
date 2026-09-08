// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::paragraph;
use crate::parser_errors::ParserError;
use crate::parsers::paragraph::parse_paragraph;
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::{find_next_kind, skip_kinds};
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_block(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let line_end_index = find_next_kind(tokens, &[TK::NewLine], start_at, None)
        .expect("Token stream ends with a newline.");
    let following_index = skip_kinds(tokens, &[TK::BlankLine], line_end_index + 1);
    match tokens[following_index].kind {
        TK::Indent => parse_compound_block(tokens, start_at, following_index),
        TK::Field | TK::BulletListMarker => {
            parse_paragraph(tokens, start_at, Some(following_index), None)
        }
        TK::Dedent | TK::EOF => parse_compound_block(tokens, start_at, start_at - 1),
        _ => Err(ParserError::UnexpectedBlockEndError {}),
    }
}

pub(crate) fn parse_compound_block(
    tokens: &[Token],
    start_at: usize,
    indent_position: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    block.with_attr("indent", tokens[indent_position].lexeme.len());
    if indent_position >= start_at {
        block.with_attr("hanging_indent", true);
    }
    let mut index = start_at;
    while index < tokens.len() {
        if index == indent_position {
            index += 1;
        }; // TODO: include in token_slice
        match tokens[index].kind {
            TK::BlankLine => {
                block.push_child(AstNode::new_ref(NodeClass::BlankLine));
                index += 1;
            }
            TK::Dedent => {
                // TODO: Only dedent the indent, modify the dedent token in place.
                index += 1;
                break;
            }
            _ => {
                let (paragraph, new_index) =
                    paragraph::parse_paragraph(tokens, index, None, Some(indent_position))?;
                block.push_child(paragraph);
                index = new_index;
            }
        }
    }

    Ok((block, index))
}
