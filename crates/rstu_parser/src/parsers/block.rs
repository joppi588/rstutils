// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::paragraph;
use crate::parser_errors::ParserError;
use crate::parsers::paragraph::parse_paragraph;
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::find_next_kind;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

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
    let (paragraph, new_index) = parse_paragraph(tokens, index, None, None)?;
    block.push_child(paragraph);
    Ok((block, new_index))
}

pub(crate) fn parse_block_hanging_indent(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    let mut indent: Option<usize> = None;
    let mut index = start_at;
    let index_line_end = find_next_kind(&tokens, &[TK::NewLine], index, None)
        .expect("Token stream ends with a newline."); // TODO: Integrate this in token stream.

    // Parse the first paragraph
    match tokens[index_line_end + 1].kind {
        TK::BlankLine | TK::Field | TK::BulletListMarker => {
            let (paragraph, new_index) =
                paragraph::parse_paragraph(tokens, index, Some(index_line_end + 1), None)?;
            block.push_child(paragraph);
            index = new_index;
        }
        TK::Indent => {
            let (paragraph, new_index) =
                paragraph::parse_paragraph(tokens, index, None, Some(index_line_end + 1))?;
            block.push_child(paragraph);
            indent = Some(tokens[index_line_end + 1].lexeme.len());

            index = new_index;
        }
        _ => return Err(ParserError::UnexpectedBlockEndError {}),
    }

    // Parse the rest
    while index < tokens.len() - 2 {
        let index_line_end = find_next_kind(&tokens, &[TK::NewLine, TK::BlankLine], index, None)
            .expect("Token stream ends with a newline.");
        match (tokens[index].kind, tokens[index_line_end + 1].kind) {
            (TK::Indent, _) => {
                let (paragraph, new_index) =
                    paragraph::parse_paragraph(tokens, index + 1, None, None)?;
                block.push_child(paragraph);
                indent = Some(tokens[index].lexeme.len());
                index = new_index;
            }
            (TK::Word, _) => {
                let (paragraph, new_index) = paragraph::parse_paragraph(tokens, index, None, None)?;
                block.push_child(paragraph);
                index = new_index;
            }
            (TK::BlankLine, TK::BlankLine | TK::Indent | TK::Dedent) => {
                block.push_child(AstNode::new_ref(NodeClass::BlankLine));
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

    if let Some(indent) = indent {
        block.with_attr("indent", indent);
    }
    Ok((block, index))
}
