// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::parser_errors::ParserError;
use crate::token::{Token, TokenKind as TK};
use crate::{
    paragraph,
    token_slice::{find_next_kind, skip_kinds},
};
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_block(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let line_end_index = find_next_kind(tokens, &[TK::NewLine], start_at, None)
        .expect("Token stream ends with a newline.");
    match tokens[line_end_index + 1].kind {
        TK::Indent => parse_indented_block_hanging(tokens, start_at, line_end_index + 1),
        TK::Field | TK::BulletListMarker | TK::BlankLine => {
            parse_single_line_block(tokens, start_at, line_end_index + 1)
        }
        _ => Err(ParserError::UnexpectedBlockEndError {}),
    }
}

pub(crate) fn parse_indented_block_hanging(
    tokens: &[Token],
    start_at: usize,
    indent_position: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let block = AstNode::new_ref(NodeClass::IndentedBlockHanging);
    block.with_attr("indent", tokens[indent_position].lexeme.len());
    let mut index = start_at;
    // TODO: Recursion/loop comes here.
    let (paragraph, new_index) =
        paragraph::parse_paragraph(tokens, index, None, Some(indent_position))?;
    block.push_child(paragraph);
    index = new_index;
    if tokens[index].kind == TK::BlankLine {
        index = skip_kinds(tokens, &[TK::BlankLine], index);
        block.push_child(AstNode::new_ref(NodeClass::BlankLine))
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
    paragraph::parse_paragraph(tokens, start_at, Some(stop_before), None)
}
