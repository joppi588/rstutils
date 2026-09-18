// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use std::cell::RefCell;
use std::rc::Rc;

use super::paragraph::{parse_paragraph, parse_paragraph_with_hanging_indent};
use crate::parse_body_elements;
use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

fn parse_block_body(
    block: &Rc<RefCell<AstNode>>,
    stream: &mut TokenStream,
) -> Result<(), ParserError> {
    loop {
        match (stream.kind_at_cursor(), stream.kind_at_nextline()) {
            (TK::Word, _) => {
                let paragraph = parse_paragraph(stream)?;
                block.push_child(paragraph);
            }
            (TK::BlankLine, TK::Indent | TK::Dedent | TK::Word) => {
                let token = stream.consume();
                block.push_blank_lines(token.lexeme.len());
            }
            (TK::Dedent, _) => {
                // TODO: Do not dedent completely, modify token stream in place
                // check indentation level.
                stream.consume();
                break;
            }
            (kind, _) if kind.nested_is(TC::BODY_ELEMENTS) => {
                parse_body_elements(stream, &block)?;
            }
            (_, TK::EoF) => {
                break;
            }
            (_, _) => {
                break; // TODO: Should this be an error case?
            }
        }
    }
    Ok(())
}

pub(crate) fn parse_block(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    if stream.kind_at_cursor() == TK::Indent {
        let token = stream.consume();
        block.with_attr("indent", token.lexeme.len());
    }
    parse_block_body(&block, stream)?;

    Ok(block)
}

pub(crate) fn parse_block_hanging_indent(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    match stream.kind_at_nextline() {
        TK::Field | TK::BulletListMarker | TK::EoF | TK::Dedent => {
            // TODO: Can we check this already before the function call?
            // single line list case
            let paragraph = parse_paragraph(stream)?;
            block.push_child(paragraph);
        }
        TK::BlankLine => {
            // first paragraph is a single line
            let paragraph = parse_paragraph(stream)?;
            block.push_child(paragraph);

            if stream.kind_peek_relative(1) == TK::Indent {
                let blank_token = stream.consume();
                block.push_blank_lines(blank_token.lexeme.len());
                let indent_token = stream.consume();
                block.with_attr("indent", indent_token.lexeme.len());
                parse_block_body(&block, stream)?;
            } else {
                return Ok(block);
            }
        }
        TK::Indent => {
            // first paragraph spans multiple lines
            block.with_attr("indent", stream.token_at_nextline().lexeme.len());
            let paragraph = parse_paragraph_with_hanging_indent(stream)?;
            block.push_child(paragraph);
            parse_block_body(&block, stream)?;
        }
        _ => return Err(ParserError::UnexpectedBlockEndError {}),
    }

    Ok(block)
}
