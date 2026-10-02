// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::parse_body_elements;
use crate::parser_errors::ParserError;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_block(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    let mut indent: usize = 0;
    if stream.token_at_cursor().kind == TK::Indent {
        let token = stream.consume();
        indent = token.len();
        block.with_attr("indent", indent);
    }
    loop {
        match stream.token_at_cursor().kind {
            TK::BlankLine => {
                let token = stream.consume();
                block.push_blank_lines(token.len());
            }
            TK::Dedent => {
                let dedent_token = stream.consume();
                let dedent = dedent_token.len();
                if dedent != indent {
                    let width = dedent.abs_diff(indent);
                    let rel_indent = if dedent < indent {
                        Token::indent(width)
                    } else {
                        Token::dedent(width)
                    };
                    stream.insert_before_cursor(rel_indent);
                }
                break;
            }
            kind if kind.nested_is(TC::BODY_ELEMENTS) => {
                parse_body_elements(stream, &block)?;
            }
            _ => {
                break; // TODO: Should this be an error case?
            }
        }
    }

    Ok(block)
}
