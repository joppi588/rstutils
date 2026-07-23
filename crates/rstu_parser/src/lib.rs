// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;

use rstu_ast::{ElementKind, Node};

use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindElementError {
    StartAtOutOfBounds {
        start_at: usize,
        token_count: usize,
    },
    SectionTitleMissingClosingAfterOpening {
        opening_index: usize,
    },
    SectionTitleUnbalancedStyle {
        opening_index: usize,
        opening_style: String,
        closing_style: String,
    },
}

pub fn parse(input: &str) -> Node {
    let _tokens = tokenize(input);
    let doc: Node = Node::new(ElementKind::Document);

    doc
}

pub fn try_find_section_header(
    tokens: &[Token],
    start_at: usize,
) -> Result<Option<(usize, usize)>, FindElementError> {
    for index in start_at..tokens.len() {
        match tokens[index].kind {
            TokenKind::SectionTitlePrefix => {
                let next_line_end = find_next_newline(tokens, index + 2).ok_or(
                    FindElementError::SectionTitleMissingClosingAfterOpening {
                        opening_index: index,
                    },
                )?;

                if next_line_end + 1 >= tokens.len() {
                    return Err(FindElementError::SectionTitleMissingClosingAfterOpening {
                        opening_index: index,
                    });
                }

                let closing_index = next_line_end + 1;
                if tokens[closing_index].kind != TokenKind::SectionTitleSuffix {
                    return Err(FindElementError::SectionTitleMissingClosingAfterOpening {
                        opening_index: index,
                    });
                }

                if tokens[index].lexeme != tokens[closing_index].lexeme {
                    return Err(FindElementError::SectionTitleUnbalancedStyle {
                        opening_index: index,
                        opening_style: tokens[index].lexeme.clone(),
                        closing_style: tokens[closing_index].lexeme.clone(),
                    });
                }

                return Ok(Some((index + 2, next_line_end + 1)));
            }
            TokenKind::SectionTitleSuffix => {
                let previous_line_start = move_back_one_line(tokens, index).unwrap_or(0);
                return Ok(Some((previous_line_start, index)));
            }
            _ => {}
        }
    }

    Ok(None)
}

fn find_next_newline(tokens: &[Token], start_at: usize) -> Option<usize> {
    tokens
        .iter()
        .enumerate()
        .skip(start_at)
        .find_map(|(index, token)| (token.kind == TokenKind::NewLine).then_some(index))
}

fn move_back_one_line(tokens: &[Token], index: usize) -> Option<usize> {
    // Move to the first token of the line ending before index
    let mut cursor = index.checked_sub(2)?;
    while !matches!(
        tokens[cursor].kind,
        TokenKind::NewLine | TokenKind::BlankLine
    ) {
        cursor = cursor.checked_sub(1)?;
    }
    Some(cursor + 1)
}
