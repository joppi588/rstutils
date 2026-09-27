// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::{block::parse_block, list::parse_field_list};
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::TokenKind as TK;
use crate::token_stream::{tokens_to_text, TokenStream};

pub(crate) fn parse_directive(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let marker_index = stream.cursor();
    let directive_colon_index = if stream.kind_at_cursor() == TK::Directive {
        None
    } else {
        Some(
            stream
                .find_next_kind_from(&[TK::DoubleColon], marker_index)
                .expect(EXPECT_NEWLINE),
        )
    };
    let first_line_end = stream.find_next_kind(&[TK::NewLine]).expect(EXPECT_NEWLINE);

    let directive = AstNode::new_ref(NodeClass::Directive);
    let directive_type = match directive_colon_index {
        None => stream.tokens()[marker_index]
            .lexeme
            .strip_prefix(".. ")
            .and_then(|marker| marker.strip_suffix("::"))
            .unwrap_or_default()
            .trim()
            .to_string(),
        Some(colon_index) => tokens_to_text(&stream.tokens()[marker_index + 1..colon_index])
            .trim()
            .to_string(),
    };
    directive.with_attr("directive_type", directive_type);

    let arguments_start = directive_colon_index.map_or(marker_index + 1, |index| index + 2);
    if first_line_end > arguments_start {
        let directive_arguments = tokens_to_text(&stream.tokens()[arguments_start..first_line_end])
            .trim()
            .to_string();
        directive.with_attr("directive_arguments", directive_arguments);
    }

    stream.set_cursor(first_line_end + 1);
    if stream.kind_at_cursor() != TK::Indent {
        return Ok(directive);
    }
    directive.with_attr("indent", stream.tokens()[stream.cursor()].lexeme.len());

    if stream.kind_peek_relative(1) == TK::Field {
        stream.consume(); // Skip the shared Indent token; parse_block consumes it otherwise.
        let options = parse_field_list(stream)?;
        directive.push_child(options);
    }

    if stream.kind_at_cursor() != TK::Dedent && stream.kind_at_cursor() != TK::EoF {
        let content = parse_block(stream)?;
        directive.push_child(content);
    }

    Ok(directive)
}
