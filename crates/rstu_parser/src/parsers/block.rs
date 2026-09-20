// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::paragraph::parse_paragraph;
use crate::parse_body_elements;
use crate::parser_errors::ParserError;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_block(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let block = AstNode::new_ref(NodeClass::Block);
    let mut indent: usize = 0;
    if stream.kind_at_cursor() == TK::Indent {
        let token = stream.consume();
        indent = token.lexeme.len();
        block.with_attr("indent", indent);
    }
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
                let dedent_token = stream.consume();
                let dedent = dedent_token.lexeme.len();
                if dedent != indent {
                    let cursor = stream.cursor();
                    let kind = if dedent < indent {
                        TK::Indent
                    } else {
                        TK::Dedent
                    };
                    let rel_indent = Token::new(kind, " ".repeat(dedent.abs_diff(indent)));
                    stream.insert_at(cursor, rel_indent);
                    stream.set_cursor(cursor);
                }
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

    Ok(block)
}
