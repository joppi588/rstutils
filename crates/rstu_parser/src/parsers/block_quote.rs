// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::parsers::block::parse_block;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_block_quote(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::BlankLine);
    let block = AstNode::new_ref(NodeClass::BlockQuote);
    stream.consume(); //Blankline
    block.push_child(AstNode::new_ref(NodeClass::BlankLine));
    block.push_child(parse_block(stream)?);
    if !matches!(
        stream.token_at_cursor().kind,
        TK::BlankLine | TK::Dedent | TK::EoF
    ) {
        return Err(ParserError::BlockQuoteMissingBlankLineError {});
    }
    Ok(block)
}
