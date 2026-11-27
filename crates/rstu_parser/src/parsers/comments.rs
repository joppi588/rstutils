// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::TokenKind::{self as TK};
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_comment(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::DoubleDot);
    let comment = AstNode::new_ref(NodeClass::Comment);

    // Empty comment: ".."" followed by blank line
    // One line comment: ".. text" followed by any non-indent
    // multi line comment 1: ".. text"
    //                        (optional blankline)
    //                        indented text
    //                    2: ..
    //                         indented text

    stream.consume(); // Comment marker
    if stream.token_at_cursor().kind == TK::Spaces {
        stream.consume(); // TODO: push something or set attribute
    }
    let mut text = stream.consume_text_incl_newline();
    if text == "\n" {
        let kind = stream.token_at_cursor().kind;
        if kind == TK::BlankLine {
            return Ok(comment);
        }
        if kind != TK::Indent {
            return Err(ParserError::UnexpectedToken {
                expected: "Blankline or indent".to_owned(),
                found: format!("{:?}", kind),
                position: stream.position_at_cursor(),
            });
        }
    }
    if stream.token_at_cursor().kind == TK::BlankLine {
        text += &stream.consume().lexeme;
    }
    if stream.token_at_cursor().kind == TK::Indent {
        let base_indent = stream.consume().len();
        comment.with_attr("indent", base_indent);
        text.push_str(&stream.consume_indented_text(base_indent));
    }
    comment.with_attr("text", text);
    Ok(comment)
}
