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
    let line_end_index = find_next_kind(tokens, &[TokenKind::NewLine], start_at)
        .expect("Token stream ends with a newline.");
    if tokens[line_end_index + 1].kind == TokenKind::Indent
    // look-adhead line_end_index
    // if there is an indent, go ahead until the "paragraph end" (dedent/indent/blankline)
    //     store the indent in the indented block
    // if no indent, parse until eol
    {
        let block = AstNode::new_ref(NodeClass::IndentedBlockHanging);
        block.with_attr("indent", tokens[line_end_index + 1].lexeme.len());
        let (paragraph, new_index) = paragraph::try_parse_paragraph(tokens, start_at + 2, None)?;
        block.push_child(paragraph);
        Ok((block, new_index))
    } else {
        paragraph::try_parse_paragraph(tokens, start_at, None)
    }
}
