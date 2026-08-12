// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::parser_errors::FindElementError;
use crate::token::{Token, TokenKind};
use crate::{paragraph, token_slice::find_next_kind};
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_indented_block_hanging(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    // TODO: Having this logic one level above, we could differenciate between paragraph continuation and new list item
    let line_end_index = find_next_kind(tokens, &[TokenKind::NewLine], start_at)
        .expect("Token stream ends with a newline.");
    if tokens[line_end_index + 1].kind == TokenKind::Indent {
        let block = AstNode::new_ref(NodeClass::IndentedBlockHanging);
        block.with_attr("indent", tokens[line_end_index + 1].lexeme.len());
        let (paragraph, new_index) =
            paragraph::try_parse_paragraph(tokens, start_at + 2, None, Some(line_end_index + 1))?;
        block.push_child(paragraph);
        Ok((block, new_index))
    } else {
        // Single line case
        paragraph::try_parse_paragraph(tokens, start_at, Some(line_end_index + 1), None)
    }
}
