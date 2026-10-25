// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::paragraph::parse_paragraph;
use super::paragraph::parse_paragraph_hanging_indent;
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_footnote(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::FootNote);

    let marker = stream.consume().lexeme;
    let label = marker
        .strip_prefix(".. [")
        .and_then(|marker| marker.strip_suffix(']'))
        .unwrap_or_default();
    let footnote = AstNode::new_ref(NodeClass::Footnote);
    footnote.with_attr("label", label);

    if stream.token_at_cursor().kind == TK::Spaces {
        stream.consume();
    }
    let index_next_line: usize = stream.find_end_of_line() + 1;
    let paragraph = match stream.token_at(index_next_line).kind {
        TK::Indent => parse_paragraph_hanging_indent(stream)?,
        _ => parse_paragraph(stream)?,
    };
    footnote.push_child(paragraph);
    Ok(footnote)
}
