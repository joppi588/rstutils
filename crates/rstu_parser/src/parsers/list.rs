// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::block::parse_block;
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

fn prepare_item_block(stream: &mut TokenStream, dedent_len: usize) -> Result<(), ParserError> {
    // List item cases
    // 1. [Optional Blankline],  Indent -> Hanging indent block
    // 2. [Optional Blankline], list marker -> Single line item
    // 3. Non-indented paragraph etc -> Error
    let next_line = stream.find_next_kind(&[TK::NewLine]).expect(EXPECT_NEWLINE) + 1;

    let indent_ahead_index = match stream.kind_at(next_line) {
        TK::Indent => Some(next_line),
        TK::BlankLine if stream.kind_at(next_line + 1) == TK::Indent => Some(next_line + 1),
        _ => None,
    };

    match indent_ahead_index {
        Some(indent_index) => {
            let indent_token = stream.take_at(indent_index);
            let cursor = stream.cursor();
            stream.insert_at(cursor, indent_token);
            stream.set_cursor(cursor);
        }
        None => match stream.kind_at(next_line) {
            TK::Field | TK::BulletListMarker | TK::EoF | TK::Dedent | TK::BlankLine => {
                stream.insert_at(next_line, Token::new(TK::Dedent, " ".repeat(dedent_len)));
            }
            _ => return Err(ParserError::ListEndError {}),
        },
    }
    Ok(())
}

// TODO: Make this the main function and do the preparation depending on the list marker.
fn finish_list_item(
    stream: &mut TokenStream,
    list: &NodeRef,
    item: NodeRef,
    dedent_len: usize,
) -> Result<(), ParserError> {
    if stream.kind_at_cursor() == TK::Spaces {
        let spaces_token = stream.consume();
        item.push_spaces(spaces_token.lexeme.len());
    }
    prepare_item_block(stream, dedent_len)?;
    let block = parse_block(stream).map_err(|_| ParserError::ListEndError {})?;
    item.push_child(block);
    list.push_child(item);
    if stream.kind_at_cursor() == TK::BlankLine {
        let blank_token = stream.consume();
        list.push_blank_lines(blank_token.lexeme.len());
    }
    Ok(())
}

pub(crate) fn parse_bullet_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let list = AstNode::new_ref(NodeClass::BulletList);
    let mut marker: Option<String> = None;

    while stream.kind_at_cursor() == TK::BulletListMarker {
        let item = AstNode::new_ref(NodeClass::BulletListItem);
        let marker_token = stream.consume();
        item.with_attr("marker", marker_token.lexeme.clone());
        if let Some(existing_marker) = &marker {
            if existing_marker != &marker_token.lexeme {
                return Err(ParserError::ListStyleError {
                    marker: existing_marker.clone(),
                    conflicting_marker: marker_token.lexeme,
                });
            }
        } else {
            marker = Some(marker_token.lexeme);
        }

        finish_list_item(stream, &list, item, 2)?;
    }

    Ok(list)
}

pub(crate) fn parse_field_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let list = AstNode::new_ref(NodeClass::FieldList);

    while stream.kind_at_cursor() == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_token = stream.consume();
        let field_name = field_token
            .lexeme
            .strip_prefix(':')
            .and_then(|name| name.strip_suffix(':'))
            .unwrap()
            .to_owned();
        item.with_attr("fieldname", field_name);

        let dedent_len = field_token.lexeme.len();
        finish_list_item(stream, &list, item, dedent_len)?;
    }

    Ok(list)
}
