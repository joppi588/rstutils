// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::parser_errors::ParserError;
use crate::token::{Token, TokenKind as TK};
use crate::{
    paragraph,
    token_slice::{find_next_kind, skip_kinds},
};
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_indented_block_hanging(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    // TODO: Having this logic one level above, we need to differenciate between paragraph continuation and new list item
    let line_end_index = find_next_kind(tokens, &[TK::NewLine], start_at, None)
        .expect("Token stream ends with a newline.");
    match tokens[line_end_index + 1].kind {
        TK::Indent => {
            let block = AstNode::new_ref(NodeClass::IndentedBlockHanging);
            block.with_attr("indent", tokens[line_end_index + 1].lexeme.len());
            let mut index = start_at;
            // TODO: Recursion comes here.
            let (paragraph, new_index) =
                paragraph::try_parse_paragraph(tokens, index, None, Some(line_end_index + 1))?;
            block.push_child(paragraph);
            index = new_index;
            if tokens[index].kind == TK::BlankLine {
                index = skip_kinds(tokens, &[TK::BlankLine], index);
                block.push_child(AstNode::new_ref(NodeClass::BlankLine))
            }
            if tokens[index].kind == TK::Dedent {
                // TODO: Only dedent the indent, modify the dedent token in place.
                index += 1;
            } else {
                panic!("Expected a dedent at {index}")
            }
            Ok((block, index))
        }
        TK::Field | TK::BulletListMarker | TK::BlankLine =>
        // Single line case
        {
            paragraph::try_parse_paragraph(tokens, start_at, Some(line_end_index + 1), None)
        }
        _ => Err(FindElementError::ListEndError {}),
    }
}
