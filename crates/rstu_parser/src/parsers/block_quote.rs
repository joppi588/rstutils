// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::parsers::block::parse_block;
use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_block_quote(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::BlankLine);
    stream.consume(); //Blankline
    let indent = stream.consume().len();
    let block = AstNode::new_ref(NodeClass::BlockQuote);
    block.push_child(AstNode::new_ref(NodeClass::BlankLine));
    block.with_attr("indent", indent);
    block.push_child(parse_block(stream)?);
    let dedent = stream.consume().len();
    if dedent > indent {
        stream.insert_before_cursor(Token::dedent(dedent - indent));
    }

    Ok(block)
}
