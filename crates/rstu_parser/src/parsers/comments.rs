// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_comment(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::DoubleDot);

    stream.consume(); // Comment marker
    let comment = AstNode::new_ref(NodeClass::Comment);

    // TODO: move this into the loop, match token and next line token?
    if stream.token_at_cursor().kind == TK::Spaces {
        stream.consume();
    }
    let text_start = stream.cursor() + 1;
    let mut text = stream
        .consume_text_until(&[TK::NewLine])
        .expect(EXPECT_NEWLINE);
    let newline_index = stream.cursor() - 1;
    if text_start <= newline_index {
        // Keep the terminating newline unless the first line had no body to begin with.
        text.push_str(&stream.token_at(newline_index).lexeme);
    }

    if stream.token_at_cursor().kind == TK::Indent {
        let base_indent = stream.take_at_cursor().len();
        comment.with_attr("indent", base_indent);

        let mut absolute_indent = base_indent;
        loop {
            match stream.token_at_cursor().kind {
                TK::Indent => {
                    absolute_indent += stream.take_at_cursor().len();
                    text.push_str(&space!(absolute_indent - base_indent));
                }
                TK::Dedent => {
                    absolute_indent -= stream.take_at_cursor().len();
                    if absolute_indent == 0 {
                        break;
                    }
                    text.push_str(&space!(absolute_indent - base_indent));
                }
                _ => text.push_str(&stream.take_at_cursor().lexeme),
            }
        }
    }

    comment.with_attr("text", text);
    Ok(comment)
}
