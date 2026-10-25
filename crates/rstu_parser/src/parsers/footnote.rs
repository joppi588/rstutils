// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::block::parse_block;
use super::paragraph::parse_paragraph_until;
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

    while stream.token_at_cursor().kind == TK::Spaces
        && stream
            .token_at_cursor()
            .lexeme
            .chars()
            .all(char::is_whitespace)
    {
        stream.consume();
    }

    match stream.token_at_cursor().kind {
        TK::NewLine => {
            stream.consume_newline();
            if stream.token_at_cursor().kind != TK::Indent {
                return Err(ParserError::UnexpectedToken {
                    expected: "Footnote body".to_owned(),
                    found: format!("{:?}", stream.token_at_cursor().kind),
                    position: stream.position_at_cursor(),
                });
            }
            append_block_children(stream, &footnote)?;
        }
        TK::BlankLine | TK::Dedent | TK::EoF => {
            return Err(ParserError::UnexpectedToken {
                expected: "Footnote body".to_owned(),
                found: format!("{:?}", stream.token_at_cursor().kind),
                position: stream.position_at_cursor(),
            });
        }
        _ => {
            footnote.push_child(parse_paragraph_until(stream, &[TK::NewLine])?);
            if stream.token_at_cursor().kind == TK::NewLine {
                stream.consume_newline();
            }
            match stream.token_at_cursor().kind {
                TK::Indent => append_block_children(stream, &footnote)?,
                TK::BlankLine | TK::FootNote | TK::EoF => {}
                _ => {
                    return Err(ParserError::MissingFinalBlankLineError {
                        entity: "Explicit markup".to_owned(),
                        position: stream.position_at_cursor(),
                    });
                }
            }
        }
    }

    if !matches!(
        stream.token_at_cursor().kind,
        TK::BlankLine | TK::Dedent | TK::EoF | TK::FootNote
    ) {
        return Err(ParserError::MissingFinalBlankLineError {
            entity: "Explicit markup".to_owned(),
            position: stream.position_at_cursor(),
        });
    }

    Ok(footnote)
}

fn append_block_children(stream: &mut TokenStream, footnote: &NodeRef) -> Result<(), ParserError> {
    let block = parse_block(stream)?;
    let children = block.borrow().children.clone();
    for child in children {
        footnote.push_child(child);
    }
    Ok(())
}
