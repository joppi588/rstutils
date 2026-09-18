// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use std::cell::RefCell;
use std::rc::Rc;

use super::paragraph::parse_paragraph;
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
            (TK::Word | TK::NewLine, _) => {
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
