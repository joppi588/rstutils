// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use std::cell::RefCell;
use std::rc::Rc;

use super::paragraph::parse_paragraph;
use crate::parser_errors::ParserError;
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::find_next_kind;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

fn parse_block_body(
    block: &Rc<RefCell<AstNode>>,
    tokens: &[Token],
    start_at: usize,
) -> Result<usize, ParserError> {
    let mut index = start_at;
    while index < tokens.len() - 2 {
        let index_line_end = find_next_kind(&tokens, &[TK::NewLine, TK::BlankLine], index, None)
            .expect("Token stream ends with a newline.");
        match (tokens[index].kind, tokens[index_line_end + 1].kind) {
            (TK::Word, _) => {
                let (paragraph, new_index) = parse_paragraph(tokens, index, None, None)?;
                block.push_child(paragraph);
                index = new_index;
            }
            (TK::BlankLine, TK::Indent | TK::Dedent | TK::Word) => {
                block.push_blank_lines(tokens[index].lexeme.len());
                index += 1;
            }
            (TK::Dedent, _) => {
                // TODO: Do not dedent completely, modify token stream in place
                index += 1;
                break;
            }
            (_, _) => {
                break; // TODO: Should this be an error case?
            }
        }
    }
    Ok(index)
}

pub(crate) fn parse_block(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    let mut index = start_at;
    if tokens[start_at].kind == TK::Indent {
        block.with_attr("indent", tokens[start_at].lexeme.len());
        index += 1;
    }
    index = parse_block_body(&block, tokens, index)?;

    Ok((block, index))
}

pub(crate) fn parse_block_hanging_indent(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    let mut index = start_at;
    let index_line_end = find_next_kind(&tokens, &[TK::NewLine], index, None)
        .expect("Token stream ends with a newline."); // TODO: Integrate this in token stream.

    match tokens[index_line_end + 1].kind {
        TK::Field | TK::BulletListMarker => {
            // single line list case
            let (paragraph, new_index) =
                parse_paragraph(tokens, index, Some(index_line_end + 1), None)?;
            block.push_child(paragraph);
            index = new_index;
        }
        TK::BlankLine => {
            // first paragraph is a single line
            let (paragraph, new_index) =
                parse_paragraph(tokens, index, Some(index_line_end + 1), None)?;
            block.push_child(paragraph);
            index = new_index;

            if index + 1 < tokens.len() && tokens[index + 1].kind == TK::Indent {
                block.push_blank_lines(tokens[index].lexeme.len());
                block.with_attr("indent", tokens[index + 1].lexeme.len());
                index = parse_block_body(&block, tokens, index + 2)?;
            } else {
                return Ok((block, index));
            }
        }
        TK::Indent => {
            // first paragraph spans multiple lines
            block.with_attr("indent", tokens[index_line_end + 1].lexeme.len());
            let (paragraph, new_index) =
                parse_paragraph(tokens, index, None, Some(index_line_end + 1))?;
            block.push_child(paragraph);
            index = parse_block_body(&block, tokens, new_index)?;
        }
        _ => return Err(ParserError::UnexpectedBlockEndError {}),
    }

    Ok((block, index))
}
