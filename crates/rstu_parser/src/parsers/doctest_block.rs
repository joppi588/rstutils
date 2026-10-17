// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_doctest_block(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::DoctestStart);

    let block = AstNode::new_ref(NodeClass::DoctestBlock);
    let mut text = String::new();
    // Indentation added by output lines, relative to the doctest start.
    let mut rel_indent = 0;

    loop {
        match stream.token_at_cursor().kind {
            TK::BlankLine | TK::EoF => break,
            TK::Indent => {
                let indent = stream.consume();
                rel_indent += indent.len();
                text.push_str(&indent.lexeme);
            }
            TK::Dedent => {
                let dedent = stream.token_at_cursor().len();
                if dedent > rel_indent {
                    // The rest of the dedent belongs to the enclosing block.
                    stream.update_at_cursor(space!(dedent - rel_indent));
                    break;
                }
                stream.consume();
                rel_indent -= dedent;
            }
            _ => text.push_str(&stream.consume().lexeme),
        }
    }

    block.with_attr("text", text);
    Ok(block)
}
