// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::{find_next_kind, tokens_to_text};
use crate::{list, paragraph};

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
    let directive_text = tokens_to_text(&tokens[directive_colon_index + 1..first_line_end]);

    let directive = AstNode::new_ref(NodeClass::Directive);
    directive.with_attr("directive_type", directive_type);
    if !directive_text.is_empty() {
        directive.with_text(directive_text);
    }

    let index = first_line_end + 1;
    if index >= tokens.len() || tokens[index].kind != TK::Indent {
        return Ok((directive, index));
    }

    if tokens[index + 1].kind == TK::Field {
        let (options, index) = list::parse_field_list(tokens, index + 1)?;
        directive.push_child(options);
        return Ok((directive, index));
    }

    let indentation = tokens[index].lexeme.clone();
    let indented_block = AstNode::new_ref(NodeClass::Block);
    indented_block.with_attr("indentation", indentation);
    let (paragraph, index) = paragraph::parse_paragraph(tokens, index + 1, None, None)?;
    indented_block.push_child(paragraph);
    directive.push_child(indented_block);

    Ok((directive, index))
}
