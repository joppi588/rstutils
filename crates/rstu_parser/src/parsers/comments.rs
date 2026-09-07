// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::{Token, TokenKind as TK};
use crate::token_slice::find_next_kind;
use crate::token_slice::{self};

pub(crate) fn parse_comment(
    tokens: &[Token],
    start_at: usize,
    first_line_end: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let mut index = first_line_end;
    if tokens[index + 1].kind == TK::Indent {
        index = find_next_kind(tokens, &[TK::Dedent], index + 1, None)
            .expect("There is always a final dedent.");
    }

    let comment = AstNode::new_ref(NodeClass::Comment);
    let comment_tokens = token_slice::tokens_without_kinds(
        &tokens[start_at + 2..index + 1],
        &[TK::Indent, TK::Dedent],
    );
    comment.with_text(token_slice::tokens_to_text(&comment_tokens));
    Ok((comment, index + 1))
}
