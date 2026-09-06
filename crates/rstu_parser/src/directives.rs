// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::{find_next_kind, tokens_to_text};
use crate::{block, list};

pub(crate) fn parse_directive(
    tokens: &[Token],
    start_at: usize,
    directive_colon_index: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let first_line_end =
        find_next_kind(tokens, &[TK::NewLine], directive_colon_index, None).expect(EXPECT_NEWLINE);

    let directive_type = tokens_to_text(&tokens[start_at + 1..directive_colon_index])
        .trim()
        .to_string();
    let directive_arguments = tokens_to_text(&tokens[directive_colon_index + 1..first_line_end]);

    let directive = AstNode::new_ref(NodeClass::Directive);
    directive.with_attr("directive_type", directive_type);
    if !directive_arguments.is_empty() {
        directive.with_attr("directive_arguments", directive_arguments);
    }

    let mut index = first_line_end + 1;
    if index >= tokens.len() || tokens[index].kind != TK::Indent {
        return Ok((directive, index));
    }

    if tokens[index + 1].kind == TK::Field {
        let (options, new_index) = list::parse_field_list(tokens, index + 1)?;
        directive.push_child(options);
        index = new_index;
    }

    if index < tokens.len() && tokens[index].kind != TK::Dedent {
        let body_start = index + usize::from(tokens[index].kind == TK::Indent);
        let (content, new_index) = block::parse_block(tokens, body_start)?;
        directive.push_child(content);
        index = new_index;
    }

    Ok((directive, index))
}
