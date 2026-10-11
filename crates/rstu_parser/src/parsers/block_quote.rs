// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parse_body_element;
use crate::parser_errors::ParserError;
use crate::parsers::paragraph::parse_paragraph;
use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_block_quote(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::BlankLine);
    let block = AstNode::new_ref(NodeClass::BlockQuote);
    block.push_blank_lines(stream.consume().len());
    debug_assert_matches!(stream.token_at_cursor().kind, TK::Indent);
    let indent: usize = stream.consume().len();
    block.with_attr("indent", indent);

    while !matches!(stream.token_at_cursor().kind, TK::Dedent | TK::Attribution) {
        parse_body_element(stream, &block)?;
    }
    if stream.token_at_cursor().kind == TK::Attribution {
        let attribution = parse_attribution(stream)?;
        block.push_child(attribution);
    }
    let dedent = stream.consume().len();
    if dedent > indent {
        let remaining_dedent = Token::dedent(dedent - indent);
        stream.insert_at_cursor(remaining_dedent);
        return Ok(block);
    }
    if !matches!(stream.token_at_cursor().kind, TK::BlankLine | TK::EoF) {
        return Err(ParserError::BlockQuoteMissingBlankLineError {});
    } else {
        block.push_blank_lines(stream.consume().len());
    }
    Ok(block)
}

fn parse_attribution(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let attribution = AstNode::new_ref(NodeClass::Attribution);

    attribution.with_attr("marker", stream.consume().lexeme);

    // Allow indented paragraph
    // TODO: move to paragraph
    let index_next_line = stream.find_end_of_line() + 1;
    let paragraph = if stream.token_at(index_next_line).kind == TK::Indent {
        let indent = stream.take_at(index_next_line);
        attribution.with_attr("hanging_indent", indent.len());

        let paragraph = parse_paragraph(stream)?;
        let dedent = stream.token_at_cursor();
        if dedent.kind != TK::Dedent {
            return Err(ParserError::UnexpectedToken {
                expected: "Dedent".to_string(),
                found: format!("{:?}", dedent.kind),
                index: stream.cursor(),
            });
        } else if dedent.len() > indent.len() {
            stream.update_at_cursor(" ".repeat(dedent.len() - indent.len()));
        }
        paragraph
    } else {
        parse_paragraph(stream)?
    };

    attribution.push_child(paragraph);
    Ok(attribution)
}
