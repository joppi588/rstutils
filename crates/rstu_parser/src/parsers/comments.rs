// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::find_next_kind;
use crate::token_stream::{self, TokenStream};

pub(crate) fn parse_comment(
    stream: &mut TokenStream,
    start_at: usize,
    first_line_end: usize,
) -> Result<NodeRef, ParserError> {
    let mut index = first_line_end;
    if stream.kind_at(index + 1) == TK::Indent {
        index = find_next_kind(stream.tokens(), &[TK::Dedent], index + 1)
            .expect("There is always a final dedent.");
    }

    let comment = AstNode::new_ref(NodeClass::Comment);
    let comment_tokens = token_stream::tokens_without_kinds(
        &stream.tokens()[start_at + 2..index + 1],
        &[TK::Indent, TK::Dedent],
    );
    comment.with_attr("text", token_stream::tokens_to_text(&comment_tokens));
    stream.set_cursor(index + 1);
    Ok(comment)
}
