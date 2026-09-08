// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::paragraph;
use crate::parser_errors::ParserError;
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::find_next_kind;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_block(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let line_end_index = find_next_kind(tokens, &[TK::NewLine], start_at, None)
        .expect("Token stream ends with a newline.");
    let following_index = skip_kinds(tokens, &[TK::BlankLine], line_end_index + 1);
    match tokens[following_index].kind {
        TK::Indent => parse_block_hanging_indent(tokens, start_at, following_index),
        TK::BlankLine => parse_compound_block(tokens, start_at, following_index),
        TK::Dedent | TK::Field | TK::BulletListMarker => {
            parse_single_line_block(tokens, start_at, following_index)
        }
        _ => Err(ParserError::UnexpectedBlockEndError {}),
    }
}

pub(crate) fn parse_compound_block(
    tokens: &[Token],
    start_at: usize,
    stop_before: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    let mut index = start_at;
    let (paragraph, new_index) =
        paragraph::parse_paragraph(tokens, index, Some(stop_before), None)?;
    block.push_child(paragraph);
    index = new_index;
    Ok((block, index))
}

pub(crate) fn parse_block_hanging_indent(
    tokens: &[Token],
    start_at: usize,
    indent_position: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let block = AstNode::new_ref(NodeClass::BlockHangingIndent);
    block.with_attr("indent", tokens[indent_position].lexeme.len());
    let mut index = start_at;
    // TODO: Recursion/loop comes here.
    let (paragraph, new_index) =
        paragraph::parse_paragraph(tokens, index, None, Some(indent_position))?;
    block.push_child(paragraph);
    index = new_index;
    if tokens[index].kind == TK::BlankLine {
        block.push_child(AstNode::new_ref(NodeClass::BlankLine));
        index += 1;
    }
    if tokens[index].kind == TK::Dedent {
        // TODO: Only dedent the indent, modify the dedent token in place.
        index += 1;
    } else {
        panic!("Expected a dedent at {index}")
    }
    Ok((block, index))
}

pub(crate) fn parse_single_line_block(
    tokens: &[Token],
    start_at: usize,
    stop_before: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let (paragraph, index) = paragraph::parse_paragraph(tokens, start_at, Some(stop_before), None)?;
    Ok((paragraph, index))
}
