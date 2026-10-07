// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::{consume_trailing_blank_lines, parse_item_body};
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};
use std::debug_assert_matches;

pub(crate) fn parse_field_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::Field);

    let list = AstNode::new_ref(NodeClass::FieldList);

    while stream.token_at_cursor().kind == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_token = stream.consume();
        let field_name = field_token
            .lexeme
            .strip_prefix(':')
            .and_then(|name| name.strip_suffix(':'))
            .unwrap()
            .to_owned();
        item.with_attr("fieldname", field_name);

        let dedent_len = field_token.len();
        item.push_child(parse_item_body(stream, dedent_len)?);
        list.push_child(item);
        consume_trailing_blank_lines(stream, &list);
    }

    Ok(list)
}
