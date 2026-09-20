// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::TokenKind as TK;
use crate::token_stream::{self, TokenStream};

pub(crate) fn parse_comment(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let comment = AstNode::new_ref(NodeClass::Comment);
    let mut index = stream.find_next_kind(&[TK::NewLine]).expect(EXPECT_NEWLINE);
    if stream.kind_at(index + 1) == TK::Indent {
        let indent_token = stream.take_at(index + 1);
        comment.with_attr("indent", indent_token.lexeme.len());
        index = stream
            .find_next_kind(&[TK::Dedent])
            .expect("There is always a final dedent.");
    }

    let comment_tokens = &stream.tokens()[stream.cursor() + 2..index + 1];
    comment.with_attr("text", token_stream::tokens_to_text(&comment_tokens));
    stream.set_cursor(index + 1);
    Ok(comment)
}
