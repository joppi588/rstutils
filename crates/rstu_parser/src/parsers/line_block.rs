// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::parsers::paragraph::parse_paragraph_hanging_indent;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_line_block(
    stream: &mut TokenStream,
    indent: usize,
) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::LineBlock);

    let block = AstNode::new_ref(NodeClass::LineBlock);
    let indent = indent;
    if indent > 0 {
        block.with_attr("indent", indent);
    }

    while stream.token_at_cursor().kind == TK::LineBlock {
        let line_block_lexeme: &str = &stream.token_at_cursor().lexeme;
        let new_indent = line_block_lexeme.len() - 2;
        if new_indent > indent {
            block.push_child(parse_line_block(stream, new_indent)?);
            break;
        } else if new_indent < indent {
            break;
        }

        stream.consume(); //line block token
        let line = AstNode::new_ref(NodeClass::Line);
        let paragraph = parse_paragraph_hanging_indent(stream)?;
        line.push_child(paragraph);
        block.push_child(line);
    }

    Ok(block)
}
