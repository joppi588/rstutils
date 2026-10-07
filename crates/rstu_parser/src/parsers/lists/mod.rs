// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod bullet;
mod enumerated;
mod field;
mod option;

pub(crate) use bullet::parse_bullet_list;
pub(crate) use enumerated::parse_enumerated_list;
pub(crate) use field::parse_field_list;
pub(crate) use option::parse_option_list;

use super::block::parse_block;
use crate::parser_errors::ParserError;
use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{NodeRef, NodeRefExt};

fn parse_item_body(stream: &mut TokenStream, dedent_len: usize) -> Result<NodeRef, ParserError> {
    let mut dedent_len = dedent_len;

    if stream.token_at_cursor().kind == TK::Spaces {
        dedent_len += stream.consume().len();
    }

    let next_line = stream.find_end_of_line() + 1;
    let indent_ahead_index = match stream.token_at(next_line).kind {
        TK::Indent => Some(next_line),
        TK::BlankLine if stream.token_at(next_line + 1).kind == TK::Indent => Some(next_line + 1),
        _ => None,
    };

    match indent_ahead_index {
        Some(indent_index) => {
            let indent_token = stream.take_at(indent_index);
            if indent_token.len() <= dedent_len {
                stream.insert_before_cursor(indent_token);
            } else {
                stream.insert_before_cursor(Token::indent(dedent_len));
                stream.insert_at(
                    indent_index + 1,
                    Token::indent(indent_token.len() - dedent_len),
                );
            }
        }
        None => match stream.token_at(next_line).kind {
            TK::Field
            | TK::BulletListMarker
            | TK::EnumeratedListMarker
            | TK::OptionGroup
            | TK::EoF
            | TK::Dedent
            | TK::BlankLine => {
                stream.insert_before_cursor(Token::indent(dedent_len));
                stream.insert_at(next_line + 1, Token::dedent(dedent_len));
            }
            _ => return Err(ParserError::ListEndError {}),
        },
    }
    parse_block(stream)
}

fn consume_trailing_blank_lines(stream: &mut TokenStream, list: &NodeRef) {
    if stream.token_at_cursor().kind == TK::BlankLine {
        let blank_token = stream.consume();
        list.push_blank_lines(blank_token.len());
    }
}
