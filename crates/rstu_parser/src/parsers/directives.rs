// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::{block::parse_block, list::parse_field_list};
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::TokenKind as TK;
use crate::token_stream::{tokens_to_text, TokenStream};

pub(crate) fn parse_directive(
    stream: &mut TokenStream,
    directive_colon_index: usize, // TODO: It is ok to know this outside, but re-calculating inside (use a loop) would safe one parameter
) -> Result<NodeRef, ParserError> {
    let first_line_end = stream
        .find_next_kind_from(&[TK::NewLine], directive_colon_index)
        .expect(EXPECT_NEWLINE);

    let directive = AstNode::new_ref(NodeClass::Directive);
    let directive_type =
        tokens_to_text(&stream.tokens()[stream.cursor() + 1..directive_colon_index])
            .trim()
            .to_string();
    directive.with_attr("directive_type", directive_type);

    if first_line_end > directive_colon_index + 1 {
        let directive_arguments =
            tokens_to_text(&stream.tokens()[directive_colon_index + 2..first_line_end]);
        directive.with_attr("directive_arguments", directive_arguments);
    }

    stream.set_cursor(first_line_end + 1);
    if stream.kind_at_cursor() != TK::Indent {
        return Ok(directive);
    }
    directive.with_attr("indent", stream.tokens()[stream.cursor()].lexeme.len());

    if stream.kind_peek_relative(1) == TK::Field {
        stream.consume_n(1); // Skip the shared Indent token; parse_block consumes it otherwise.
        let options = parse_field_list(stream)?;
        directive.push_child(options);
    }

    if stream.kind_at_cursor() != TK::Dedent && stream.kind_at_cursor() != TK::EoF {
        let content = parse_block(stream)?;
        directive.push_child(content);
    }

    Ok(directive)
}
